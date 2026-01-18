-- Add migration script here

-- Make email NOT NULL
ALTER TABLE users
ALTER COLUMN email SET NOT NULL;

-- Make email UNIQUE
ALTER TABLE users
ADD CONSTRAINT users_email_key UNIQUE (email);
