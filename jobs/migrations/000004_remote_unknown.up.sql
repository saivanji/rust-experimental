BEGIN;

CREATE TYPE _remote AS ENUM ('yes', 'no', 'partial', 'unknown');
ALTER TABLE jobs ALTER COLUMN remote TYPE _remote USING remote::text::_remote;

DROP TYPE remote;
ALTER TYPE _remote RENAME TO remote;

ALTER TABLE jobs ALTER COLUMN remote SET NOT NULL;

COMMIT;
