-- Add organization_id to app_settings table for multi-tenancy isolation
ALTER TABLE IF EXISTS app_settings
ADD COLUMN organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE;

-- Create index on organization_id for query performance
CREATE INDEX IF NOT EXISTS idx_app_settings_organization_id ON app_settings(organization_id);

-- Update the unique constraint to include organization_id
-- Drop the old constraint if it exists
ALTER TABLE IF EXISTS app_settings DROP CONSTRAINT IF EXISTS app_settings_setting_key_key;

-- Create new unique constraint that includes organization_id
-- This allows the same setting_key in different organizations
CREATE UNIQUE INDEX IF NOT EXISTS idx_app_settings_org_key 
ON app_settings(organization_id, setting_key) 
WHERE organization_id IS NOT NULL;
