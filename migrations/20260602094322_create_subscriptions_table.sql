-- Add migration script here
-- https://gemini.google.com/share/ff0dff0b4495 about the script and migration
-- Create Subscriptions Table
CREATE TABLE subscriptions(
id uuid NOT NULL,
PRIMARY KEY (id),
email TEXT NOT NULL UNIQUE,
name TEXT NOT NULL,
subscribed_at timestamptz NOT NULL
);