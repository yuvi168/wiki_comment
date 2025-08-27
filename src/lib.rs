pub mod models;
pub mod schema;

use self::models::{NewComment, UpdateComment, Comment};
use self::schema::comments::dsl::{
    comments,
    content,
    reply,
    upvotes,
    got_replies,
    userid,
    wiki_entry,
};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get(queried_wiki_entry: String) -> Vec<Comment> {
    comments
        .filter(wiki_entry.eq(&queried_wiki_entry))
        .select(Comment::as_select())
        .load(&mut establish_connection())
        .expect("Erro loading comments")
}

pub fn create(comment: NewComment) {
    use crate::schema::comments;
    diesel::insert_into(comments::table)
        .values(&comment)
        .execute(&mut establish_connection())
        .expect("Error saving new comment");
}

pub fn update(comment: UpdateComment) {
    if let Some(com_content) = comment.content {
        diesel::update(comments.find(comment.id))
            .set(content.eq(&com_content))
            .execute(&mut establish_connection())
            .expect("Could update the content of the comment.");
    };
    if let Some(com_reply) = comment.reply {
        diesel::update(comments.find(comment.id))
            .set(reply.eq(&com_reply))
            .execute(&mut establish_connection())
            .expect("Could update the reply of the comment.");
    };
    if let Some(com_upvotes) = comment.upvotes {
        diesel::update(comments.find(comment.id))
            .set(upvotes.eq(&com_upvotes))
            .execute(&mut establish_connection())
            .expect("Could update the upvotes of the comment.");
    };
    if let Some(com_got_replies) = comment.got_replies {
        diesel::update(comments.find(comment.id))
            .set(got_replies.eq(&com_got_replies))
            .execute(&mut establish_connection())
            .expect("Could update the got_replies of the comment.");
    };
    if let Some(com_userid) = comment.userid {
        diesel::update(comments.find(comment.id))
            .set(userid.eq(&com_userid))
            .execute(&mut establish_connection())
            .expect("Could update the userid of the comment.");
    };
    if let Some(com_wiki_entry) = comment.wiki_entry {
        diesel::update(comments.find(comment.id))
            .set(wiki_entry.eq(&com_wiki_entry))
            .execute(&mut establish_connection())
            .expect("Could update the wiki_entry of the comment.");
    };
}


