pub mod endpoint;
pub mod client;
pub mod game_server;

pub(crate) mod helper;


mod tests {

    #[tokio::test]
    async fn information() -> Result<(), Box<dyn std::error::Error>>{
        use std::env;
        use crate::client::TebexClient;

        let secret = env::var("TEBEX_SECRET").unwrap();
        let mut client = TebexClient::new(secret.as_str());

        let information = client.get_information().await?;
        println!("{:?}", information);

        Ok(())
    }
}