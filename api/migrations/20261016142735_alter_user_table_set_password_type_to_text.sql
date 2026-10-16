-- hashed password length can span up to a variety of lengths
-- depending on the hashing algorithm used. To accommodate this,
-- we will change the password_hash column to use the TEXT data type instead of VARCHAR(80).

ALTER TABLE users
ALTER COLUMN password_hash TYPE TEXT;