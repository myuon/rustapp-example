-- Your SQL goes here
create table user_records
(
  id varchar(36) primary key,
  name varchar(32) unique,
  display_name varchar(128)
);
