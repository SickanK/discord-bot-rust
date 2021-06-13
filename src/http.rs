use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use http::request::Request;

struct DiscordHttp {
    rate_limit: Option<DateTime<Utc>>,
    requests_per_second: Arc<Mutex<u16>>,
}

impl DiscordHttp {
    pub fn new() -> Self {
        DiscordHttp {
            rate_limit: None,
            requests_per_second: Arc::new(Mutex::new(0)),
        }
    }

    pub fn request<T>(&self, req: Request<T>) {}

    pub fn new_request(&self) {}
}
