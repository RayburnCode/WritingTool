-- Insert sample roles if they don't exist
INSERT INTO roles (id, name, is_default) 
VALUES 
  (1, 'admin', false),
  (2, 'user', true)
ON CONFLICT (id) DO NOTHING;

-- Insert test users (removed trailing comma after is_active)
INSERT INTO users (
  id, 
  email, 
  username, 
  password_hash, 
  salt, 
  role_id, 
  is_active  -- No comma here
) VALUES (
  'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
  'test@example.com',
  'testuser',
  -- bcrypt hash for "password123"
  '$2a$12$EixZaYVK1fsbw1ZfbX3OXePaWxn96p36WQoeG6Lruj3vjPGga31lW',
  'random-salt-value',
  2,  -- user role
  true  -- No comma here
);

-- Insert profile for test user (removed trailing comma after avatar_url)
INSERT INTO profiles (
  user_id,
  first_name,
  last_name,
  avatar_url  -- No comma here
) VALUES (
  'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
  'Test',
  'User',
  'https://www.gravatar.com/avatar/2c7d99fe281ecd3bcd65ab915bac6dd5?s=250'  -- No comma here
);

-- Insert test posts
INSERT INTO posts (
    user_id,
    title,
    body  -- No comma here
) VALUES 
    -- Post from test user
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'My First Post',
        'This is an example post content.'  -- No comma here
    ),
    -- Post from admin user
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'Admin Announcement',
        'Welcome to our platform!'  -- No comma here
    ),
    -- Additional test posts
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'Another Test Post',
        'Just adding more content to test pagination.'  -- No comma here
    );