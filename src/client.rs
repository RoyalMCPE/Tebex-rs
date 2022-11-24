use std::env;

use reqwest::Client;

use crate::{endpoint::Endpoint, response::{self, information::GameServerInformationResponse}};

const HEADER_KEY: &'static str = "X-Tebex-Secret";

pub struct TebexClient {
    secret: String,
    client: Client
}

impl TebexClient {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            client: Client::new()
        }
    }

    pub async fn get_information(&self) -> GameServerInformationResponse {
        // TODO: Cache this response as it's unlikely to change very often but the information may be useful
        self.client
            .get(Endpoint::Information.to_string())
            .header(HEADER_KEY, &self.secret)
            .send()
            .await
            .unwrap()
            .json::<response::information::GameServerInformationResponse>()
            .await
            .unwrap()
    }
}

impl Default for TebexClient {
    fn default() -> Self {
        let secret = env::var("TEBEX_SECRET").unwrap();

        Self::new(secret)
    }
}