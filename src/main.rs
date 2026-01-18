mod handlers;
mod middleware;
mod migrations;
mod models;

use axum::{
    middleware as axum_middleware,
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

use handlers::{admin, system};
use middleware::admin_middleware;
use migrations::parse_sql_statements;
use models::AppState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("JWT_SECRET not set, using default (NOT SECURE FOR PRODUCTION)");
        "my-secret-key".to_string()
    });

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/app_db".to_string());

    // Create database pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let migration_files = vec![
        include_str!("../migrations/001_create_users_table.sql"),
        include_str!("../migrations/002_create_app_settings_table.sql"),
        include_str!("../migrations/002_create_roles_and_permissions.sql"),
        include_str!("../migrations/004_add_organization_to_app_settings.sql"),
    ];

    // Run migrations
    for migration_content in migration_files {
        let statements = parse_sql_statements(migration_content);
        for statement in statements {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed)
                    .execute(&db_pool)
                    .await
                    .expect("Failed to run migrations");
            }
        }
    }

    info!("Database connected and migrations applied");

    let state = Arc::new(AppState {
        jwt_secret,
        start_time: std::time::SystemTime::now(),
        db_pool,
    });

    // Build protected routes
    let protected_routes = Router::new()
        .route("/system/uptime", get(system::system_uptime))
        .route("/system/onboarding", post(system::system_onboarding))
        .route("/profile", get(system::get_profile))
        .route("/admin/logo", post(system::update_logo))
        .route_layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::auth_middleware,
        ));

    // Build admin-only routes
    let admin_routes = Router::new()
        .route(
            "/admin/roles",
            get(admin::list_roles).post(admin::create_role),
        )
        .route(
            "/admin/roles/:role_id",
            get(admin::get_role).post(admin::update_role),
        )
        .route("/admin/roles/:role_id/delete", post(admin::delete_role))
        .route(
            "/admin/roles/:role_id/permissions",
            post(admin::set_role_permission),
        )
        .route("/admin/users", get(admin::list_users))
        .route(
            "/admin/users/:user_id/roles",
            get(admin::get_user_roles).post(admin::assign_user_role),
        )
        .route(
            "/admin/users/:user_id/roles/remove",
            post(admin::remove_user_role),
        )
        .route_layer(axum_middleware::from_fn_with_state(
            state.clone(),
            admin_middleware,
        ));

    // Build public routes
    let app = Router::new()
        .route("/health", get(system::health_check))
        .route("/system/version", get(system::system_version))
        .route("/validate-token", post(system::validate_token))
        .route("/admin/logo", get(system::get_logo))
        .merge(protected_routes)
        .merge(admin_routes)
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to bind to {}: {}. Check if the port is already in use or if you have permission to bind to it.", addr, e);
        });

    info!("Server starting on http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
