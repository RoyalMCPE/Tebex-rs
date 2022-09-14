use crate::endpoint;

const HEADER_KEY: &'static str = "X-Tebex-Secret";

pub fn create_header(client: &reqwest::Client, secret: &str, endpoint: endpoint::Endpoint) -> reqwest::RequestBuilder{
    client.get(endpoint.to_string())
        .header(HEADER_KEY, secret)
}