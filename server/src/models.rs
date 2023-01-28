use diesel::prelude::*;
use diesel::sql_types::*;

#[derive(Queryable)]
pub struct User{
    pub id: Serial,
    pub username: Text,
    pub hash: Bytea,
    pub salt: Bytea,
    pub teacher: Bool,
    pub code: Option<Text>,
}

#[derive(Queryable)]
pub struct Announcement{
    pub id: Serial,
    pub title: Text,
    pub description: Text,
    pub user_id: Integer,
}

#[derive(Queryable)]
pub struct Event{
    pub id: Serial,
    pub title: Text,
    pub description: Text,
    pub date: Text,
    pub certification: Bool,
    pub completed: Bool,
    pub user_id: Integer,
}
