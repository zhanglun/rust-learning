use bindings:: {
    Windows::Foundation::Uri,
    Windows::Web::Syndication::SyndicationClient,
    Windows::Win32::UI::WindowsAndMessaging::*,
};

fn main() -> windows::Result<()> {
    println!("Hello, world!");
    
    let uri = Uri::CreateUri("https://post.smzdm.com/feed")?;
    let client = SyndicationClient::new()?;
    let feed = client.RetrieveFeedAsync(uri)?.get()?;
    let mut text = String::from("");

    for item in feed.Items()? {
        println!("{}", item.Title()?.Text()?);
        let s = item.Title()?.Text()?.to_string();
        text.push_str(&s);
        text.push_str("\n");
    }

    println!("标题 你好，微软");

    unsafe {
        MessageBoxA(None, "标题", "你好，微软", MB_OK);
    }

    Ok(())
}