// @generated automatically by Diesel CLI.

diesel::table! {
    micrographs (uuid) {
        uuid -> Text,
        name -> Text,
        path -> Nullable<Text>,
        import_path -> Text,
        thumbnail_img -> Nullable<Binary>,
        display_img -> Nullable<Binary>,
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
        binary_img -> Text,
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

diesel::joinable!(segments -> micrographs (micrograph_id));

diesel::allow_tables_to_appear_in_same_query!(
    micrographs,
    segments,
);
