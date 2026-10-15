-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    first_name VARCHAR(70) NOT NULL,
    last_name VARCHAR(70) NOT NULL,
    other_name VARCHAR(70),
    email VARCHAR(254) UNIQUE,
    password_hash VARCHAR(80) NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);