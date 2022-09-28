// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        age -> Int2,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
