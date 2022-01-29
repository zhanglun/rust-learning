extern crate app;
extern crate diesel;

use self::app::*;

fn main() {
    let connection = establish_connection();
    let channels = parse_opml();

    create_channel(&connection, &channels);

    println!("Channels inserted!");
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
