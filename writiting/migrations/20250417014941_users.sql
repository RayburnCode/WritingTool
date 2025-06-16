-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Users table with optimized constraints and indexing
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email VARCHAR(255) NOT NULL,
  username VARCHAR(50) NOT NULL,
  password_hash TEXT NOT NULL,
  salt TEXT NOT NULL,
  role_id INTEGER NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT true,
  email_verified BOOLEAN NOT NULL DEFAULT false,
  last_login_at TIMESTAMPTZ,
  reset_token TEXT,
  reset_token_expires_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  CONSTRAINT fk_role FOREIGN KEY (role_id) REFERENCES roles(id)
);

-- Create indexes after table creation for better performance
CREATE UNIQUE INDEX idx_users_email ON users (LOWER(email));
CREATE UNIQUE INDEX idx_users_username ON users (LOWER(username));
CREATE INDEX idx_users_role_id ON users (role_id) WHERE is_active = true;
CREATE INDEX idx_users_reset_token ON users (reset_token) WHERE reset_token IS NOT NULL;

-- User profiles with optimized JSONB and text fields
CREATE TABLE profiles (
  user_id UUID PRIMARY KEY,
  first_name VARCHAR(100),
  last_name VARCHAR(100),
  bio TEXT CHECK (LENGTH(bio) <= 2000),
  avatar_url TEXT CHECK (avatar_url ~ '^https?://'),
  website_url TEXT CHECK (website_url ~ '^https?://'),
  social_links JSONB DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Add GIN index for JSONB field if you'll query it
CREATE INDEX idx_profiles_social_links ON profiles USING GIN (social_links);

-- User preferences with enum-like constraints
CREATE TABLE user_preferences (
  user_id UUID PRIMARY KEY,
  language VARCHAR(10) NOT NULL DEFAULT 'en' 
    CHECK (language IN ('en', 'es', 'fr', 'de', 'ja', 'zh')),
  theme VARCHAR(20) NOT NULL DEFAULT 'light' 
    CHECK (theme IN ('light', 'dark', 'system')),
  email_notifications BOOLEAN NOT NULL DEFAULT true,
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Social media platforms with validation
CREATE TABLE social_media_platforms (
  id SMALLSERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  base_url VARCHAR(255) NOT NULL CHECK (base_url ~ '^https?://'),
  icon_class VARCHAR(50),
  CONSTRAINT uq_platform_name UNIQUE (name)
);

-- Optimized user social links with proper constraints
CREATE TABLE user_social_links (
  id SERIAL PRIMARY KEY,
  user_id UUID NOT NULL,
  platform_id SMALLINT NOT NULL,
  username VARCHAR(100) NOT NULL,
  is_public BOOLEAN NOT NULL DEFAULT true,
  created_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT fk_platform FOREIGN KEY (platform_id) REFERENCES social_media_platforms(id),
  CONSTRAINT uq_user_platform UNIQUE (user_id, platform_id)
);

-- Create a view for commonly accessed social links
CREATE VIEW user_social_profiles AS
SELECT 
  u.id AS user_id,
  u.username,
  jsonb_object_agg(smp.name, smp.base_url || usl.username) AS social_profiles
FROM 
  users u
LEFT JOIN 
  user_social_links usl ON u.id = usl.user_id
LEFT JOIN 
  social_media_platforms smp ON usl.platform_id = smp.id
WHERE 
  usl.is_public = true
GROUP BY 
  u.id, u.username;

-- Pre-populate common platforms (using transaction for atomicity)
BEGIN;
INSERT INTO social_media_platforms (name, base_url, icon_class) VALUES
  ('Instagram', 'https://instagram.com/', 'fa-instagram'),
  ('Twitter/X', 'https://x.com/', 'fa-x-twitter'),
  ('LinkedIn', 'https://linkedin.com/in/', 'fa-linkedin'),
  ('GitHub', 'https://github.com/', 'fa-github'),
  ('Facebook', 'https://facebook.com/', 'fa-facebook'),
  ('YouTube', 'https://youtube.com/', 'fa-youtube'),
  ('TikTok', 'https://tiktok.com/', 'fa-tiktok'),
  ('Discord', 'https://discord.com/', 'fa-discord');
COMMIT;