use std::error::Error;

use ethers::types::H160;
use eyre::Result;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResponseData {
    twitterUsername: String,
}

pub async fn get_user(
    address: H160
) -> Result<String, Box<dyn Error>> {
    // Send the request
    let client = reqwest::Client::new();
    let response = client.get(format!("https://prod-api.kosetto.com/users/{:#?}", address))
        .send()
        .await?;

    // Deserialize the response
    let data: ResponseData = response.json().await?;
    println!("Twitter: @{}", data.twitterUsername);
    Ok(data.twitterUsername)
}

#[cfg(test)]
mod test_case {
    use super::*;

    #[tokio::test]
    async fn my_stats() {
        let result = get_user("0xBb3B8c342556De95CFF4676D0f864235Eca1128D".parse().unwrap()).await;

        assert_eq!(result.unwrap(), "ThePepeology");
    }
}