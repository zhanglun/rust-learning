extern crate app;
extern crate diesel;

use self::app::*;

fn main() {
    let connection = establish_connection();
    let channels = parse_opml();

    create_channel(&connection, &channels);

    println!("Channels inserted!");
}
