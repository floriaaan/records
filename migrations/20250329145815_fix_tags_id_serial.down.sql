-- Revert the changes to the tags table
-- First, drop the foreign key constraint
ALTER TABLE records_tags DROP CONSTRAINT IF EXISTS records_tags_tag_id_fkey;

-- Remove the SERIAL default
ALTER TABLE tags ALTER COLUMN id DROP DEFAULT;

-- Drop the sequence
DROP SEQUENCE IF EXISTS tags_id_seq;

-- Re-create the foreign key constraint
ALTER TABLE records_tags ADD CONSTRAINT records_tags_tag_id_fkey 
    FOREIGN KEY (tag_id) REFERENCES tags (id);
