extern crate app;
extern crate diesel;

use self::diesel::prelude::*;
use self::app::*;
use self::models::Post;
use std::env::args;

fn main() {
    use diesel_demo::schema::posts::dsl::{posts, published};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let connection = &mut establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(connection)
        .expect(&format!("Unable to find post {}", id));

    let post: models::Post = posts
        .find(id)
        .first(connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}