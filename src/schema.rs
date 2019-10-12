table! {
    user_login_record (user_id) {
        user_id -> Varchar,
        password_hash -> Varchar,
        status -> Nullable<Varchar>,
    }
}

table! {
    user_records (id) {
        id -> Varchar,
        name -> Varchar,
        display_name -> Varchar,
        role -> Nullable<Varchar>,
    }
}

joinable!(user_login_record -> user_records (user_id));

allow_tables_to_appear_in_same_query!(
    user_login_record,
    user_records,
);
