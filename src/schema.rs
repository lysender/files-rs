// @generated automatically by Diesel CLI.

diesel::table! {
    buckets (id) {
        id -> Nullable<Text>,
        client_id -> Text,
        name -> Text,
        label -> Text,
    }
}

diesel::table! {
    directories (id) {
        id -> Nullable<Text>,
        dir_type -> Text,
        bucket_id -> Text,
        name -> Text,
        label -> Text,
        file_count -> Integer,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::joinable!(directories -> buckets (bucket_id));

diesel::allow_tables_to_appear_in_same_query!(
    buckets,
    directories,
);
