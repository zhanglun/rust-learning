use serde::{Serialize, Deserialize};
use super::schema::channels;

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct Channel {
  pub id: i32,
  pub title: String,
  pub description: String,
  pub feed_url: String,
  pub link: String,
  pub ttl: i32,
  pub favicon: String,
  pub category: String,
  pub tag: String,
  pub created_date: String,
  pub update_date: String,
  pub last_sync_date: String,
}

#[derive(Insertable)]
#[derive(Debug, Serialize, Deserialize)]
#[table_name = "channels"]
pub struct NewChannel {
  pub title: String,
  pub description: String,
  pub feed_url: String,
  pub link: String,
}