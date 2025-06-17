-- First drop the trigger
DROP TRIGGER IF EXISTS update_post_timestamp ON posts;

-- Then drop the function
DROP FUNCTION IF EXISTS update_post_timestamp;

-- Then drop the index
DROP INDEX IF EXISTS idx_posts_created_at;

-- Finally drop the table
DROP TABLE IF EXISTS posts CASCADE;