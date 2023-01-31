// @generated automatically by Diesel CLI.

diesel::table! {
    announcements (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
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
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        teacher -> Bool,
        hash -> Bytea,
        salt -> Bytea,
    }
}

diesel::joinable!(events -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    announcements,
    events,
    users,
);
