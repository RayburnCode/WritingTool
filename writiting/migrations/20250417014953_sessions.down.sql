-- First drop the trigger and function
DROP TRIGGER IF EXISTS update_sessions_modtime ON sessions;
DROP FUNCTION IF EXISTS update_session_modtime;

-- Then drop the indexes (order matters for partial indexes)
DROP INDEX IF EXISTS idx_sessions_token;
DROP INDEX IF EXISTS idx_sessions_expires;
DROP INDEX IF EXISTS idx_sessions_user_id;
DROP INDEX IF EXISTS idx_sessions_unique_active_token;

-- Finally drop the table
DROP TABLE IF EXISTS sessions;