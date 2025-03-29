-- Clean up any duplicate tokens - keep only the most recent token for each user
WITH duplicates AS (
    SELECT id, user_id, 
           ROW_NUMBER() OVER (PARTITION BY user_id ORDER BY created_at DESC) as row_num
    FROM collection_tokens
)
DELETE FROM collection_tokens
WHERE id IN (
    SELECT id FROM duplicates WHERE row_num > 1
);

-- Add a unique constraint to user_id to ensure one token per user
ALTER TABLE collection_tokens ADD CONSTRAINT unique_user_token UNIQUE (user_id);