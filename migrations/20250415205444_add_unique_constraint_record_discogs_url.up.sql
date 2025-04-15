-- Add a unique constraint for the combination of user_id and discogs_url
-- This prevents a user from having duplicate records with the same discogs URL
ALTER TABLE records 
ADD CONSTRAINT unique_user_discogs_url 
UNIQUE (user_id, discogs_url);
