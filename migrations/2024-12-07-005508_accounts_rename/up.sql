-- Your SQL goes here
BEGIN;
-- Step 1: Change the column type for the primary key to UUID
ALTER TABLE users_accounts RENAME COLUMN id TO account_id;

-- Step 2: Ensure the primary key constraint is consistent
ALTER TABLE users_accounts
  ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW();
COMMIT;
