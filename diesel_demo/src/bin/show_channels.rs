extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_demo::schema::channels::dsl::*;

    let connection = establish_connection();
    let results = channels
        .load::<Channel>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}, {}, {}", post.title, post.name, post.url);
        println!("----------\n");
    }
}