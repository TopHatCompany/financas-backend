-- Your SQL goes here
CREATE TABLE users_accounts (
   id             UUID NOT NULL DEFAULT public.uuid_generate_v7(),
   user_id        UUID NOT NULL,
   identification TEXT NOT NULL CHECK (identification <> ''),
   kind           TEXT NOT NULL CHECK (kind <> ''),
   initial_amount NUMERIC NOT NULL,
   PRIMARY KEY (id),
   CONSTRAINT fk_users
      FOREIGN KEY (user_id)
      REFERENCES users(id)
      ON UPDATE CASCADE
      ON DELETE CASCADE
);
