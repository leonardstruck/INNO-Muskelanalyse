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
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    segments (uuid) {
        uuid -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        path -> Text,
        location_x -> Nullable<Integer>,
        location_y -> Nullable<Integer>,
        height -> Nullable<Integer>,
        width -> Nullable<Integer>,
        measured_length -> Nullable<Float>,
        measured_width -> Nullable<Float>,
        measured_angle -> Nullable<Float>,
        micrograph_id -> Text,
        status -> Text,
    }
}

diesel::joinable!(case_micrographs -> cases (case_id));
diesel::joinable!(case_micrographs -> micrographs (micrograph_id));
diesel::joinable!(segments -> micrographs (micrograph_id));

diesel::allow_tables_to_appear_in_same_query!(
    case_micrographs,
    cases,
    micrographs,
    segments,
);
