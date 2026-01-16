use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tower_http::cors::CorsLayer;
use tracing::info;
use uuid::Uuid;
use chrono::{DateTime, Utc};

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
    properties: JsonValue,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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

    // Run migrations
    sqlx::query(include_str!("../migrations/001_create_users_table.sql"))
        .execute(&db_pool)
        .await
        .expect("Failed to run migrations");

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

    // Build public routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/system/version", get(system_version))
        .route("/validate-token", post(validate_token))
        .merge(protected_routes)
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
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)
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
    let existing_user: Option<User> = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE sub = $1"
    )
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
    let user_email = claims.email.clone().unwrap_or_else(|| format!("{}@example.com", sub));
    let user_fullname = claims.name.clone().unwrap_or_else(|| "Unknown User".to_string());
    
    // Create new user
    let new_user: User = sqlx::query_as::<_, User>(
        "INSERT INTO users (sub, user_email, user_fullname, organization, properties) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING *"
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
    
    let user: Option<User> = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE sub = $1"
    )
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
