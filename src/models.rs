use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub email_verified: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub admin: Option<bool>,
}

// Application state
#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
    pub start_time: std::time::SystemTime,
    pub db_pool: sqlx::PgPool,
}

// Response structures
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct VersionResponse {
    pub version: String,
}

#[derive(Deserialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct ValidateTokenResponse {
    pub valid: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct UptimeResponse {
    pub uptime_seconds: u64,
    pub uptime_formatted: String,
}

// Database models
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub sub: String,
    pub user_email: String,
    pub user_fullname: String,
    pub organization: Option<String>,
    pub group_id: Option<Uuid>,
    pub properties: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Permission {
    pub id: Uuid,
    pub role_id: Uuid,
    pub page: String,
    pub can_view: bool,
    pub can_edit: bool,
    pub can_view_own: bool,
    pub can_edit_own: bool,
    pub can_view_ours: bool,
    pub can_edit_ours: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRole {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_at: DateTime<Utc>,
    pub assigned_by: Option<Uuid>,
}

#[derive(Serialize)]
pub struct OnboardingResponse {
    pub user_id: Uuid,
    pub message: String,
    pub is_new_user: bool,
}

#[derive(Serialize)]
pub struct ProfileResponse {
    pub user: User,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// Admin API structures
#[derive(Serialize)]
pub struct RoleWithPermissions {
    pub role: Role,
    pub permissions: Vec<Permission>,
}

#[derive(Serialize)]
pub struct UserWithRoles {
    pub user: User,
    pub roles: Vec<Role>,
}

#[derive(Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_admin: bool,
}

#[derive(Deserialize)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_admin: Option<bool>,
}

#[derive(Deserialize)]
pub struct SetPermissionRequest {
    pub page: String,
    pub can_view: bool,
    pub can_edit: bool,
    pub can_view_own: bool,
    pub can_edit_own: bool,
    pub can_view_ours: bool,
    pub can_edit_ours: bool,
}

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_id: Uuid,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
