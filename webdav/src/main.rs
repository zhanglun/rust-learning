use std::env;

use webdav::client::Client;

async fn connect_server() {
    let args: Vec<String> = env::args().collect();
    println!("args{:?}", args);
    let url = "https://dav.jianguoyun.com/dav/";
    let username = "zhanglun1410@gmail.com";
    let password = args[1].split("=").collect::<Vec<_>>()[1];

    println!("password{:?}", password);

    let client = Client::init(&username, password);

    let list_result = client.list(url, "1").await;
    let list = match list_result {
        Ok(l) => l,
        Err(error) => panic!("Problem creating the file: {error:?}")
    };

    println!("list {:?}", list);
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    connect_server().await;
}
