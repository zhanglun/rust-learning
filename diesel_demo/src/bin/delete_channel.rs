extern crate diesel_demo;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;

fn main() {
    use diesel_demo::schema::channels::dsl::*;

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(channels)
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} ", num_deleted);
}