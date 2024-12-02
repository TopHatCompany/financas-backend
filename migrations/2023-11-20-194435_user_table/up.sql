-- Your SQL goes here
CREATE TABLE users (
    id             UUID NOT NULL DEFAULT public.uuid_generate_v7(),
    identification TEXT NOT NULL CHECK (identification <> ''),
    PRIMARY KEY (id)
);
