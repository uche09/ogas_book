-- Add migration script here

ALTER TABLE users
ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE businesses
ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE refresh_tokens
ALTER COLUMN id SET DEFAULT gen_random_uuid();