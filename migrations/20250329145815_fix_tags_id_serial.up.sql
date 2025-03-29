-- Fix the tags table to use SERIAL for the ID column
-- First, drop any existing constraints
ALTER TABLE records_tags DROP CONSTRAINT IF EXISTS records_tags_tag_id_fkey;

-- Change the id column to use SERIAL
CREATE SEQUENCE IF NOT EXISTS tags_id_seq;
ALTER TABLE tags ALTER COLUMN id SET DEFAULT nextval('tags_id_seq');
ALTER SEQUENCE tags_id_seq OWNED BY tags.id;
-- Set the sequence to start from the current maximum ID + 1
SELECT setval('tags_id_seq', COALESCE((SELECT MAX(id) FROM tags), 0) + 1, false);

-- Re-create the foreign key constraint
ALTER TABLE records_tags ADD CONSTRAINT records_tags_tag_id_fkey 
    FOREIGN KEY (tag_id) REFERENCES tags (id);
