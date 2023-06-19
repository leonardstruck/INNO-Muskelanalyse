// @generated automatically by Diesel CLI.

diesel::table! {
    config (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    micrographs (uuid) {
        uuid -> Text,
        name -> Text,
        import_path -> Text,
        thumbnail_img -> Binary,
        display_img -> Binary,
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
        binary_img -> Binary,
        location_x -> Nullable<Integer>,
        location_y -> Nullable<Integer>,
        height -> Nullable<Integer>,
        width -> Nullable<Integer>,
        measured_length -> Nullable<Float>,
        measured_width -> Nullable<Float>,
        measured_angle -> Nullable<Float>,
        micrograph_id -> Text,
        status -> Text,
        measured_midpoint_x -> Nullable<Float>,
        measured_midpoint_y -> Nullable<Float>,
    }
}

diesel::joinable!(segments -> micrographs (micrograph_id));

diesel::allow_tables_to_appear_in_same_query!(
    config,
    micrographs,
    segments,
);
