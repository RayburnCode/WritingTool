<!-- @format -->

//// -- Schemas, Tables and References

Table posts {
id serial [pk, increment]
title varchar(100) [not null]
body text [not null]
created_at timestamptz [default: `now()`]
updated_at timestamptz [default: `now()`]

Indexes {
created_at [name: 'idx_posts_created_at']
}
}

//// -- Core Tables and References

Table roles {
id serial [pk, increment]
name varchar(50) [not null, unique]
description text
is_default boolean [default: false, not null]
created_at timestamptz [default: `now()`, not null]
updated_at timestamptz [default: `now()`, not null]
}

Table permissions {
id serial [pk, increment]
code varchar(100) [not null, unique]
description text
created_at timestamptz [default: `now()`, not null]
}

Table role_permissions {
role_id integer [not null]
permission_id integer [not null]

Indexes {
(role_id, permission_id) [pk]
}
}
//// -- Relationships and Constraints

Ref: role_permissions.role_id > roles.id [delete: cascade]
Ref: role_permissions.permission_id > permissions.id [delete: cascade]
//// -- Default Data Documentation

Enum default_roles {
admin
moderator
user [note: 'Default role for new users']
}

//// -- Core Tables and References

Table users {
id uuid [pk, default: `gen_random_uuid()`]
email varchar(255) [not null, unique]
username varchar(50) [not null, unique]
password_hash text [not null]
salt text [not null]
role_id integer [not null]
is_active boolean [default: true, not null]
email_verified boolean [default: false, not null]
created_at timestamptz [default: `now()`, not null]
updated_at timestamptz [default: `now()`, not null]
}

//// -- Relationships and Constraints

Ref: users.role_id > roles.id
//// -- Core Table Structure

Table sessions {
id uuid [pk, default: `gen_random_uuid()`]
user_id uuid [not null]
token text [not null, unique]
user_agent text
ip_address text
expires_at timestamptz [not null]
created_at timestamptz [default: `now()`, not null]
updated_at timestamptz [default: `now()`, not null]

Indexes {
token [name: 'idx_sessions_token']
user_id [name: 'idx_sessions_user_id']
}
}

//// -- Relationships and Security Notes

Ref: sessions.user_id > users.id [delete: cascade]

//// -- Recommended Extensions
TablePartial session_activity {
last_used_at timestamptz
is_revoked boolean [default: false]
}

Enum session_type {
browser [note: 'Web browser session']
mobile [note: 'Mobile app session']
api [note: 'Programmatic access']
}
//// -- LEVEL 1: Core Tables

Table post_comments {
id serial [pk, increment]
post_id integer [not null]
user_id uuid [not null]
parent_id integer
content text [not null]
created_at timestamptz [default: `now()`, not null]
updated_at timestamptz [default: `now()`, not null]
}

Table reactions {
id serial [pk, increment]
user_id uuid [not null]
content_type varchar(20) [not null]
content_id integer [not null]
type varchar(20) [not null]
created_at timestamptz [default: `now()`, not null]

Indexes {
(user_id, content_type, content_id) [unique]
}
}

Table notifications {
id uuid [pk, default: `gen_random_uuid()`]
user_id uuid [not null]
type varchar(50) [not null]
content jsonb [not null]
read boolean [default: false, not null]
created_at timestamptz [default: `now()`, not null]
}

Table profiles {
user_id uuid [pk]
bio text
avatar_url text
website_url text
}

Table follows {
follower_id uuid
following_id uuid
created_at timestamptz [default: `now()`, not null]

Indexes {
(follower_id, following_id) [pk]
}
}

Table bookmarks {
user_id uuid
post_id integer
created_at timestamptz [default: `now()`, not null]

Indexes {
(user_id, post_id) [pk]
}
}

Table user_preferences {
user_id uuid [pk]
language varchar(10) [default: 'en']
theme varchar(20) [default: 'light']
email_notifications boolean [default: true]
created_at timestamptz [default: `now()`]
updated_at timestamptz [default: `now()`]
}

//// -- LEVEL 2: Relationships

Ref: post_comments.post_id > posts.id [delete: cascade]
Ref: post_comments.user_id > users.id
Ref: post_comments.parent_id > post_comments.id [delete: cascade]

Ref: reactions.user_id > users.id [delete: cascade]

Ref: notifications.user_id > users.id [delete: cascade]

Ref: profiles.user_id > users.id [delete: cascade]

Ref: follows.follower_id > users.id [delete: cascade]
Ref: follows.following_id > users.id [delete: cascade]

