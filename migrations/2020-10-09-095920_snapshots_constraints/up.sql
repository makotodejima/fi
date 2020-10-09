-- Your SQL goes here
ALTER TABLE snapshots ADD CONSTRAINT snapshot_unique UNIQUE (account_id, date)