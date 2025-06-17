-- Drop views first (they depend on tables)
DROP VIEW IF EXISTS post_stats;

-- Drop indexes in reverse order of creation
DROP INDEX IF EXISTS idx_bookmarks_created_at;
DROP INDEX IF EXISTS idx_bookmarks_user_id;
DROP INDEX IF EXISTS idx_follows_following_id;
DROP INDEX IF EXISTS idx_follows_follower_id;
DROP INDEX IF EXISTS idx_notifications_created_at;
DROP INDEX IF EXISTS idx_notifications_user_id_unread;
DROP INDEX IF EXISTS idx_comment_reactions_user_id;
DROP INDEX IF EXISTS idx_comment_reactions_comment_id;
DROP INDEX IF EXISTS idx_post_reactions_user_id;
DROP INDEX IF EXISTS idx_post_reactions_post_id;
DROP INDEX IF EXISTS idx_post_comments_created_at;
DROP INDEX IF EXISTS idx_post_comments_parent_id;
DROP INDEX IF EXISTS idx_post_comments_user_id;
DROP INDEX IF EXISTS idx_post_comments_post_id;

-- Drop tables in reverse order of foreign key dependencies
DROP TABLE IF EXISTS bookmarks;
DROP TABLE IF EXISTS follows;
DROP TABLE IF EXISTS notifications;
DROP TABLE IF EXISTS comment_reactions;
DROP TABLE IF EXISTS post_reactions;
DROP TABLE IF EXISTS post_comments;

-- Drop custom types
DROP TYPE IF EXISTS notification_type;
DROP TYPE IF EXISTS reaction_type;
DROP TYPE IF EXISTS content_type;