use std::env;
use tebex_rs::client::TebexClient;

#[tokio::main]
async fn main() {
    let secret = env::var("TEBEX_SECRET").unwrap();
    let client = TebexClient::new(&secret[..]);

    let information = client.get_information().await;
    println!("Account Name: {}", information.account.name);
    println!("Store Domain: {}", information.account.domain);
    println!("Game Type: {:?}", information.account.game_type);
}