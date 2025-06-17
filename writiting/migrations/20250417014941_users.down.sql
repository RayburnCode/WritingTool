-- Drop the view first (depends on tables)
DROP VIEW IF EXISTS user_social_profiles;

-- Drop indexes in reverse order of creation
DROP INDEX IF EXISTS idx_user_social_links_platform_id;
DROP INDEX IF EXISTS idx_user_social_links_user_id;
DROP INDEX IF EXISTS idx_users_reset_token;
DROP INDEX IF EXISTS idx_users_role_id;
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_users_email;

-- Drop tables in reverse order of foreign key dependencies
DROP TABLE IF EXISTS user_social_links;
DROP TABLE IF EXISTS social_media_platforms;
DROP TABLE IF EXISTS user_preferences;
DROP TABLE IF EXISTS profiles;
DROP TABLE IF EXISTS users;

-- Only drop extension if you're sure no other tables use it
-- DROP EXTENSION IF EXISTS "pgcrypto";