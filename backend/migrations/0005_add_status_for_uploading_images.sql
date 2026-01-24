-- Add migration script here
ALTER TABLE user_images
ADD COLUMN upload_status VARCHAR(50) DEFAULT 'pending';