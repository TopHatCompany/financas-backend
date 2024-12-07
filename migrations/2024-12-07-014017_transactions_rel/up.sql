-- Your SQL goes here
BEGIN;

  ALTER TABLE transactions
    DROP CONSTRAINT transactions_account_check;

  ALTER TABLE transactions
    RENAME COLUMN account TO account_id;

  ALTER TABLE transactions
    ALTER COLUMN account_id SET DATA TYPE UUID USING "account_id"::UUID;

  ALTER TABLE IF EXISTS public.transactions
    ADD Constraint fk_transactions_account_id FOREIGN KEY(account_id) REFERENCES users_accounts(account_id);

COMMIT;
