#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use opml::OPML;
use std::{env, fs, path};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::*;

pub fn create_post<'a>(conn: &SqliteConnection, title: &'a str, body: &'a str) -> usize {
    use schema::posts;

    let new_post = models::NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}

pub fn parse_opml() -> Vec<NewChannel> {
    let file = path::Path::new("./src/feedly.opml");
    let ctx = fs::read_to_string(file).expect("!!!!");
    let doc = OPML::from_str(&ctx).unwrap();
    let outlines = doc.body.outlines;
    let mut list: Vec<NewChannel> = Vec::new();

    for outline in outlines {
        match outline.xml_url {
            None => (),
            Some(url) => {
                let c = NewChannel {
                    title: outline.title.unwrap_or("".to_string()).as_str(),
                    name: outline.text.as_str(),
                    url: url.as_str(),
                    description: "",
                };

                list.push(c);
            }
        }
    }

    list
}

pub fn create_channel<'a>(conn: &SqliteConnection, list: Vec<Channel>) {
    use schema::channels;

    for channel in list {
        let new_channel = models::NewChannel {
            description: "this is description",
            title: channel.title.as_str(),
            name: channel.name.as_str(),
            url: channel.url.as_str(),
        };

        diesel::insert_into(channels::table)
            .values(&new_channel)
            .execute(conn)
            .expect("Error saving new channel");
    }
}
