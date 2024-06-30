// @generated automatically by Diesel CLI.

diesel::table! {
    buckets (id) {
        id -> Text,
        client_id -> Text,
        name -> Text,
        created_at -> BigInt,
    }
}

diesel::table! {
    clients (id) {
        id -> Text,
        name -> Text,
        status -> Text,
        created_at -> BigInt,
    }
}

diesel::table! {
    dirs (id) {
        id -> Text,
        bucket_id -> Text,
        name -> Text,
        label -> Text,
        file_count -> Integer,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

diesel::joinable!(buckets -> clients (client_id));
diesel::joinable!(dirs -> buckets (bucket_id));

diesel::allow_tables_to_appear_in_same_query!(
    buckets,
    clients,
    dirs,
);
