#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use reqwest;
use rss::Channel;
use std::error::Error;
use serde::{Serialize, Deserialize};
use feed_rs::parser;

#[derive(Serialize, Deserialize)]
struct MyChannel {
  title: String,
}

#[tauri::command]
fn my_custom_command1() {
  println!("I was invoked from JS!");
}

async fn request(url: &str) -> Result<feed_rs::model::Feed, Box<dyn Error>> {
  let content = reqwest::get(url).await?.text().await?;
  let channel = parser::parse(content.as_bytes()).unwrap();
  // let channel = serde_json::to_string(&channel)?;

  Ok(channel)
}

#[tauri::command]
async fn fetch_feed(url: String) -> String {
  // let result = request(&url).await;
  // let result = match result {
  //   Ok(res) => res,
  //   Err(e) => return Err("asdf"),
  // };

  // result
  return "asdf".to_string();
}

#[tauri::command]
async fn my_custom_command2(obj: String) {
  println!("{:?}", obj);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      fetch_feed,
      my_custom_command1,
      my_custom_command2,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
