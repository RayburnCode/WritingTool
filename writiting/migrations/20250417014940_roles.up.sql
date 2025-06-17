-- Roles table with standardized IDs
CREATE TABLE roles (
    id INTEGER PRIMARY KEY,  -- Explicit IDs for consistency
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Standard role IDs (reserve 1-99 for system roles)
INSERT INTO roles (id, name, description, is_default) VALUES 
    (1, 'super_admin', 'Full system access including user roles/permissions', false),
    (2, 'admin', 'Administrative access to most features', false),
    (3, 'moderator', 'Content moderation privileges', false),
    (4, 'user', 'Standard authenticated user', true),
    (5, 'guest', 'Limited unauthenticated access', false);

-- Permission hierarchy (reserve 100-999 for system permissions)
CREATE TABLE permissions (
    id INTEGER PRIMARY KEY,
    code VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    category VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Core permission groups
INSERT INTO permissions (id, code, description, category) VALUES
    -- User permissions (100-199)
    (100, 'user:create', 'Create new users', 'users'),
    (101, 'user:read', 'View user profiles', 'users'),
    (102, 'user:update', 'Edit user information', 'users'),
    (103, 'user:delete', 'Delete users', 'users'),
    (104, 'user:impersonate', 'Login as other users', 'users'),
    
    -- Content permissions (200-299)
    (200, 'content:create', 'Create new content', 'content'),
    (201, 'content:read', 'View content', 'content'),
    (202, 'content:update', 'Edit content', 'content'),
    (203, 'content:delete', 'Delete content', 'content'),
    (204, 'content:publish', 'Publish content', 'content'),
    
    -- System permissions (900-999)
    (900, 'system:settings', 'Modify system settings', 'system'),
    (901, 'system:audit', 'View audit logs', 'system'),
    (902, 'system:maintenance', 'Perform maintenance operations', 'system');

-- Role-Permission assignments
CREATE TABLE role_permissions (
    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id INTEGER NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- Super Admin gets everything
INSERT INTO role_permissions (role_id, permission_id)
SELECT 1, id FROM permissions;

-- Admin gets most permissions except system-level
INSERT INTO role_permissions (role_id, permission_id)
SELECT 2, id FROM permissions WHERE id < 900;

-- Moderator gets content permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT 3, id FROM permissions 
WHERE category = 'content' OR code IN ('user:read');

-- Standard user permissions
INSERT INTO role_permissions (role_id, permission_id) VALUES
    (4, 101),  -- user:read
    (4, 201),  -- content:read
    (4, 200);  -- content:create

-- Guest permissions (read-only)
INSERT INTO role_permissions (role_id, permission_id) VALUES
    (5, 101),  -- user:read
    (5, 201);  -- content:read

-- Indexes for performance
CREATE INDEX idx_role_permissions_role ON role_permissions(role_id);
CREATE INDEX idx_role_permissions_perm ON role_permissions(permission_id);

-- Add update trigger for roles
CREATE OR REPLACE FUNCTION update_role_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_role_timestamp
BEFORE UPDATE ON roles
FOR EACH ROW EXECUTE FUNCTION update_role_timestamp();