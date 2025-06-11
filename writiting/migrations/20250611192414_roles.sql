-- Add migration script here
-- Roles table
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Permissions table
CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    code VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Role-Permission junction table
CREATE TABLE role_permissions (
    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id INTEGER NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- Set up default roles
INSERT INTO roles (name, is_default) VALUES 
    ('admin', false),
    ('moderator', false),
    ('user', true);

-- Common permissions
INSERT INTO permissions (code, description) VALUES
    ('user:read', 'Read user information'),
    ('user:write', 'Create or update users'),
    ('user:delete', 'Delete users'),
    ('post:read', 'Read posts'),
    ('post:write', 'Create or update posts'),
    ('post:delete', 'Delete posts'),
    ('comment:write', 'Create comments'),
    ('comment:delete', 'Delete comments');

-- Admin permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p
WHERE r.name = 'admin';

-- Moderator permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p
WHERE r.name = 'moderator' AND p.code IN (
    'post:read', 'post:write', 'post:delete',
    'comment:write', 'comment:delete'
);

-- User permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p
WHERE r.name = 'user' AND p.code IN (
    'post:read', 'comment:write'
);