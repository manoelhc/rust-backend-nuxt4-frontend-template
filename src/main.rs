use axum::{
    extract::{Path, Query, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tower_http::cors::CorsLayer;
use tracing::info;
use uuid::Uuid;

// Environment constants
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    sub: String,
    exp: usize,
    email_verified: Option<bool>,
    mfa_enabled: Option<bool>,
    email: Option<String>,
    name: Option<String>,
    admin: Option<bool>,
}

#[derive(Clone)]
struct AppState {
    jwt_secret: String,
    start_time: SystemTime,
    db_pool: PgPool,
}

// Response structures
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

#[derive(Serialize)]
struct VersionResponse {
    version: String,
}

#[derive(Deserialize)]
struct ValidateTokenRequest {
    token: String,
}

#[derive(Serialize)]
struct ValidateTokenResponse {
    valid: bool,
    message: String,
}

#[derive(Serialize)]
struct UptimeResponse {
    uptime_seconds: u64,
    uptime_formatted: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: Uuid,
    sub: String,
    user_email: String,
    user_fullname: String,
    organization: Option<String>,
    group_id: Option<Uuid>,
    properties: JsonValue,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
struct Group {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
struct Role {
    id: Uuid,
    name: String,
    description: Option<String>,
    is_admin: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
struct Permission {
    id: Uuid,
    role_id: Uuid,
    page: String,
    can_view: bool,
    can_edit: bool,
    can_view_own: bool,
    can_edit_own: bool,
    can_view_ours: bool,
    can_edit_ours: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct UserRole {
    id: Uuid,
    user_id: Uuid,
    role_id: Uuid,
    assigned_at: DateTime<Utc>,
    assigned_by: Option<Uuid>,
}

#[derive(Serialize)]
struct OnboardingResponse {
    user_id: Uuid,
    message: String,
    is_new_user: bool,
}

#[derive(Serialize)]
struct ProfileResponse {
    user: User,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Admin API structures
#[derive(Serialize)]
struct RoleWithPermissions {
    role: Role,
    permissions: Vec<Permission>,
}

#[derive(Serialize)]
struct UserWithRoles {
    user: User,
    roles: Vec<Role>,
}

#[derive(Deserialize)]
struct CreateRoleRequest {
    name: String,
    description: Option<String>,
    is_admin: bool,
}

#[derive(Deserialize)]
struct UpdateRoleRequest {
    name: Option<String>,
    description: Option<String>,
    is_admin: Option<bool>,
}

#[derive(Deserialize)]
struct SetPermissionRequest {
    page: String,
    can_view: bool,
    can_edit: bool,
    can_view_own: bool,
    can_edit_own: bool,
    can_view_ours: bool,
    can_edit_ours: bool,
}

#[derive(Deserialize)]
struct AssignRoleRequest {
    role_id: Uuid,
}

#[derive(Deserialize)]
struct PaginationQuery {
    page: Option<u32>,
    per_page: Option<u32>,
}

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

    let sqls = vec![
        include_str!("../migrations/001_create_users_table.sql")
            .split(';')
            .collect::<Vec<&str>>(),
        include_str!("../migrations/002_create_roles_and_permissions.sql")
            .split(';')
            .collect::<Vec<&str>>(),
    ];

    // Run migrations
    for sql in sqls.into_iter().flatten() {
        let trimmed_sql = sql.trim().to_string() + ";";
        if !trimmed_sql.is_empty() {
            sqlx::query(trimmed_sql.as_str())
                .execute(&db_pool)
                .await
                .expect("Failed to run migrations");
        }
    }

    info!("Database connected and migrations applied");

    let state = Arc::new(AppState {
        jwt_secret,
        start_time: SystemTime::now(),
        db_pool,
    });

    // Build protected routes
    let protected_routes = Router::new()
        .route("/system/uptime", get(system_uptime))
        .route("/system/onboarding", post(system_onboarding))
        .route("/profile", get(get_profile))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // Build admin-only routes
    let admin_routes = Router::new()
        .route("/admin/roles", get(list_roles).post(create_role))
        .route("/admin/roles/:role_id", get(get_role).post(update_role))
        .route("/admin/roles/:role_id/delete", post(delete_role))
        .route("/admin/roles/:role_id/permissions", post(set_role_permission))
        .route("/admin/users", get(list_users))
        .route("/admin/users/:user_id/roles", get(get_user_roles).post(assign_user_role))
        .route("/admin/users/:user_id/roles/remove", post(remove_user_role))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            admin_middleware,
        ));

    // Build public routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/system/version", get(system_version))
        .route("/validate-token", post(validate_token))
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

async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Service is healthy".to_string(),
    })
}

async fn system_version() -> impl IntoResponse {
    Json(VersionResponse {
        version: VERSION.to_string(),
    })
}

async fn validate_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ValidateTokenRequest>,
) -> impl IntoResponse {
    match validate_jwt_token_with_claims(&payload.token, &state.jwt_secret) {
        Ok(claims) => {
            // Check email_verified and mfa_enabled
            let email_verified = claims.email_verified.unwrap_or(false);
            let mfa_enabled = claims.mfa_enabled.unwrap_or(false);

            if !email_verified {
                return Json(ValidateTokenResponse {
                    valid: false,
                    message: "Email not verified".to_string(),
                });
            }

            if !mfa_enabled {
                return Json(ValidateTokenResponse {
                    valid: false,
                    message: "MFA not enabled".to_string(),
                });
            }

            Json(ValidateTokenResponse {
                valid: true,
                message: "Token is valid".to_string(),
            })
        }
        Err(e) => Json(ValidateTokenResponse {
            valid: false,
            message: format!("Token is invalid: {}", e),
        }),
    }
}

