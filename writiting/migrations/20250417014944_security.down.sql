-- Drop indexes first (they depend on tables)
DROP INDEX IF EXISTS idx_account_lockouts_active;
DROP INDEX IF EXISTS idx_account_lockouts_user_id;
DROP INDEX IF EXISTS idx_audit_logs_created_at;
DROP INDEX IF EXISTS idx_audit_logs_entity;
DROP INDEX IF EXISTS idx_audit_logs_user_id;
DROP INDEX IF EXISTS idx_security_logs_ip;
DROP INDEX IF EXISTS idx_security_logs_created_at;
DROP INDEX IF EXISTS idx_security_logs_action;
DROP INDEX IF EXISTS idx_security_logs_user_id;
DROP INDEX IF EXISTS idx_rate_limit_expires;
DROP INDEX IF EXISTS idx_api_keys_active;
DROP INDEX IF EXISTS idx_api_keys_user;
DROP INDEX IF EXISTS idx_email_verification_active;
DROP INDEX IF EXISTS idx_email_verification_unique_user;
DROP INDEX IF EXISTS idx_password_reset_active;

-- Drop tables in reverse order of creation
DROP TABLE IF EXISTS account_lockouts;
DROP TABLE IF EXISTS audit_logs;
DROP TABLE IF EXISTS security_logs;
DROP TABLE IF EXISTS rate_limits;
DROP TABLE IF EXISTS api_keys;
DROP TABLE IF EXISTS email_verification_tokens;
DROP TABLE IF EXISTS password_reset_tokens;

-- Drop custom types last (they're used by tables)
DROP TYPE IF EXISTS audit_action;
DROP TYPE IF EXISTS security_action;
DROP TYPE IF EXISTS api_scope;