use reqwest::{Client, Method};
use std::env;

async fn connect_server() {
    let args: Vec<String> = env::args().collect();
    println!("args{:?}", args);
    let url = "https://dav.jianguoyun.com/dav/";
    let username = "zhanglun1410@gmail.com";
    let password = args[1].split("=").collect::<Vec<_>>()[1];

    println!("password{:?}", password);

    // 创建HTTP客户端
    let client = Client::new()
        .request(Method::from_bytes(b"PROPFIND").unwrap(), url)
        .basic_auth(username, Some(password));
    // .send()
    // .await;

    println!("client{:?}", client);

    let body = r#"<?xml version="1.0" encoding="utf-8" ?>
    <D:propfind xmlns:D="DAV:">
        <D:allprop/>
    </D:propfind>
"#;

    let result = Client::new()
        .request(Method::from_bytes(b"PROPFIND").unwrap(), url)
        .basic_auth(username, Some(password))
        .body(body)
        .send()
        .await
        .unwrap();

    println!("result ===> {:?}", result.text().await);
    // let res = match result {
    //   Ok(data) => data,
    //   Err(error) => error.to_string(),
    // };
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    connect_server().await;
}
