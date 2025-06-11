<!-- @format -->

-- Initial migration
CREATE TABLE roles (
id SERIAL PRIMARY KEY,
name VARCHAR(50) NOT NULL UNIQUE,
permissions TEXT[] NOT NULL DEFAULT '{}'
);

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

CREATE TABLE posts (
id SERIAL PRIMARY KEY,
title VARCHAR(100) NOT NULL,
body TEXT NOT NULL,
user_id UUID NOT NULL REFERENCES users(id),
category_id INTEGER REFERENCES categories(id),
is_published BOOLEAN NOT NULL DEFAULT false,
view_count INTEGER NOT NULL DEFAULT 0,
created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add indexes
CREATE INDEX idx_posts_user_id ON posts(user_id);
CREATE INDEX idx_posts_category_id ON posts(category_id);

-- Session

CREATE TABLE sessions (
id UUID PRIMARY KEY,
user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
token TEXT NOT NULL UNIQUE,
user_agent TEXT,
ip_address TEXT,
expires_at TIMESTAMPTZ NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sessions_token ON sessions(token);
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
