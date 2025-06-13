CREATE TABLE rate_limits (
  bucket VARCHAR(255) PRIMARY KEY, -- e.g., "login:192.168.1.1"
  tokens DECIMAL NOT NULL,
  last_refill TIMESTAMPTZ NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL
);