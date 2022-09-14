use reqwest::Client;

use crate::{endpoint::Endpoint, response::{self, information::GameServerInformationResponse}};

const HEADER_KEY: &'static str = "X-Tebex-Secret";

pub struct TebexClient<'str> {
    secret: &'str str,
    client: Client
}

impl<'str> TebexClient<'str> {
    pub fn new(secret: &'str str) -> Self {
        Self {
            secret,
            client: Client::new()
        }
    }

    pub async fn get_information(&self) -> GameServerInformationResponse {
        // TODO: Cache this response as it's unlikely to change very often but the information may be useful
        self.client
            .get(Endpoint::Information.to_string())
            .header(HEADER_KEY, self.secret)
            .send()
            .await
            .unwrap()
            .json::<response::information::GameServerInformationResponse>()
            .await
            .unwrap()
    }
}