fn validate_jwt_token_with_claims(token: &str, secret: &str) -> Result<Claims, String> {
    let validation = Validation::default();
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|e| format!("{}", e))
}

async fn system_uptime(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let uptime = match SystemTime::now().duration_since(state.start_time) {
        Ok(duration) => duration,
        Err(_) => {
            tracing::warn!("System clock appears to have gone backwards");
            Duration::from_secs(0)
        }
    };

    let seconds = uptime.as_secs();
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    let formatted = if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, secs)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    };

    Json(UptimeResponse {
        uptime_seconds: seconds,
        uptime_formatted: formatted,
    })
}

async fn system_onboarding(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<Json<OnboardingResponse>, (StatusCode, Json<ErrorResponse>)> {
    let sub = &claims.sub;

    // Check if user exists
    let existing_user: Option<User> =
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE sub = $1")
            .bind(sub)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Database error".to_string(),
                    }),
                )
            })?;

    if let Some(user) = existing_user {
        return Ok(Json(OnboardingResponse {
            user_id: user.id,
            message: "User already registered".to_string(),
            is_new_user: false,
        }));
    }

    // Extract user information from claims
    let user_email = claims
        .email
        .clone()
        .unwrap_or_else(|| format!("{}@example.com", sub));
    let user_fullname = claims
        .name
        .clone()
        .unwrap_or_else(|| "Unknown User".to_string());

    // Create new user
    let new_user: User = sqlx::query_as::<_, User>(
        "INSERT INTO users (sub, user_email, user_fullname, organization, properties) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING *",
    )
    .bind(sub)
    .bind(&user_email)
    .bind(&user_fullname)
    .bind::<Option<String>>(None)
    .bind(serde_json::json!({}))
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create user: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create user".to_string(),
            }),
        )
    })?;

    Ok(Json(OnboardingResponse {
        user_id: new_user.id,
        message: "User registered successfully".to_string(),
        is_new_user: true,
    }))
}

async fn get_profile(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<Json<ProfileResponse>, (StatusCode, Json<ErrorResponse>)> {
    let sub = &claims.sub;

    let user: Option<User> = sqlx::query_as::<_, User>("SELECT * FROM users WHERE sub = $1")
        .bind(sub)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Database error".to_string(),
                }),
            )
        })?;

    match user {
        Some(user) => Ok(Json(ProfileResponse { user })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "User not found".to_string(),
            }),
        )),
    }
}

