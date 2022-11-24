pub mod endpoint;
pub mod client;
pub mod response;

#[cfg(test)]
mod tests {
    
    #[tokio::test]
    async fn information() {
        use crate::client::TebexClient;
        
        let client = TebexClient::default();

        let information = client.get_information().await;
        println!("{:?}", information);
    }
}