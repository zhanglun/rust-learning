#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use reqwest;
use rss::Channel;
use std::error::Error;

#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}

async fn request(url: &str) -> Result<String, Box<dyn Error>> {
  let content = reqwest::get(url).await?.text().await?;
  Ok(content.to_string())
}

async fn request2(url: &str) -> Result<Channel, Box<dyn Error>> {
  let content = reqwest::get(url).await?.bytes().await?;
  let channel = Channel::read_from(&content[..])?;
  Ok(channel)
}

#[tauri::command]
async fn fetch_feed1(url: String) -> String {
  let body = request(&url).await;
  let body = match body {
    Ok(res) => {
      return res.to_string();
    },
    Err(e) => {
      panic!("====>{:?}", e)
    }
  };
}

#[tauri::command]
async fn fetch_feed2(url: String) -> Channel {
  let body = request2(&url).await;
  let body = match body {
    Ok(res) => {
      println!("{:?}", res.title);
      return res;
    }
    Err(e) => {
      panic!("====>{:?}", e)
    }
  };
}

fn main() {
  print!("asdfasdf ===>");
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![fetch_feed1, fetch_feed2, my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
