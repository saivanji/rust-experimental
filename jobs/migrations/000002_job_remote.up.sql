BEGIN;
CREATE TYPE remote AS ENUM ('yes', 'no', 'partial');
ALTER TABLE jobs ADD COLUMN remote remote;
COMMIT;
