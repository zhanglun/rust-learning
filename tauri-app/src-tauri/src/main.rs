#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate app;
extern crate diesel;

use self::app::*;
use self::models::*;
use self::diesel::prelude::*;

use reqwest;
use rss::*;
use std::error::Error;

#[tauri::command]
async fn fetch_feed(url: String) -> String {
  let res = fetch_rss_item(&url).await;
  let res = match res {
    Ok(data) => data,
    Err(error) => error.to_string(),
  };

  res
}

fn load_channels_from_db () -> Result<String, Box<dyn Error>> {
  use app::schema::channels::dsl::*;

  let connection = establish_connection();
  let results = channels
    .load::<models::Channel>(&connection)
    .expect("Error loading posts");
  let results = serde_json::to_string(&results)?;

  Ok(results)
}

#[tauri::command]
fn load_channels() -> String {
  let res = load_channels_from_db();
  let res = match res {
    Ok(data) => data,
    Err(error) => error.to_string(),
  };

  res
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      fetch_feed,
      load_channels,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
