pub mod endpoint;
pub mod client;
pub mod transactions;
pub mod game_server;

mod tests {

    #[tokio::test]
    async fn information() -> Result<(), anyhow::Error>{
        use std::env;
        use crate::client::TebexClient;

        let secret = env::var("TEBEX_SECRET").unwrap();
        let mut client = TebexClient::new(secret.as_str());

        let information = client.get_information().await.unwrap();
        println!("{:?}", information);
        Ok(())
    }

    #[tokio::test]
    async fn online_transactions() {
        use std::env;
        use crate::client::TebexClient;

        let secret = env::var("TEBEX_SECRET").unwrap();
        let client = TebexClient::new(secret.as_str());

        let mut transactions = client.get_transactions();

        transactions.get_online_transactions().await;

    }
}