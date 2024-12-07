-- Your SQL goes here
BEGIN;
-- Step 1: Change the column type for the primary key to UUID
ALTER TABLE users RENAME COLUMN id TO user_id;

-- Step 2: Ensure the primary key constraint is consistent
ALTER TABLE users
  ADD COLUMN username TEXT DEFAULT 'username',
  ADD COLUMN created_at TIMESTAMP NOT NULL  DEFAULT NOW(),
  ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW();
COMMIT;
