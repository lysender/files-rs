// @generated automatically by Diesel CLI.

diesel::table! {
    buckets (id) {
        id -> Nullable<Text>,
        client_id -> Text,
        name -> Text,
        label -> Text,
    }
}
