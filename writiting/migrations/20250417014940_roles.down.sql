-- Drop the trigger first to avoid dependency issues
DROP TRIGGER IF EXISTS trigger_update_role_timestamp ON roles;

-- Drop the function
DROP FUNCTION IF EXISTS update_role_timestamp;

-- Drop indexes (order matters due to dependencies)
DROP INDEX IF EXISTS idx_role_permissions_perm;
DROP INDEX IF EXISTS idx_role_permissions_role;

-- Drop junction table first due to foreign key constraints
DROP TABLE IF EXISTS role_permissions;

-- Then drop permissions table
DROP TABLE IF EXISTS permissions;

-- Finally drop roles table
DROP TABLE IF EXISTS roles;