async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            match validate_jwt_token_with_claims(token, &state.jwt_secret) {
                Ok(claims) => {
                    // Check email_verified and mfa_enabled
                    if !claims.email_verified.unwrap_or(false) {
                        return Err(StatusCode::FORBIDDEN);
                    }
                    if !claims.mfa_enabled.unwrap_or(false) {
                        return Err(StatusCode::FORBIDDEN);
                    }

                    // Insert claims into request extensions
                    req.extensions_mut().insert(claims);
                    return Ok(next.run(req).await);
                }
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

async fn admin_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            match validate_jwt_token_with_claims(token, &state.jwt_secret) {
                Ok(claims) => {
                    // Check email_verified and mfa_enabled
                    if !claims.email_verified.unwrap_or(false) {
                        return Err(StatusCode::FORBIDDEN);
                    }
                    if !claims.mfa_enabled.unwrap_or(false) {
                        return Err(StatusCode::FORBIDDEN);
                    }

                    // Check if user is admin
                    if !claims.admin.unwrap_or(false) {
                        return Err(StatusCode::FORBIDDEN);
                    }

                    // Insert claims into request extensions
                    req.extensions_mut().insert(claims);
                    return Ok(next.run(req).await);
                }
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

// Extractor for Claims from request extensions
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))
    }
}

// ==================== Admin Endpoints ====================

// List all roles with their permissions
async fn list_roles(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RoleWithPermissions>>, (StatusCode, Json<ErrorResponse>)> {
    let roles: Vec<Role> = sqlx::query_as::<_, Role>("SELECT * FROM roles ORDER BY name")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Database error".to_string(),
                }),
            )
        })?;

    let mut result = Vec::new();
    for role in roles {
        let permissions: Vec<Permission> =
            sqlx::query_as::<_, Permission>("SELECT * FROM permissions WHERE role_id = $1")
                .bind(role.id)
                .fetch_all(&state.db_pool)
                .await
                .map_err(|e| {
                    tracing::error!("Database error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "Database error".to_string(),
                        }),
                    )
                })?;

        result.push(RoleWithPermissions { role, permissions });
    }

    Ok(Json(result))
}

// Create a new role
async fn create_role(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Json<Role>, (StatusCode, Json<ErrorResponse>)> {
    let role: Role = sqlx::query_as::<_, Role>(
        "INSERT INTO roles (name, description, is_admin) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.is_admin)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create role: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create role".to_string(),
            }),
        )
    })?;

    Ok(Json(role))
}

// Get a specific role with permissions
async fn get_role(
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<Uuid>,
) -> Result<Json<RoleWithPermissions>, (StatusCode, Json<ErrorResponse>)> {
    let role: Option<Role> = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE id = $1")
        .bind(role_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Database error".to_string(),
                }),
            )
        })?;

    let role = role.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Role not found".to_string(),
            }),
        )
    })?;

    let permissions: Vec<Permission> =
        sqlx::query_as::<_, Permission>("SELECT * FROM permissions WHERE role_id = $1")
            .bind(role_id)
            .fetch_all(&state.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Database error".to_string(),
                    }),
                )
            })?;

    Ok(Json(RoleWithPermissions { role, permissions }))
}

// Update a role
async fn update_role(
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<Uuid>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<Role>, (StatusCode, Json<ErrorResponse>)> {
    // Build dynamic update query
    let mut query = "UPDATE roles SET updated_at = NOW()".to_string();
    let mut has_updates = false;

    if payload.name.is_some() {
        query.push_str(", name = $2");
        has_updates = true;
    }
    if payload.description.is_some() {
        query.push_str(if has_updates {
            ", description = $3"
        } else {
            ", description = $2"
        });
        has_updates = true;
    }
    if payload.is_admin.is_some() {
        let idx = if payload.name.is_some() && payload.description.is_some() {
            4
        } else if payload.name.is_some() || payload.description.is_some() {
            3
        } else {
            2
        };
        query.push_str(&format!(", is_admin = ${}", idx));
        has_updates = true;
    }

    query.push_str(" WHERE id = $1 RETURNING *");

    if !has_updates {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "No fields to update".to_string(),
            }),
        ));
    }

    let mut q = sqlx::query_as::<_, Role>(&query).bind(role_id);

    if let Some(name) = payload.name {
        q = q.bind(name);
    }
    if let Some(description) = payload.description {
        q = q.bind(description);
    }
    if let Some(is_admin) = payload.is_admin {
        q = q.bind(is_admin);
    }

    let role: Role = q.fetch_one(&state.db_pool).await.map_err(|e| {
        tracing::error!("Failed to update role: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to update role".to_string(),
            }),
        )
    })?;

    Ok(Json(role))
}