Ref: bookmarks.user_id > users.id [delete: cascade]
Ref: bookmarks.post_id > posts.id [delete: cascade]

Ref: user_preferences.user_id > users.id [delete: cascade]

//// -- LEVEL 3: Enums and Documentation

Enum content_types {
post
comment
}

Enum reaction_types {
like
upvote
downvote
heart
laugh
}

Enum notification_types {
new_comment
reaction
follow
mention
}

//// Docs: https://dbml.dbdiagram.io/docs
//// -- LEVEL 1: Core Security Tables

Table password_reset_tokens {
token uuid [pk, default: `gen_random_uuid()`]
user_id uuid [not null]
expires_at timestamptz [not null]
used boolean [default: false, not null]

Indexes {
user_id
expires_at
}
}

Table email_verification_tokens {
token uuid [pk, default: `gen_random_uuid()`]
user_id uuid [not null, unique]
expires_at timestamptz [not null]

Indexes {
expires_at
}
}

//// -- LEVEL 2: Relationships and Constraints

Ref: password_reset_tokens.user_id > users.id [delete: cascade]
Ref: email_verification_tokens.user_id > users.id [delete: cascade]

//// -- Recommended Extensions

TablePartial token_metadata {
created_at timestamptz [default: `now()`, not null]
ip_address inet
user_agent text
}

Enum token_status {
active
used
revoked
}

//// -- Audit Trail Suggestion

Table security_logs {
id bigserial [pk]
user_id uuid [ref: > users.id]
action_type varchar(50) [not null] // 'password_reset_requested'
token_id uuid
ip_address inet
created_at timestamptz [default: `now()`]
}

//// Docs: https://dbml.dbdiagram.io/docs
//// -- LEVEL 1: Core Audit Structure

Table audit_logs {
id bigserial [pk, increment]
user_id uuid
action varchar(50) [not null]
entity_type varchar(50)
entity_id text // Flexible ID storage
ip_address inet
user_agent text
metadata jsonb
created_at timestamptz [default: `now()`, not null]

Indexes {
user_id [name: 'idx_audit_logs_user_id']
(entity_type, entity_id) [name: 'idx_audit_logs_entity']
created_at
}
}

//// -- LEVEL 2: Relationships and Retention

Ref: audit_logs.user_id > users.id [delete: set null]

//// -- Action Taxonomy

Enum security_actions {
user_login
password_change
role_modification
token_issued
permission_granted
}

Enum content_actions {
post_created
comment_deleted
reaction_added
profile_updated
}

//// -- Advanced Features

TablePartial audit_analysis {
risk_score smallint [note: '0-100 anomaly detection']
investigation_status varchar(20) [default: 'unreviewed']
}

//// Docs: https://dbml.dbdiagram.io/docs
//// -- LEVEL 1: Core Rate Limiting Structure

Table rate_limits {
bucket varchar(255) [pk]
tokens decimal [not null]
last_refill timestamptz [not null]
expires_at timestamptz [not null]

Indexes {
expires_at [name: 'idx_rate_limits_expiry']
}
}

Enum rate_limit_strategy {
token_bucket [note: 'Standard token bucket algorithm']
fixed_window
sliding_window_log
sliding_window_counter
}

//// -- Suggested Extensions

TablePartial limit_metadata {
created_at timestamptz [default: `now()`]
modified_at timestamptz [default: `now()`]
violation_count integer [default: 0]
}

Enum limit_state {
active
warned
throttled
banned
}

//// Docs: https://dbml.dbdiagram.io/docs
//// -- LEVEL 1: Core API Key Structure

Table api_keys {
key uuid [pk, default: `gen_random_uuid()`]
user_id uuid [not null]
name text [not null]
scopes text[] [not null]
expires_at timestamptz
last_used_at timestamptz
created_at timestamptz [default: `now()`]

Indexes {
user_id
expires_at [name: 'idx_api_keys_expiry']
}
}

//// -- LEVEL 2: Relationships and Security Policies

Ref: api_keys.user_id > users.id

//// -- Scope Taxonomy

Enum api_scopes {
read
write
delete
admin
metrics
audit
}

//// -- Monitoring Extensions

TablePartial key_audit {
revoked_at timestamptz
revoked_by uuid [ref: > users.id]
last_ip inet
usage_count integer [default: 0]
}

Enum key_status {
active
expired
revoked
compromised
}

