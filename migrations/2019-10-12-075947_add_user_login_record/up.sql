-- Your SQL goes here
create table user_login_record (
  user_id varchar(64) primary key,
  password_hash varchar(60) not null,
  status varchar(32),
  foreign key (user_id) references user_records (id) on delete cascade on update restrict
);

