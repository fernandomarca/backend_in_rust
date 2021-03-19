extern crate diesel;

use crate::database::establish_connection;
use crate::models::posts::{NewPost, Post};
use crate::schema::posts;
use diesel::prelude::*;

pub async fn create_post<'a>(title: &'a str, body: &'a str) -> Post {
    //use schema::posts;
    let data_connect = establish_connection().await;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    let result = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&data_connect)
        .expect("Error saving new post");
    result
}
