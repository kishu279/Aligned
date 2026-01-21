-- Add object_key column to store R2 storage key
ALTER TABLE user_images 
ADD COLUMN object_key TEXT;

-- Add index for faster lookups
CREATE INDEX idx_user_images_object_key ON user_images(object_key);