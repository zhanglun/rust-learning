use reqwest::{header, Error, Method, RequestBuilder, Response, Url};

use crate::parse_xml;

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

    fn start_request(&self, method: Method, path: &str) -> RequestBuilder {
        self.client
            .request(method, Url::parse(path).unwrap())
            .basic_auth(self.username.as_str(), Some(self.password.as_str()))
    }

    pub async fn list(
        &self,
        path: &str,
        depth: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        //   let body = r#"<?xml version="1.0" encoding="utf-8" ?>
        //     <D:propfind xmlns:D="DAV:">
        //         <D:allprop/>
        //     </D:propfind>
        // "#;

        //   let response = self
        //       .start_request(Method::from_bytes(b"PROPFIND").unwrap(), path)
        //       .headers(self.custom_header("depth", depth))
        //       .body(body)
        //       .send()
        //       .await?
        //       .text()
        //       .await?;
        //   // let text = response.text().await?;

        //   println!("response{:?}", response);

        let response = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<d:multistatus
	xmlns:d="DAV:"
	xmlns:s="http://ns.jianguoyun.com">
	<d:response>
		<d:href>/dav/</d:href>
		<d:propstat>
			<d:prop>
				<d:getcontenttype>httpd/unix-directory</d:getcontenttype>
				<d:displayname>dav</d:displayname>
				<d:owner>zhanglun1410@gmail.com</d:owner>
				<d:resourcetype>
					<d:collection/>
				</d:resourcetype>
				<d:getcontentlength>0</d:getcontentlength>
				<d:getlastmodified>Fri, 14 Jun 2024 07:28:33 GMT</d:getlastmodified>
				<d:current-user-privilege-set>
					<d:privilege>
						<d:read/>
					</d:privilege>
				</d:current-user-privilege-set>
			</d:prop>
			<d:status>HTTP/1.1 200 OK</d:status>
		</d:propstat>
	</d:response>
	<d:response>
		<d:href>/dav/%e6%88%91%e7%9a%84%e5%9d%9a%e6%9e%9c%e4%ba%91/</d:href>
		<d:propstat>
			<d:prop>
				<d:getcontenttype>httpd/unix-directory</d:getcontenttype>
				<d:displayname>我的坚果云</d:displayname>
				<d:owner>zhanglun1410@gmail.com</d:owner>
				<d:resourcetype>
					<d:collection/>
				</d:resourcetype>
				<d:getcontentlength>0</d:getcontentlength>
				<d:getlastmodified>Fri, 14 Jun 2024 07:28:33 GMT</d:getlastmodified>
				<d:current-user-privilege-set>
					<d:privilege>
						<d:read/>
					</d:privilege>
					<d:privilege>
						<d:write/>
					</d:privilege>
					<d:privilege>
						<d:all/>
					</d:privilege>
					<d:privilege>
						<d:read_acl/>
					</d:privilege>
					<d:privilege>
						<d:write_acl/>
					</d:privilege>
				</d:current-user-privilege-set>
			</d:prop>
			<d:status>HTTP/1.1 200 OK</d:status>
		</d:propstat>
	</d:response>
</d:multistatus>"#;

        parse_xml(&response).unwrap();

        Ok(response.to_string())
    }
}
