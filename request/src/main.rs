#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://feed.appinn.com")
        .await?;
    println!("{:#?}", resp);
    Ok(())
}