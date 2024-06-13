use reqwest::header;

pub struct Client {
    username: String,
    password: String,
    client: reqwest::Client,
}

impl Client {
    pub fn init(username: &str, password: &str) -> Self {
        Client {
            username: username.to_owned(),
            password: password.to_owned(),
            client: reqwest::Client::new(),
        }
    }
    
    fn custom_header(&self, name: &str, value: &str) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::HeaderName::from_bytes(name.as_bytes()).unwrap(),
            header::HeaderValue::from_bytes(value.as_bytes()).unwrap(),
        );
        headers
    }
}
