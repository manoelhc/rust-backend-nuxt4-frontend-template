use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tower_http::cors::CorsLayer;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Clone)]
struct AppState {
    jwt_secret: String,
    start_time: SystemTime,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
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

    let state = Arc::new(AppState {
        jwt_secret,
        start_time: SystemTime::now(),
    });

    // Build our application with routes
    let protected_routes = Router::new()
        .route("/system/uptime", get(system_uptime))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // Note: CorsLayer::permissive() is used for development convenience.
    // In production, restrict CORS to specific origins using CorsLayer::new()
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/validate-token", post(validate_token))
        .merge(protected_routes)
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect(&format!("Failed to bind to {}. Check if the port is already in use or if you have permission to bind to it.", addr));

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

async fn validate_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ValidateTokenRequest>,
) -> impl IntoResponse {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        &payload.token,
        &DecodingKey::from_secret(state.jwt_secret.as_ref()),
        &validation,
    );

    match token_data {
        Ok(_) => Json(ValidateTokenResponse {
            valid: true,
            message: "Token is valid".to_string(),
        }),
        Err(_) => Json(ValidateTokenResponse {
            valid: false,
            message: "Token is invalid or expired".to_string(),
        }),
    }
}

async fn system_uptime(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let uptime = SystemTime::now()
        .duration_since(state.start_time)
        .unwrap_or(Duration::from_secs(0));

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

async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Check for Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            let validation = Validation::default();
            if decode::<Claims>(
                token,
                &DecodingKey::from_secret(state.jwt_secret.as_ref()),
                &validation,
            )
            .is_ok()
            {
                return Ok(next.run(req).await);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
