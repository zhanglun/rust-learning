extern crate app;
extern crate diesel;

use self::app::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use app::schema::channels::dsl::*;

    let connection = establish_connection();
    let results = channels
        .load::<Channel>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}, {}, {}", post.title, post.feed_url, post.link);
        println!("----------\n");
    }
}