use std::sync::Arc;

use dotenv::dotenv;
use eyre::Result;
use ethers::{providers::{Http, Provider, Ws, Middleware, StreamExt}, types::{BlockNumber, H160, Filter, U256, U64}, prelude::*};

mod credentials;
use credentials::*;
mod decoder;
use decoder::*;
mod builder;
use builder::*;
mod runner;
use runner::*;
mod api;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();

    // Sender/Receiver to watch for new wallets
    let (sender, mut receiver) = mpsc::channel(100);
    // Establishing Connections to WS & HTTP providers
    let provider: Arc<Provider<Http>> = Arc::new(Provider::<Http>::try_from(http_provider_url())?);
    let mainnet_ws_provider: Provider<Ws> = Provider::<Ws>::connect("wss://convincing-frequent-general.quiknode.pro/").await?;
    // To fetch pending transactions
    let ws_provider: Provider<Ws> = Provider::<Ws>::connect(ws_provider_url()).await?;
    // Getting pending transactions from mempool
    let mut mined_transactions_stream = ws_provider.subscribe_logs(&Filter::new().select(BlockNumber::Latest).event("Trade(address,address,bool,uint256,uint256,uint256,uint256,uint256)")).await?;
    let mut pending_mainnet_stream = mainnet_ws_provider.subscribe_full_pending_txs().await?;
    // Getting Latest blocks for mined TRXs
    let mut latest_block_stream = ws_provider.subscribe_blocks().await?;
    // This signs transactions
    let wallet: LocalWallet = my_private_key().parse().unwrap(); 
    let wallet = wallet.with_chain_id(8453u16);
    let client = SignerMiddleware::new(provider.clone(), wallet);

    // Variables
    let mut block_number = U64::zero();
    let mut nonce = U256::zero();
    let mut base_fee = U256::zero();
    let mut watchlist = vec![];

    let result = provider.get_balance(my_address().parse::<H160>().unwrap(), Some(BlockNumber::Latest.into())).await;
    println!("Bal {:#?}", result);

    loop {
        tokio::select! {
            Some(to_watch) = receiver.recv() => {
                watchlist.push(to_watch)
            },
            Some(latest_block) = latest_block_stream.next() => {

                // Get Mined Blocks
                block_number = latest_block.number.unwrap();
                nonce = provider.get_transaction_count(my_address().parse::<H160>().unwrap(), Some(BlockNumber::Latest.into())).await?;
                base_fee = latest_block.next_block_base_fee().unwrap();

            },

            Some(mined_transaction) = mined_transactions_stream.next() => {

                let results = decode_buy_share(mined_transaction.data);

                if is_new_share(results) {
                    tokio::spawn (
                        runner(watchlist.clone(), results, client.clone(), nonce, block_number, base_fee)
                    );
                }
                
            },

            Some(pending_transaction) = pending_mainnet_stream.next() => {

                let new_address = decode_bridge_to_base(pending_transaction);
                if new_address.is_some(){
                    tokio::spawn(
                        add_to_watchlist(new_address.unwrap(), sender.clone())
                    );
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
