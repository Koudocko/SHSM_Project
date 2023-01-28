// @generated automatically by Diesel CLI.

diesel::table! {
    announcements (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        user_id -> Int4,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        date -> Text,
        certification -> Bool,
        completed -> Bool,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hash -> Bytea,
        salt -> Bytea,
        teacher -> Bool,
        code -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    announcements,
    events,
    users,
);
