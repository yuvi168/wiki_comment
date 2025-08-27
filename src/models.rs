use crate::schema::comments;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub reply: Option<String>,
    pub upvotes: i32,
    pub got_replies: i32,
    pub userid: String,
    pub wiki_entry: String,
}

#[derive(Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub content: String,
    pub reply: Option<String>,
    pub userid: String,
    pub wiki_entry: String, 
}

#[derive(Deserialize)]
pub struct UpdateComment {
    pub id: i32,
    pub content: Option<String>,
    pub reply: Option<String>,
    pub upvotes: Option<i32>,
    pub got_replies: Option<i32>,
    pub userid: Option<String>,
    pub wiki_entry: Option<String>,
}

