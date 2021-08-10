BEGIN;

ALTER TABLE jobs ALTER COLUMN remote DROP NOT NULL;

CREATE TYPE _remote AS ENUM ('yes', 'no', 'partial');
ALTER TABLE jobs ALTER COLUMN remote TYPE _remote USING remote::text::_remote;

DROP TYPE remote;
ALTER TYPE _remote RENAME TO remote;

COMMIT;
