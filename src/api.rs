use std::{future::Future, pin::Pin, time::Duration};

use ethers::types::H160;
use eyre::Result;
use reqwest::Error;
use serde_derive::Deserialize;

use crate::credentials::twitter_token;

#[derive(Debug, Deserialize)]
pub struct ResponseData {
    twitterUsername: String,
}

#[derive(Deserialize)]
struct Response {
    data: UserData,
}

#[derive(Deserialize)]
struct UserData {
    public_metrics: PublicMetrics,
}

#[derive(Deserialize)]
struct PublicMetrics {
    followers_count: u64,
}

pub fn get_user(address: H160, retry_count: usize) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send>> {
    if retry_count == 0 {
        return Box::pin(async { Err("Request Timeout".to_string()) });
    }

    Box::pin(async move {
        let client = reqwest::Client::new();
        let response = client.get(format!("https://prod-api.kosetto.com/users/{:#?}", address))
            .timeout(Duration::from_millis(200))
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

pub async fn get_user_followers(user_id: &str) -> Result<u64, Error> {
    let client = reqwest::Client::new();
    let url = format!("https://api.twitter.com/2/users/by/username/{}?user.fields=public_metrics", user_id);

    let response = client.get(&url)
        .bearer_auth(twitter_token())
        .send()
        .await?;

    let response_data: Response = response.json().await?;

    Ok(response_data.data.public_metrics.followers_count)
}

#[cfg(test)]
mod test_case {
    use dotenv::dotenv;

    use super::*;

    #[tokio::test]
    async fn my_stats() {
        dotenv().ok();

        let result = get_user("0xBb3B8c342556De95CFF4676D0f864235Eca1128D".parse().unwrap(), 2).await;

        assert_eq!(result.clone().unwrap(), "ThePepeology");

        let result = get_user_followers(&result.unwrap()).await;

        assert_ne!(result.unwrap(), 0);
    }
}