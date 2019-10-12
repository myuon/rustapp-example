use diesel::*;

table! {
  user_records {
    id -> VarChar,
    name -> VarChar,
    display_name -> VarChar,
  }
}
