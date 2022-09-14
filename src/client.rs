use anyhow::anyhow;
use reqwest::Client;

use crate::{endpoint::Endpoint, transactions::{Transactions, self}, game_server::GameServer};

pub(crate) const HEADER_KEY: &'static str = "X-Tebex-Secret";

pub struct TebexClient<'str> {
    secret: &'str str,
    client: Client,

    game_server: Option<GameServer>
}

impl<'str> TebexClient<'str> {
    pub fn new(secret: &'str str) -> Self {
        Self {
            secret,
            client: Client::new(),
            game_server: None
        }
    }

    pub async fn get_information(&mut self) -> Result<&GameServer, Box<dyn std::error::Error>> {
        if let None = self.game_server {
            let info = Self::create_request(&self.client, self.secret, Endpoint::Information)
                .send()
                .await?
                .json::<GameServer>()
                .await?;

            self.game_server = Some(info)
        }

        Ok(self.game_server.as_ref().ok_or(anyhow!("No gameserver information has been recieved!"))?)
    }

    pub fn get_transactions(&self) -> Transactions {
        transactions::Transactions::new()
    }

    // this is just a usef
    pub(crate) fn create_request(client: &Client, secret: &str, endpoint: Endpoint) -> reqwest::RequestBuilder {
        client
            .get(endpoint.to_string())
            .header(HEADER_KEY, secret)
    }
}