use std::{future::Future, pin::Pin};

use ethers::types::H160;
use eyre::Result;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResponseData {
    twitterUsername: String,
}

pub fn get_user(address: H160, retry_count: usize) -> Pin<Box<dyn Future<Output = Result<String, String>>>> {
    if retry_count == 0 {
        return Box::pin(async { Err("Request Timeout".to_string()) });
    }

    Box::pin(async move {
        let client = reqwest::Client::new();
        let response = client.get(format!("https://prod-api.kosetto.com/users/{:#?}", address))
            .send()
            .await;

        match response {
            Ok(resp) => match resp.json::<ResponseData>().await {
                Ok(data) => Ok(data.twitterUsername),
                Err(_) => get_user(address, retry_count - 1).await,
            },
            Err(_) => get_user(address, retry_count - 1).await,
        }
    })
}

#[cfg(test)]
mod test_case {
    use super::*;

    #[tokio::test]
    async fn my_stats() {
        let result = get_user("0xBb3B8c342556De95CFF4676D0f864235Eca1128D".parse().unwrap(), 2).await;

        assert_eq!(result.unwrap(), "ThePepeology");
    }
}