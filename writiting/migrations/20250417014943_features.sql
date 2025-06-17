-- writiting/migrations/20250417014943_features.sql

-- Comments system with proper indexing
CREATE TABLE post_comments (
  id SERIAL PRIMARY KEY,
  post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE SET NULL, -- Keep comments if user deleted
  parent_id INTEGER REFERENCES post_comments(id) ON DELETE CASCADE,
  content TEXT NOT NULL CHECK (LENGTH(TRIM(content)) > 0),
  is_deleted BOOLEAN NOT NULL DEFAULT false, -- Soft delete for moderation
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for comments
CREATE INDEX idx_post_comments_post_id ON post_comments(post_id);
CREATE INDEX idx_post_comments_user_id ON post_comments(user_id);
CREATE INDEX idx_post_comments_parent_id ON post_comments(parent_id);
CREATE INDEX idx_post_comments_created_at ON post_comments(created_at);

-- Reactions with proper type safety
CREATE TYPE reaction_type AS ENUM ('like', 'upvote', 'downvote', 'heart', 'laugh', 'angry');
CREATE TYPE content_type AS ENUM ('post', 'comment');

-- Separate tables for different reaction targets (better than generic approach)
CREATE TABLE post_reactions (
  id SERIAL PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
  reaction_type reaction_type NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT uq_user_post_reaction UNIQUE (user_id, post_id)
);

CREATE TABLE comment_reactions (
  id SERIAL PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  comment_id INTEGER NOT NULL REFERENCES post_comments(id) ON DELETE CASCADE,
  reaction_type reaction_type NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT uq_user_comment_reaction UNIQUE (user_id, comment_id)
);

-- Indexes for reactions
CREATE INDEX idx_post_reactions_post_id ON post_reactions(post_id);
CREATE INDEX idx_post_reactions_user_id ON post_reactions(user_id);
CREATE INDEX idx_comment_reactions_comment_id ON comment_reactions(comment_id);
CREATE INDEX idx_comment_reactions_user_id ON comment_reactions(user_id);

-- Notifications with proper type constraints
CREATE TYPE notification_type AS ENUM (
  'comment_reply', 'post_reaction', 'comment_reaction', 
  'new_follower', 'post_mention', 'comment_mention'
);

CREATE TABLE notifications (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  type notification_type NOT NULL,
  content JSONB NOT NULL,
  read BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for notifications
CREATE INDEX idx_notifications_user_id_unread ON notifications(user_id) WHERE read = false;
CREATE INDEX idx_notifications_created_at ON notifications(created_at);

-- Follow system with proper constraints
CREATE TABLE follows (
  follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  following_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (follower_id, following_id),
  CONSTRAINT no_self_follow CHECK (follower_id != following_id)
);

-- Indexes for follows
CREATE INDEX idx_follows_follower_id ON follows(follower_id);
CREATE INDEX idx_follows_following_id ON follows(following_id);

-- Bookmarks
CREATE TABLE bookmarks (
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_id, post_id)
);

-- Indexes for bookmarks
CREATE INDEX idx_bookmarks_user_id ON bookmarks(user_id);
CREATE INDEX idx_bookmarks_created_at ON bookmarks(created_at);

-- Views for common queries
CREATE VIEW post_stats AS
SELECT 
  p.id,
  p.title,
  COUNT(DISTINCT pc.id) as comment_count,
  COUNT(DISTINCT pr.id) as reaction_count,
  COUNT(DISTINCT b.user_id) as bookmark_count
FROM posts p
LEFT JOIN post_comments pc ON p.id = pc.post_id AND pc.is_deleted = false
LEFT JOIN post_reactions pr ON p.id = pr.post_id
LEFT JOIN bookmarks b ON p.id = b.post_id
GROUP BY p.id, p.title;