use diesel::prelude::*;
use diesel::sql_types::*;

#[derive(Queryable)]
pub struct User{
    pub id: Serial,
    pub username: Text,
    pub teacher: Bool,
    pub hash: Bytea,
    pub salt: Bytea,
}

#[derive(Queryable)]
pub struct Announcement{
    pub id: Serial,
    pub title: Text,
    pub description: Text,
}

#[derive(Queryable)]
pub struct Event{
    pub id: Serial,
    pub title: Text,
    pub description: Text,
    pub date: Text,
    pub certification: Bool,
    pub completed: Bool,
    pub user_id: Option<Integer>,
}
