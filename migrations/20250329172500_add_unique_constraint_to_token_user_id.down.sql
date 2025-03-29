-- Remove the unique constraint on user_id
ALTER TABLE collection_tokens DROP CONSTRAINT unique_user_token;