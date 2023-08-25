use dotenv::dotenv;
use eyre::Result;
use ethers::{providers::{Http, Provider,Ws, Middleware, StreamExt}, types::{BlockNumber, H160, Filter, U256, U64}};

mod credentials;
use credentials::*;
mod decoder;
use decoder::*;
mod builder;
use builder::*;

#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();

    // Establishing Connections to WS & HTTP providers
    let provider: Provider<Http> = Provider::<Http>::try_from(http_provider_url())?;
    // To fetch pending transactions
    let ws_provider: Provider<Ws> = Provider::<Ws>::connect(ws_provider_url()).await?;
    // Getting pending transactions from mempool
    let mut pending_transactions_stream = ws_provider.subscribe_logs(&Filter::new().select(BlockNumber::Latest).event("Trade(address,address,bool,uint256,uint256,uint256,uint256,uint256)")).await?;
    // Getting Latest blocks for mined TRXs
    let mut latest_block_stream = ws_provider.subscribe_blocks().await?;

    // Variables
    let mut block_number = U64::zero();
    let mut nonce = U256::zero();
    let mut base_fee = U256::zero();

    let result = provider.get_balance(my_address().parse::<H160>().unwrap(), Some(BlockNumber::Latest.into())).await;
    println!("Bal {:#?}", result);

    loop {
        tokio::select! {
            Some(latest_block) = latest_block_stream.next() => {

                // Get Mined Blocks
                block_number = latest_block.number.unwrap();
                nonce = provider.get_transaction_count(my_address().parse::<H160>().unwrap(), Some(BlockNumber::Latest.into())).await?;
                base_fee = latest_block.next_block_base_fee().unwrap();
                println!("{} {}", "New nonce:", nonce);
                println!("{} {}", "New Block:", block_number);
                println!("{} {}", "New fee:", base_fee);

            },

            Some(pending_transaction) = pending_transactions_stream.next() => {

                let results = decode_buy_share(pending_transaction.data);

                if is_new_share(results) {
                    let buy_transaction = build_buy_transaction(results.1, U256::one(), results.7);
                    let result2 = provider.call(&buy_transaction.into(), Some(block_number.into())).await;
                    println!("PT {:#?} \n {:#?}", results, result2)
                }
                
            },

            else => {
                eprintln!("{}", "[X] Failed To Fetch The Pending Transaction Or Block!");
                break;
            }
        }
    }

    Ok(())
}
