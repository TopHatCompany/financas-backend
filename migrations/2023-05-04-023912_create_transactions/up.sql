-- Your SQL goes here

CREATE TABLE IF NOT EXISTS transactions
(
  id              UUID      NOT NULL DEFAULT public.uuid_generate_v7(),
  transacted_date DATE      NOT NULL,
  amount          NUMERIC   NOT NULL,
  description     TEXT      NOT NULL CHECK (kind <> ''),
  label           TEXT      NOT NULL CHECK (kind <> ''),
  kind            TEXT      NOT NULL CHECK (kind <> ''),
  created_at      TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at      TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id)
);
