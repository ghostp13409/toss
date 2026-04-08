use reqwest::{Client, Method, Response};
use std::collections::HashMap;

#[derive(Clone)]
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
        params: Vec<(String, String)>,
        body: Option<String>,
    ) -> Result<Response, reqwest::Error> {
        let mut final_url = url.to_string();
        if !params.is_empty() {
            let mut qs = String::new();
            for (k, v) in &params {
                if !qs.is_empty() {
                    qs.push('&');
                }
                // naive escaping, should be fine for now or we could just use url encoding
                // but since urlencoding isn't available, we can just do basic replace
                let ek = k.replace(" ", "%20");
                let ev = v.replace(" ", "%20");
                qs.push_str(&ek);
                qs.push('=');
                qs.push_str(&ev);
            }
            if final_url.contains('?') {
                final_url.push('&');
            } else {
                final_url.push('?');
            }
            final_url.push_str(&qs);
        }

        let mut request = self.client.request(method, &final_url);

        for (key, value) in headers {
            request = request.header(key, value);
        }

        if let Some(b) = body {
            request = request.body(b);
        }

        request.send().await
    }
}
