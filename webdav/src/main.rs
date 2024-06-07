use reqwest::{Client, Method};
use std::env;

async fn connect_server () {
  let args: Vec<String> = env::args().collect();
  println!("args{:?}", args);
  let url = "https://dav.jianguoyun.com/dav/";
  let username = "zhanglun1410@gmail.com";
  let password = args[1].split("=").collect::<Vec<_>>()[1];

  println!("password{:?}", password);

  // 创建HTTP客户端
  let client = Client::new();

  // 发送身份验证请求
  let response = client
      .request(Method::GET, url)
      .basic_auth(username, Some(password))
      .send()
      .await;

    println!("response{:?}", response);

  // 检查身份验证是否成功
  match response {
    Ok(response) => {
        println!("response.status() {:?}", response.status());
        
        if response.status().is_success() {
            println!("Authentication successful!");

            // 上传文件
            // match upload_file(&client, url, "example.txt").await {
            //     Ok(_) => println!("File uploaded successfully!"),
            //     Err(err) => eprintln!("Failed to upload file: {:?}", err),
            // }

            // // 下载文件
            // match download_file(&client, url, "example.txt", "downloaded.txt").await {
            //     Ok(_) => println!("File downloaded successfully!"),
            //     Err(err) => eprintln!("Failed to download file: {:?}", err),
            // }

            // // 删除文件
            // match delete_file(&client, url, "example.txt").await {
            //     Ok(_) => println!("File deleted successfully!"),
            //     Err(err) => eprintln!("Failed to delete file: {:?}", err),
            // }
        } else {
            println!("Authentication failed!");
        }
    }
    Err(err) => eprintln!("Failed to send authentication request: {:?}", err),
  }
}

#[tokio::main]
async fn main() {
  println!("Hello, world!");
  connect_server().await;
}
