// @generated automatically by Diesel CLI.

diesel::table! {
    test (id) {
        id -> Numeric,
    }
}

diesel::table! {
    users (id) {
        id -> Numeric,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    x (id) {
        id -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    test,
    users,
    x,
);
