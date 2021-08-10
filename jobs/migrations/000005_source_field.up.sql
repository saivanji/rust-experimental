BEGIN;
CREATE TYPE source AS ENUM ('stackoverflow');
ALTER TABLE jobs ADD COLUMN source source NOT NULL;
COMMIT;
