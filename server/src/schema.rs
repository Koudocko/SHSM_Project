// @generated automatically by Diesel CLI.

diesel::table! {
    j (id) {
        id -> Numeric,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    j,
    test,
    users,
);
