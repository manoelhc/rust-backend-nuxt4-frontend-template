-- Initial database schema with multi-tenancy support
-- This migration creates all tables with organization isolation from the start

-- Create organizations table first (required for foreign keys)
CREATE TABLE IF NOT EXISTS organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_organizations_name ON organizations(name);

-- Insert a default system organization for backward compatibility
INSERT INTO organizations (id, name, description)
VALUES ('00000000-0000-0000-0000-000000000000'::uuid, 'System', 'System-wide organization for global resources')
ON CONFLICT (id) DO NOTHING;

-- Create groups table for "ours" permissions
CREATE TABLE IF NOT EXISTS groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_groups_organization_id ON groups(organization_id);

-- Create users table with organization support
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sub VARCHAR(255) UNIQUE NOT NULL,
    user_email VARCHAR(255) NOT NULL,
    user_fullname VARCHAR(255) NOT NULL,
    organization VARCHAR(255),
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    group_id UUID REFERENCES groups(id) ON DELETE SET NULL,
    properties JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_users_sub ON users(sub);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(user_email);
CREATE INDEX IF NOT EXISTS idx_users_organization_id ON users(organization_id);
CREATE INDEX IF NOT EXISTS idx_users_group_id ON users(group_id);

-- Create roles table with organization support
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_roles_name ON roles(name);
CREATE INDEX IF NOT EXISTS idx_roles_organization_id ON roles(organization_id);

-- Unique constraint on roles: name must be unique per organization
CREATE UNIQUE INDEX IF NOT EXISTS idx_roles_name_org_unique ON roles(name, COALESCE(organization_id, '00000000-0000-0000-0000-000000000000'::uuid));

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
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_permissions_role_id ON permissions(role_id);
CREATE INDEX IF NOT EXISTS idx_permissions_page ON permissions(page);
CREATE INDEX IF NOT EXISTS idx_permissions_organization_id ON permissions(organization_id);

-- Unique constraint on permissions: role_id + page must be unique per organization
CREATE UNIQUE INDEX IF NOT EXISTS idx_permissions_role_page_org_unique ON permissions(role_id, page, COALESCE(organization_id, '00000000-0000-0000-0000-000000000000'::uuid));

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

-- Create app_settings table with organization support for storing application configuration
CREATE TABLE IF NOT EXISTS app_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    setting_key VARCHAR(255) NOT NULL,
    setting_value TEXT,
    metadata JSONB DEFAULT '{}',
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_app_settings_key ON app_settings(setting_key);
CREATE INDEX IF NOT EXISTS idx_app_settings_organization_id ON app_settings(organization_id);

-- Unique constraint: setting_key must be unique per organization
CREATE UNIQUE INDEX IF NOT EXISTS idx_app_settings_org_key 
ON app_settings(organization_id, setting_key) 
WHERE organization_id IS NOT NULL;

-- Add comments explaining organization_id usage for multi-tenancy
COMMENT ON COLUMN users.organization_id IS 'Organization ID for multi-tenancy data isolation. All queries should filter by this field.';
COMMENT ON COLUMN groups.organization_id IS 'Organization ID for multi-tenancy data isolation. All queries should filter by this field.';
COMMENT ON COLUMN roles.organization_id IS 'Organization ID for multi-tenancy data isolation. All queries should filter by this field.';
COMMENT ON COLUMN permissions.organization_id IS 'Organization ID for multi-tenancy data isolation. All queries should filter by this field.';
COMMENT ON COLUMN app_settings.organization_id IS 'Organization ID for multi-tenancy data isolation. All queries should filter by this field.';

-- Insert default roles using DO block to handle conflicts properly
DO $$
BEGIN
    -- Insert Admin role if it doesn't exist
    IF NOT EXISTS (SELECT 1 FROM roles WHERE name = 'Admin' AND organization_id IS NULL) THEN
        INSERT INTO roles (name, description, is_admin)
        VALUES ('Admin', 'Full system administrator with all permissions', TRUE);
    END IF;
    
    -- Insert View role if it doesn't exist
    IF NOT EXISTS (SELECT 1 FROM roles WHERE name = 'View' AND organization_id IS NULL) THEN
        INSERT INTO roles (name, description, is_admin)
        VALUES ('View', 'View-only access to assigned pages', FALSE);
    END IF;
END $$;

-- Insert default permissions for Admin role
DO $$
DECLARE
    admin_role_id UUID;
BEGIN
    SELECT id INTO admin_role_id FROM roles WHERE name = 'Admin' AND organization_id IS NULL LIMIT 1;
    
    -- Admin has full permissions on all default pages
    -- Insert each permission only if it doesn't exist
    INSERT INTO permissions (role_id, page, can_view, can_edit, can_view_own, can_edit_own, can_view_ours, can_edit_ours)
    SELECT admin_role_id, 'dashboard', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
    WHERE NOT EXISTS (SELECT 1 FROM permissions WHERE role_id = admin_role_id AND page = 'dashboard' AND organization_id IS NULL);
    
    INSERT INTO permissions (role_id, page, can_view, can_edit, can_view_own, can_edit_own, can_view_ours, can_edit_ours)
    SELECT admin_role_id, 'users', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
    WHERE NOT EXISTS (SELECT 1 FROM permissions WHERE role_id = admin_role_id AND page = 'users' AND organization_id IS NULL);
    
    INSERT INTO permissions (role_id, page, can_view, can_edit, can_view_own, can_edit_own, can_view_ours, can_edit_ours)
    SELECT admin_role_id, 'roles', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
    WHERE NOT EXISTS (SELECT 1 FROM permissions WHERE role_id = admin_role_id AND page = 'roles' AND organization_id IS NULL);
    
    INSERT INTO permissions (role_id, page, can_view, can_edit, can_view_own, can_edit_own, can_view_ours, can_edit_ours)
    SELECT admin_role_id, 'profile', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
    WHERE NOT EXISTS (SELECT 1 FROM permissions WHERE role_id = admin_role_id AND page = 'profile' AND organization_id IS NULL);
    
    INSERT INTO permissions (role_id, page, can_view, can_edit, can_view_own, can_edit_own, can_view_ours, can_edit_ours)
    SELECT admin_role_id, 'preferences', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
    WHERE NOT EXISTS (SELECT 1 FROM permissions WHERE role_id = admin_role_id AND page = 'preferences' AND organization_id IS NULL);
    
    INSERT INTO permissions (role_id, page, can_view, can_edit, can_view_own, can_edit_own, can_view_ours, can_edit_ours)
    SELECT admin_role_id, 'support', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
    WHERE NOT EXISTS (SELECT 1 FROM permissions WHERE role_id = admin_role_id AND page = 'support' AND organization_id IS NULL);
END $$;
