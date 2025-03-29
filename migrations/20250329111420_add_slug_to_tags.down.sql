-- Add down migration script here
ALTER TABLE tags
DROP CONSTRAINT IF EXISTS tags_slug_unique;

ALTER TABLE tags
DROP COLUMN IF EXISTS slug;
