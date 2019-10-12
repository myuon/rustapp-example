-- Your SQL goes here
ALTER TABLE user_records MODIFY COLUMN name VARCHAR(64) NOT NULL;
ALTER TABLE user_records MODIFY COLUMN display_name VARCHAR(128) NOT NULL;
