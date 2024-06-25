// @generated automatically by Diesel CLI.

diesel::table! {
    buckets (id) {
        id -> Text,
        client_id -> Text,
        name -> Text,
        label -> Text,
    }
}

diesel::table! {
    directories (id) {
        id -> Text,
        dir_type -> Text,
        bucket_id -> Text,
        name -> Text,
        label -> Text,
        file_count -> Integer,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

diesel::joinable!(directories -> buckets (bucket_id));

diesel::allow_tables_to_appear_in_same_query!(buckets, directories,);
