use anyhow::{Ok, anyhow};
use reqwest::Client;

use crate::{endpoint::Endpoint, game_server, helper};

pub struct TebexClient<'str> {
    secret: &'str str,
    client: Client,

    server_info: Option<game_server::GameServer>
}

impl<'str> TebexClient<'str> {
    pub fn new(secret: &'str str) -> Self {
        Self {
            secret,
            client: Client::new(),
            server_info: None
        }
    }

    pub async fn get_information(&mut self) -> Result<&game_server::GameServer, anyhow::Error> {
        if self.server_info.is_none() {
            let info = helper::create_header(&self.client, self.secret, Endpoint::Information)
                .send()
                .await?
                .json::<game_server::GameServer>()
                .await?;

            self.server_info = Some(info)
        }
        
        Ok(self.server_info.as_ref().ok_or(anyhow!("No server information has been recieved!"))?)
    }
}