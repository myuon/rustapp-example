-- Your SQL goes here
create table users(
  id varchar(36) primary key,
  name varchar(32) unique,
  display_name varchar(128)
);
