// @generated automatically by Diesel CLI.

diesel::table! {
    bin (id) {
        id -> Int4,
        title -> Text,
        latitude -> Numeric,
        longitude -> Numeric,
        status -> Nullable<Numeric>,
    }
}
