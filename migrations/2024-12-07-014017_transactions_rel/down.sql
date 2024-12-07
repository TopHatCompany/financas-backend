-- This file should undo anything in `up.sql
begin;
ALTER TABLE transactions
DROP CONSTRAINT fk_transactions_account_id;

  ALTER TABLE IF EXISTS public.transactions
    ADD Constraint transactions_account_check account <> ''::text;

  ALTER TABLE transactions
    RENAME COLUMN account_id TO account;

  ALTER TABLE transactions
    ALTER COLUMN account SET DATA TYPE TEXT;

end;
