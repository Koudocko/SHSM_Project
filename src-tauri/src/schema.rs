// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Numeric,
        username -> Varchar,
        password -> Varchar,
    }
}
