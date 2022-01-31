#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::*;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use opml::OPML;
use std::{env, fs, path};
use std::error::Error;

pub fn establish_connection() -> SqliteConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  SqliteConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn parse_opml() -> Vec<models::NewChannel> {
  let file = path::Path::new("./src/feedly.opml");
  let ctx = fs::read_to_string(file).expect("!!!!");
  let doc = OPML::from_str(&ctx).unwrap();
  let outlines = doc.body.outlines;
  let mut list: Vec<NewChannel> = Vec::new();

  for outline in outlines {
    match outline.xml_url {
      None => (),
      Some(xml_url) => {
        // let title = outline.title.unwrap_or("".to_string());
        let link = outline.html_url.unwrap_or("".to_string());
        let title = outline.text;

        let c = models::NewChannel {
          title,
          link,
          feed_url: xml_url,
          description: "asdf".to_string(),
        };

        list.push(c);
      }
    }
  }

  list
}

pub fn create_channel(conn: &SqliteConnection, list: &Vec<NewChannel>) {
  use schema::channels;

  for channel in list {
    diesel::insert_or_ignore_into(channels::table)
      .values(channel)
      .execute(conn)
      .expect("Error saving new channel");
  }
}

pub async fn fetch_rss_item(url: &str) -> Result<String, Box<dyn Error>> {
  let content = reqwest::get(url).await?.bytes().await?;
  let channel = rss::Channel::read_from(&content[..])?;
  let items = serde_json::to_string(&channel.items)?;

  Ok(items)
}