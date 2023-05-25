-- Your SQL goes here

CREATE TABLE IF NOT EXISTS transactions
(
  id              UUID      NOT NULL DEFAULT public.uuid_generate_v7(),
  transacted_date DATE      NOT NULL,
  amount          NUMERIC   NOT NULL,
  currency        TEXT      NOT NULL CHECK (currency <> ''),
  account         TEXT      NOT NULL CHECK (account <> ''),
  description     TEXT      NOT NULL,
  label           TEXT      NOT NULL CHECK (label <> ''),
  kind            TEXT      NOT NULL CHECK (kind <> ''),
  opts            TEXT      NOT NULL,
  created_at      TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at      TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id)
);
