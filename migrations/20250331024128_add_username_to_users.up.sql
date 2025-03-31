ALTER TABLE users ADD COLUMN username VARCHAR(255) UNIQUE;
-- Initially set username to email for existing users
UPDATE users SET username = email WHERE username IS NULL;
-- Make username NOT NULL after setting initial values
ALTER TABLE users ALTER COLUMN username SET NOT NULL;
