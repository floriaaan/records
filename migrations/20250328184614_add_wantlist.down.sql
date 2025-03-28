-- Add down migration script here
ALTER TABLE records
DROP COLUMN owned,
DROP COLUMN wanted;