-- writiting/migrations/20250611192540_users.sql
CREATE TABLE users (
id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
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

ALTER TABLE users ALTER COLUMN id TYPE UUID USING gen_random_uuid();



