-- This file should undo anything in `up.sql`
do
$$
begin
-- Step 1: Change the column type for the primary key to UUID
ALTER TABLE users RENAME COLUMN user_id TO id;

-- Step 2: Ensure the primary key constraint is consistent
ALTER TABLE users
  DROP COLUMN username,
  DROP COLUMN created_at, 
  DROP COLUMN updated_at;
commit;
$$;
