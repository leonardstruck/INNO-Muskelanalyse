// @generated automatically by Diesel CLI.

diesel::table! {
    case_micrographs (id) {
        id -> Integer,
        case_id -> Integer,
        micrograph_id -> Text,
    }
}

diesel::table! {
    cases (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    micrographs (uuid) {
        uuid -> Text,
        name -> Text,
        path -> Nullable<Text>,
        import_path -> Text,
        thumbnail_path -> Nullable<Text>,
        file_size -> Nullable<Integer>,
        file_type -> Nullable<Text>,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(case_micrographs -> cases (case_id));
diesel::joinable!(case_micrographs -> micrographs (micrograph_id));

diesel::allow_tables_to_appear_in_same_query!(
    case_micrographs,
    cases,
    micrographs,
);
