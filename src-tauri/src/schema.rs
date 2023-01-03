// @generated automatically by Diesel CLI.

diesel::table! {
    cases (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
