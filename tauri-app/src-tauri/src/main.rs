#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate app;

use self::app::*;
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

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      fetch_feed,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri Application");
}
