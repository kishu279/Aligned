-- Add firebase_uid column with NOT NULL constraint and unique index
-- This migration is idempotent (safe to run even if column already exists)

-- Add column if not exists
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'users' AND column_name = 'firebase_uid'
    ) THEN
        ALTER TABLE users ADD COLUMN firebase_uid VARCHAR(128);
    END IF;
END $$;

-- Backfill existing rows with temp values
UPDATE users
SET firebase_uid = 'TEMP_' || id::text
WHERE firebase_uid IS NULL;

-- Make NOT NULL if not already
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'users' AND column_name = 'firebase_uid' AND is_nullable = 'YES'
    ) THEN
        ALTER TABLE users ALTER COLUMN firebase_uid SET NOT NULL;
    END IF;
END $$;

-- Add unique constraint if not exists
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'users_firebase_uid_key'
    ) THEN
        ALTER TABLE users ADD CONSTRAINT users_firebase_uid_key UNIQUE (firebase_uid);
    END IF;
END $$;
