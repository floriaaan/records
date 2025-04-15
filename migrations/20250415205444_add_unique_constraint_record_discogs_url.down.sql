-- Remove the unique constraint for the combination of user_id and discogs_url
ALTER TABLE records 
DROP CONSTRAINT unique_user_discogs_url;
