use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;

use crate::models::{AppState, Claims};

/// Validate JWT token and extract claims
pub fn validate_jwt_token_with_claims(token: &str, secret: &str) -> Result<Claims, String> {
    let validation = Validation::default();
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|e| format!("{}", e))
}

/// Authentication middleware for protected routes
/// Validates JWT token and checks email_verified and mfa_enabled
pub async fn auth_middleware(
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

/// Admin-only middleware
/// Same as auth_middleware but also checks for admin claim
pub async fn admin_middleware(
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
