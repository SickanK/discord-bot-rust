use std::collections::HashMap;

use futures::executor::block_on;
use http::HeaderMap;
use reqwest::{self, Response};
use tokio::task::{self, JoinHandle};

pub struct DiscordHttp {
    auth_token: String,
}

impl DiscordHttp {
    pub fn new(auth_token: String) -> Self {
        let auth_token = format!("Bot {}", auth_token);
        DiscordHttp { auth_token }
    }

    pub fn request(
        &self,
        uri: String,
        headers: Option<HeaderMap>,
        query: Option<Vec<(&str, &str)>>,
        body: Option<String>,
    ) -> Result<Response, reqwest::Error> {
        let mut auth_header = HeaderMap::new();
        auth_header.insert("Authorization", self.auth_token.parse().unwrap());

        let join: JoinHandle<Result<Response, reqwest::Error>> = task::spawn(async move {
            let client = reqwest::Client::new();

            loop {
                let mut req = client.post(uri.to_string()).headers(auth_header.clone());

                if let Some(ref h) = headers {
                    req = req.headers(h.clone());
                }

                if let Some(ref b) = body {
                    req = req.body::<String>(b.to_string());
                }

                let response = req.send().await;

                if let Err(e) = &response {
                    if e.is_timeout() {
                        continue;
                    }
                }

                return response;
            }
        });

        let result = block_on(join).unwrap();
        result
    }
}
