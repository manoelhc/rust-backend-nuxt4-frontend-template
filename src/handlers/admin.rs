use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{
    AppState, AssignRoleRequest, Claims, CreateRoleRequest, ErrorResponse, PaginationQuery,
    Permission, Role, RoleWithPermissions, SetPermissionRequest, UpdateRoleRequest, User,
    UserRole, UserWithRoles,
};

// ==================== Role Management ====================

/// List all roles with their permissions
pub async fn list_roles(
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

/// Create a new role
pub async fn create_role(
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

/// Get a specific role with permissions
pub async fn get_role(
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

/// Update a role
pub async fn update_role(
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<Uuid>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<Role>, (StatusCode, Json<ErrorResponse>)> {
    // Validate that at least one field is provided
    if payload.name.is_none() && payload.description.is_none() && payload.is_admin.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "No fields to update".to_string(),
            }),
        ));
    }

    // Get current role to use as defaults for unspecified fields
    let current_role: Option<Role> = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE id = $1")
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

    let current_role = current_role.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Role not found".to_string(),
            }),
        )
    })?;

    // Use provided values or fall back to current values
    let name = payload.name.unwrap_or(current_role.name);
    let description = payload.description.or(current_role.description);
    let is_admin = payload.is_admin.unwrap_or(current_role.is_admin);

    // Update with all fields
    let role: Role = sqlx::query_as::<_, Role>(
        "UPDATE roles SET name = $2, description = $3, is_admin = $4, updated_at = NOW() 
         WHERE id = $1 RETURNING *"
    )
    .bind(role_id)
    .bind(name)
    .bind(description)
    .bind(is_admin)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
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

/// Delete a role
pub async fn delete_role(
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

/// Set or update permission for a role on a specific page
pub async fn set_role_permission(
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

// ==================== User Management ====================

/// List all users with pagination
pub async fn list_users(
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

/// Get roles for a specific user
pub async fn get_user_roles(
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

/// Assign a role to a user
pub async fn assign_user_role(
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

/// Remove a role from a user
pub async fn remove_user_role(
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
