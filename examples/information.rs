use std::env;
use tebex_rs::client::TebexClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = env::var("TEBEX_SECRET").unwrap();
    let mut client = TebexClient::new(&secret[..]);

    let information = client.get_information().await?;
    let account = information.get_account();

    println!("Account Name: {}", account.name);
    println!("Store Domain: {}", account.domain);
    println!("Game Type: {:?}", account.game_type);
    
    Ok(())
}