-- Add migration script here

ALTER TABLE users
ADD COLUMN firebase_uid VARCHAR(128);

ALTER TABLE users SET NOT NULL;
ADD CONSTRAINT users_firebase_uid_key UNIQUE(firebase_uid);
