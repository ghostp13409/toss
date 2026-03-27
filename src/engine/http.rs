use reqwest::{Client, Method, Response};
use std::collections::HashMap;

pub struct RequestEngine {
    client: Client,
}

impl RequestEngine {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn send(
        &self,
        method: Method,
        url: &str,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Result<Response, reqwest::Error> {
        let mut request = self.client.request(method, url);

        for (key, value) in headers {
            request = request.header(key, value);
        }

        if let Some(b) = body {
            request = request.body(b);
        }

        request.send().await
    }
}
