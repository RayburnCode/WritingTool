-- Add migration script here
CREATE TABLE users (
id UUID PRIMARY KEY,
email VARCHAR(255) NOT NULL UNIQUE,
username VARCHAR(50) NOT NULL UNIQUE,
password_hash TEXT NOT NULL,
salt TEXT NOT NULL,
role_id INTEGER NOT NULL REFERENCES roles(id),
is_active BOOLEAN NOT NULL DEFAULT true,
email_verified BOOLEAN NOT NULL DEFAULT false,
created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- Add indexes
CREATE INDEX idx_posts_user_id ON posts(user_id);
CREATE INDEX idx_posts_category_id ON posts(category_id);
