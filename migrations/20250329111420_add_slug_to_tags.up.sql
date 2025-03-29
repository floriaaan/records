-- Add up migration script here
ALTER TABLE tags
ADD COLUMN slug VARCHAR NOT NULL DEFAULT '';

-- Create unique constraint after adding default values
ALTER TABLE tags
ADD CONSTRAINT tags_slug_unique UNIQUE (slug);
