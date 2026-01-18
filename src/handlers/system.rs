use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

use crate::middleware::validate_jwt_token_with_claims;
use crate::models::{
    AppSetting, AppState, Claims, ErrorResponse, HealthResponse, LogoResponse, LogoUploadRequest,
    OnboardingResponse, ProfileResponse, UptimeResponse, User, ValidateTokenRequest,
    ValidateTokenResponse, VersionResponse,
};

// Environment constants
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Service is healthy".to_string(),
    })
}

/// Get application version
pub async fn system_version() -> impl IntoResponse {
    Json(VersionResponse {
        version: VERSION.to_string(),
    })
}

/// Validate a JWT token
pub async fn validate_token(
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

/// Get system uptime
pub async fn system_uptime(State(state): State<Arc<AppState>>) -> impl IntoResponse {
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

/// User onboarding - auto-register user from JWT claims
pub async fn system_onboarding(
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
    let organization = claims.organization.clone();

    // Try to find organization_id if organization name is provided
    let organization_id: Option<Uuid> = if let Some(org_name) = &organization {
        sqlx::query_scalar("SELECT id FROM organizations WHERE name = $1")
            .bind(org_name)
            .fetch_optional(&state.db_pool)
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    // Create new user
    let new_user: User = sqlx::query_as::<_, User>(
        "INSERT INTO users (sub, user_email, user_fullname, organization, organization_id, properties) 
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING *",
    )
    .bind(sub)
    .bind(&user_email)
    .bind(&user_fullname)
    .bind(&organization)
    .bind(organization_id)
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

/// Get user profile
pub async fn get_profile(
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

/// Update application logo (organization-scoped)
pub async fn update_logo(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    axum::Json(payload): axum::Json<LogoUploadRequest>,
) -> Result<Json<LogoResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate input
    if payload.logo_url.is_empty() || payload.alt_text.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Logo URL and alt text are required".to_string(),
            }),
        ));
    }

    // Extract organization_id from claims
    let organization_id: Option<Uuid> = if let Some(org_name) = &claims.organization {
        sqlx::query_scalar("SELECT id FROM organizations WHERE name = $1")
            .bind(org_name)
            .fetch_optional(&state.db_pool)
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    let metadata = serde_json::json!({
        "alt_text": payload.alt_text.clone(),
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    // Upsert logo setting with organization_id
    let _result: AppSetting = sqlx::query_as::<_, AppSetting>(
        "INSERT INTO app_settings (organization_id, setting_key, setting_value, metadata)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (organization_id, setting_key) DO UPDATE SET
         setting_value = $3,
         metadata = $4,
         updated_at = NOW()
         RETURNING *",
    )
    .bind(organization_id)
    .bind("navbar_logo_url")
    .bind(&payload.logo_url)
    .bind(metadata)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to save logo: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to save logo".to_string(),
            }),
        )
    })?;

    tracing::info!(
        "Logo updated successfully for organization: {:?}",
        organization_id
    );

    Ok(Json(LogoResponse {
        logo_url: payload.logo_url,
        alt_text: payload.alt_text,
    }))
}

/// Get application logo (organization-scoped)
pub async fn get_logo(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<Json<LogoResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Extract organization_id from claims
    let organization_id: Option<Uuid> = if let Some(org_name) = &claims.organization {
        sqlx::query_scalar("SELECT id FROM organizations WHERE name = $1")
            .bind(org_name)
            .fetch_optional(&state.db_pool)
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    // Fetch logo for the current organization
    let logo_setting: Option<AppSetting> = sqlx::query_as::<_, AppSetting>(
        "SELECT * FROM app_settings WHERE organization_id = $1 AND setting_key = $2",
    )
    .bind(organization_id)
    .bind("navbar_logo_url")
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

    match logo_setting {
        Some(setting) => {
            let alt_text = setting
                .metadata
                .get("alt_text")
                .and_then(|v| v.as_str())
                .unwrap_or("Application Logo")
                .to_string();

            Ok(Json(LogoResponse {
                logo_url: setting.setting_value.unwrap_or_default(),
                alt_text,
            }))
        }
        None => Ok(Json(LogoResponse {
            logo_url: "".to_string(),
            alt_text: "Application Logo".to_string(),
        })),
    }
}
