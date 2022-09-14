pub mod endpoint;
pub mod client;
pub mod response;

mod tests {
    use std::env;

    use crate::client::TebexClient;

    #[tokio::test]
    async fn information() {
        let secret = env::var("TEBEX_SECRET").unwrap();
        let client = TebexClient::new(secret.as_str());

        let information = client.get_information().await;
        println!("{:?}", information);
    }
}