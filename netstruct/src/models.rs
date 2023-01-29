use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::*;

#[derive(Identifiable, Queryable)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub teacher: bool,
    pub code: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub teacher: bool,
    pub code: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(User))]
pub struct Announcement{
    pub id: i32,
    pub title: String,
    pub description: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = announcements)]
pub struct NewAnnouncement {
    pub title: String,
    pub description: String,
    pub user_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(User))]
pub struct Event{
    pub id: i32,
    pub title: String,
    pub description: String,
    pub date: String,
    pub certification: bool,
    pub completed: bool,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub title: String,
    pub description: String,
    pub date: String,
    pub certification: bool,
    pub completed: bool,
    pub user_id: i32,
}