-- Add multi-tenancy support with organization_id
-- This migration adds organization_id to all tables for data isolation

-- Create organizations table
CREATE TABLE IF NOT EXISTS organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_organizations_name ON organizations(name);

-- Add organization_id to users table
ALTER TABLE users ADD COLUMN IF NOT EXISTS organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_users_organization_id ON users(organization_id);

-- Add organization_id to groups table
ALTER TABLE groups ADD COLUMN IF NOT EXISTS organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_groups_organization_id ON groups(organization_id);

-- Add organization_id to roles table
ALTER TABLE roles ADD COLUMN IF NOT EXISTS organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_roles_organization_id ON roles(organization_id);

-- Add organization_id to permissions table
ALTER TABLE permissions ADD COLUMN IF NOT EXISTS organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_permissions_organization_id ON permissions(organization_id);

-- Update unique constraint on roles to include organization_id
-- First drop the old constraint if it exists
DO $$ 
BEGIN
    IF EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'roles_name_key'
    ) THEN
        ALTER TABLE roles DROP CONSTRAINT roles_name_key;
    END IF;
END $$;

-- Add new unique constraint with organization_id
CREATE UNIQUE INDEX IF NOT EXISTS idx_roles_name_org_unique ON roles(name, COALESCE(organization_id, '00000000-0000-0000-0000-000000000000'::uuid));

-- Update unique constraint on permissions to include organization_id
DO $$ 
BEGIN
    IF EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'permissions_role_id_page_key'
    ) THEN
        ALTER TABLE permissions DROP CONSTRAINT permissions_role_id_page_key;
    END IF;
END $$;

-- Add new unique constraint with organization_id
CREATE UNIQUE INDEX IF NOT EXISTS idx_permissions_role_page_org_unique ON permissions(role_id, page, COALESCE(organization_id, '00000000-0000-0000-0000-000000000000'::uuid));

-- Add comment explaining organization_id usage
COMMENT ON COLUMN users.organization_id IS 'Organization ID for multi-tenancy data isolation. All queries should filter by this field.';
COMMENT ON COLUMN groups.organization_id IS 'Organization ID for multi-tenancy data isolation. All queries should filter by this field.';
COMMENT ON COLUMN roles.organization_id IS 'Organization ID for multi-tenancy data isolation. All queries should filter by this field.';
COMMENT ON COLUMN permissions.organization_id IS 'Organization ID for multi-tenancy data isolation. All queries should filter by this field.';

-- Insert a default organization for existing data (NULL means system-wide/no organization)
-- This allows backward compatibility
INSERT INTO organizations (id, name, description)
VALUES ('00000000-0000-0000-0000-000000000000'::uuid, 'System', 'System-wide organization for global resources')
ON CONFLICT (id) DO NOTHING;
