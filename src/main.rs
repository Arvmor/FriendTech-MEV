use dotenv::dotenv;
use eyre::Result;
use ethers::{providers::{Http, Provider,Ws, Middleware, StreamExt}, types::{BlockNumber, H160, Filter}};

mod credentials;
use credentials::*;

#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();

    // Establishing Connections to WS & HTTP providers
    let provider: Provider<Http> = Provider::<Http>::try_from(http_provider_url())?;
    // To fetch pending transactions
    let ws_provider: Provider<Ws> = Provider::<Ws>::connect(ws_provider_url()).await?;
    // Getting pending transactions from mempool
    let mut pending_transactions_stream = ws_provider.subscribe_logs(&Filter::new().select(BlockNumber::Latest).event("Trade(address,address,bool,uint256,uint256,uint256,uint256,uint256)")).await?;

    let result = provider.get_balance(my_address().parse::<H160>().unwrap(), Some(BlockNumber::Latest.into())).await;
    println!("Bal {:#?}", result);
    loop {
        tokio::select! {
            Some(pending_transaction) = pending_transactions_stream.next() => {

                println!("PT {:#?}", pending_transaction)

            },

            else => {
                eprintln!("{}", "[X] Failed To Fetch The Pending Transaction Or Block!");
                break;
            }
        }
    }

    Ok(())
}
