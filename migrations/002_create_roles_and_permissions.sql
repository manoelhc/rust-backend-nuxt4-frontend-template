-- Create groups table for "ours" permissions
CREATE TABLE IF NOT EXISTS groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add group_id to users table
ALTER TABLE users ADD COLUMN IF NOT EXISTS group_id UUID REFERENCES groups(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_users_group_id ON users(group_id);

-- Create roles table
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_roles_name ON roles(name);

-- Create permissions table (permission matrix per role per page)
CREATE TABLE IF NOT EXISTS permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    page VARCHAR(255) NOT NULL,
    can_view BOOLEAN NOT NULL DEFAULT FALSE,
    can_edit BOOLEAN NOT NULL DEFAULT FALSE,
    can_view_own BOOLEAN NOT NULL DEFAULT FALSE,
    can_edit_own BOOLEAN NOT NULL DEFAULT FALSE,
    can_view_ours BOOLEAN NOT NULL DEFAULT FALSE,
    can_edit_ours BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(role_id, page)
);

CREATE INDEX IF NOT EXISTS idx_permissions_role_id ON permissions(role_id);
CREATE INDEX IF NOT EXISTS idx_permissions_page ON permissions(page);

-- Create user_roles junction table (users can have multiple roles)
CREATE TABLE IF NOT EXISTS user_roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    assigned_by UUID REFERENCES users(id) ON DELETE SET NULL,
    UNIQUE(user_id, role_id)
);

CREATE INDEX IF NOT EXISTS idx_user_roles_user_id ON user_roles(user_id);
CREATE INDEX IF NOT EXISTS idx_user_roles_role_id ON user_roles(role_id);

-- Insert default roles
INSERT INTO roles (name, description, is_admin) 
VALUES 
    ('Admin', 'Full system administrator with all permissions', TRUE),
    ('View', 'View-only access to assigned pages', FALSE)
ON CONFLICT (name) DO NOTHING;

-- Get the Admin role ID for default permissions
DO $$
DECLARE
    admin_role_id UUID;
BEGIN
    SELECT id INTO admin_role_id FROM roles WHERE name = 'Admin';
    
    -- Admin has full permissions on all default pages
    INSERT INTO permissions (role_id, page, can_view, can_edit, can_view_own, can_edit_own, can_view_ours, can_edit_ours)
    VALUES 
        (admin_role_id, 'dashboard', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE),
        (admin_role_id, 'users', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE),
        (admin_role_id, 'roles', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE),
        (admin_role_id, 'profile', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE),
        (admin_role_id, 'preferences', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE),
        (admin_role_id, 'support', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE)
    ON CONFLICT (role_id, page) DO NOTHING;
END $$;