//// Docs: https://dbml.dbdiagram.io/docs
//// -- LEVEL 1: Core Feature Flag Structure

Table feature_flags {
name varchar(100) [pk]
is_enabled boolean [default: false]
rollout_percentage integer [default: 0, note: '0-100 scale']
target_users uuid[] [note: 'Bypass percentage for these users']
created_at timestamptz [default: `now()`]
updated_at timestamptz [default: `now()`]

Indexes {
(is_enabled, rollout_percentage) [name: 'idx_flag_activation']
updated_at
}
}

//// -- Advanced Controls

Enum rollout_phase {
dev [note: 'Internal testing only']
staging
canary [note: '1-5% production']
progressive [note: '25/50/75% stages']
general [note: '100% available']
}

TablePartial flag_metadata {
description text
owner varchar(100) [note: 'Team responsible']
docs_url text
}

//// -- Audit & Compliance

Table feature_flag_history {
flag_name varchar(100) [ref: > feature_flags.name]
changed_by uuid [ref: > users.id]
old_state jsonb
new_state jsonb
changed_at timestamptz [default: `now()`]

Indexes {
flag_name
changed_at [name: 'idx_flag_change_history']
}
}

//// -- Performance Optimization

//// Docs: https://dbml.dbdiagram.io/docs
//// -- LEVEL 1: Core Monitoring Structure

Table database_connections {
id bigserial [pk, increment]
application_name text [not null]
connection_count integer [not null]
max_connections integer [not null]
created_at timestamptz [default: `now()`]

Indexes {
application_name
created_at [name: 'idx_connection_trends']
}
}

//// -- Alert Thresholds

TablePartial connection_alert_rules {
app_name text [note: 'Null means all applications']
warn_threshold_pct integer [default: 70, note: '0-100% of max']
critical_threshold_pct integer [default: 90]
notify_channels text[] [default: '{slack_ops}']
}

//// -- Extended Metrics (Recommended)

Table database_metrics {
id bigserial [pk]
metric_name text [not null]
metric_value numeric [not null]
created_at timestamptz [default: `now()`]

Indexes {
metric_name
created_at
}
}

//// Docs: https://dbml.dbdiagram.io/docs
//// -- LEVEL 1: Core Vault Structure

Table encrypted_secrets {
id uuid [pk, default: `gen_random_uuid()`]
name varchar(100) [not null, unique]
current_value bytea [not null]
previous_value bytea
encryption_key_id varchar(100) [not null]
rotated_at timestamptz [not null]
created_at timestamptz [default: `now()`]

Indexes {
name [name: 'idx_encrypted_secrets_name']
rotated_at
}
}

//// -- Rotation Workflow

Enum secret_status {
active
staged [note: 'New version prepared but not activated']
retired
compromised
}

TablePartial rotation_metadata {
staged_value bytea
staged_key_id varchar(100)
next_rotation timestamptz
last_audit timestamptz
}

//// -- Audit Requirements

Table secret_access_logs {
id bigserial [pk]
secret_id uuid [ref: > encrypted_secrets.id]
accessed_by uuid [ref: > users.id]
operation varchar(20) [note: 'decrypt/rotate/read_meta']
accessed_at timestamptz [default: `now()`]
client_ip inet
user_agent text

Indexes {
secret_id
accessed_at [name: 'idx_secret_access_times']
}
}

//// Docs: https://dbml.dbdiagram.io/docs
//// -- LEVEL 1: Core Maintenance Structure

Table maintenance_windows {
id uuid [pk, default: `gen_random_uuid()`]
starts_at timestamptz [not null]
ends_at timestamptz [not null]
description text [not null]
is_active boolean
created_by uuid
created_at timestamptz [default: `now()`]

Indexes {
(starts_at, ends_at) [name: 'idx_maintenance_schedule']
is_active [name: 'idx_active_maintenance']
}
}

//// -- Impact Analysis Extensions

TablePartial affected_services {
service_name varchar(50) [not null]
impact_level varchar(20) [note: 'full/partial/degraded']
expected_downtime_minutes integer
}

Enum maintenance_type {
database [note: 'Schema migrations']
infrastructure [note: 'Hardware upgrades']
security [note: 'Emergency patches']
compliance [note: 'Audit requirements']
}

//// -- Audit Requirements

Table maintenance_logs {
window_id uuid [ref: > maintenance_windows.id]
action varchar(50) [not null]
performed_by uuid [ref: > users.id]
performed_at timestamptz [default: `now()`]

Indexes {
window_id
performed_at
}
}