// Delete a role
async fn delete_role(
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let result = sqlx::query("DELETE FROM roles WHERE id = $1")
        .bind(role_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete role: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to delete role".to_string(),
                }),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Role not found".to_string(),
            }),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

// Set or update permission for a role on a specific page
async fn set_role_permission(
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<Uuid>,
    Json(payload): Json<SetPermissionRequest>,
) -> Result<Json<Permission>, (StatusCode, Json<ErrorResponse>)> {
    let permission: Permission = sqlx::query_as::<_, Permission>(
        "INSERT INTO permissions (role_id, page, can_view, can_edit, can_view_own, can_edit_own, can_view_ours, can_edit_ours)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (role_id, page) 
         DO UPDATE SET 
            can_view = $3, 
            can_edit = $4, 
            can_view_own = $5, 
            can_edit_own = $6, 
            can_view_ours = $7, 
            can_edit_ours = $8,
            updated_at = NOW()
         RETURNING *"
    )
    .bind(role_id)
    .bind(&payload.page)
    .bind(payload.can_view)
    .bind(payload.can_edit)
    .bind(payload.can_view_own)
    .bind(payload.can_edit_own)
    .bind(payload.can_view_ours)
    .bind(payload.can_edit_ours)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to set permission: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to set permission".to_string(),
            }),
        )
    })?;

    Ok(Json(permission))
}

// List all users with pagination
async fn list_users(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<Vec<UserWithRoles>>, (StatusCode, Json<ErrorResponse>)> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(50).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let users: Vec<User> =
        sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY user_fullname LIMIT $1 OFFSET $2")
            .bind(per_page as i64)
            .bind(offset as i64)
            .fetch_all(&state.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Database error".to_string(),
                    }),
                )
            })?;

    let mut result = Vec::new();
    for user in users {
        let roles: Vec<Role> = sqlx::query_as::<_, Role>(
            "SELECT r.* FROM roles r 
             INNER JOIN user_roles ur ON r.id = ur.role_id 
             WHERE ur.user_id = $1",
        )
        .bind(user.id)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Database error".to_string(),
                }),
            )
        })?;

        result.push(UserWithRoles { user, roles });
    }

    Ok(Json(result))
}

// Get roles for a specific user
async fn get_user_roles(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<Role>>, (StatusCode, Json<ErrorResponse>)> {
    let roles: Vec<Role> = sqlx::query_as::<_, Role>(
        "SELECT r.* FROM roles r 
         INNER JOIN user_roles ur ON r.id = ur.role_id 
         WHERE ur.user_id = $1",
    )
    .bind(user_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Database error".to_string(),
            }),
        )
    })?;

    Ok(Json(roles))
}

// Assign a role to a user
async fn assign_user_role(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    claims: Claims,
    Json(payload): Json<AssignRoleRequest>,
) -> Result<Json<UserRole>, (StatusCode, Json<ErrorResponse>)> {
    // Get current user's ID
    let admin_user: Option<User> = sqlx::query_as::<_, User>("SELECT * FROM users WHERE sub = $1")
        .bind(&claims.sub)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Database error".to_string(),
                }),
            )
        })?;

    let assigned_by_id = admin_user.map(|u| u.id);

    let user_role: UserRole = sqlx::query_as::<_, UserRole>(
        "INSERT INTO user_roles (user_id, role_id, assigned_by) 
         VALUES ($1, $2, $3) 
         ON CONFLICT (user_id, role_id) DO NOTHING
         RETURNING *",
    )
    .bind(user_id)
    .bind(payload.role_id)
    .bind(assigned_by_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to assign role: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to assign role".to_string(),
            }),
        )
    })?;

    Ok(Json(user_role))
}

// Remove a role from a user
async fn remove_user_role(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<AssignRoleRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let result = sqlx::query("DELETE FROM user_roles WHERE user_id = $1 AND role_id = $2")
        .bind(user_id)
        .bind(payload.role_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to remove role: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to remove role".to_string(),
                }),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "User role not found".to_string(),
            }),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
