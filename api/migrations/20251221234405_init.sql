-- Add migration script here

CREATE TABLE users (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  username TEXT UNIQUE,
  phone_number TEXT NOT NULL,
  country country NOT NULL,
  role user_role NOT NULL DEFAULT 'freelancer',
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE otps (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,\
  email TEXT NOT NULL,
  otp_hash TEXT NOT NULL,
  purpose TEXT NOT NULL CHECK (purpose IN ('password_reset')),
  expires_at TIMESTAMPTZ NOT NULL,
  used_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
