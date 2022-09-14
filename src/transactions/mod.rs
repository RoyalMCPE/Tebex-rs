use std::collections::HashMap;

use reqwest::Client;

use crate::{endpoint::Endpoint, client::TebexClient};

use self::transaction::Transaction;

pub mod player;
pub mod transaction;

pub struct Transactions {
    client: Client,
    online: HashMap<String, Vec<Transaction>>
}

impl Transactions {

    pub fn new() -> Self {
        Self {
            client: Client::new(),
            online: HashMap::new()
        }
    }

    pub async fn get_online_transactions(&mut self) {
        let due = self.get_due_transactions().await;

        // TODO: Check the online hash for and already cached transactions
        for p in &due {
            let transactions = TebexClient::create_request(&self.client, "", Endpoint::OnlineCommands(p.id()))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            println!("Raw: {:?}", transactions);

            let t = serde_json::from_value::<Vec<Transaction>>(transactions["commands"].clone()).unwrap();
            self.online.insert(p.uuid().to_string(), t);
        }

        // println!("{:?}", self.online);
    }

    pub fn get_offline_transactions(&self) {}

    async fn get_due_transactions<'str>(&self) -> Vec<player::Player> {
        let resp = TebexClient::create_request(&self.client, "", Endpoint::DueTransactions)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();

        serde_json::from_value(resp["players"].clone()).unwrap()
    }
}