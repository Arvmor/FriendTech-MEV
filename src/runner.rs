use std::{sync::Arc, ops::{Add, Mul, Div}, error::Error};
use colored::*;
use ethers::{
    types::{Eip1559TransactionRequest, BlockNumber, U64, U256},
    providers::{Http, Provider, Middleware}, signers::Wallet, prelude::{*, k256::ecdsa::SigningKey}
};
use eyre::Result;
use tokio::sync::mpsc::Sender;

use crate::{api::{get_user, get_user_followers}, builder::build_buy_transaction};

pub async fn send_trx(
    client: SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
    mut transaction: Eip1559TransactionRequest,
    block_number: U64,
    base_fee: U256
) -> Result<()> {
    transaction = transaction.max_fee_per_gas(base_fee).max_priority_fee_per_gas(base_fee);

    match client.send_transaction(transaction, Some(BlockNumber::Number(block_number.add(1)).into())).await {
        Ok(info) => {
            match info.await {
                Ok(info2) => println!("Success: {:#?}", info2.unwrap().transaction_hash),
                Err(error) => eprintln!("failed {:#?}", error)
            }
        },
        Err(error) => eprintln!("failed {:#?}", error)
    }
    Ok(())
}

pub async fn runner(
    watchlist: Vec<H160>,
    results: (H160, H160, bool, U256, U256, U256, U256, U256),
    client: SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
    nonce: U256,
    block_number: U64,
    base_fee: U256
) -> Result<(), Box<dyn Error + Send>> {

    if watchlist.contains(&results.1) {
        println!("{}", "SENDING !".green());
        send_trx(client.clone(), build_buy_transaction(results.1, U256::from(4), results.7, nonce), block_number, base_fee.mul(1100u16).div(10)).await.unwrap();
    } else {
        let username = match get_user(results.1, 14).await {
            Ok(username) => username,
            Err(error) => {
                eprint!("{} {:#?}", "[!] Failed Fetch Username".red(), error);
                return Ok(())
            }
        };
        println!("{} @{:#?}", "ID:".blue(), username);
        let follower_count = get_user_followers(&username).await.unwrap();
        println!("{} {:#?}", "Follower Count:".blue(), follower_count);

        if follower_count > 1000 {
            println!("{}", "SENDING !".green());
            send_trx(client.clone(), build_buy_transaction(results.1, U256::from(4), results.7, nonce), block_number, 60000000000u128.into()).await.unwrap();
        }
    }

    Ok(())
}

pub async fn add_to_watchlist(
    address: H160,
    sender: Sender<H160>
) -> Result<(), Box<dyn Error + Send>> {

    println!("{} {:#?}", "Victim".yellow(), address);
    let username = match get_user(address, 1000).await {
        Ok(username) => username,
        Err(error) => {
            eprint!("{} {:#?}", "[!] Failed Fetch Username".red(), error);
            return Ok(())
        }
    };

    println!("{} @{:#?}", "ID:".blue(), username);
    let follower_count = get_user_followers(&username).await.unwrap();
    println!("{} {:#?}", "Follower Count:".blue(), follower_count);

    if follower_count > 1000 {
        println!("{} {:#?}", "Added !".green(), address);
        sender.send(address).await.expect("Failed to add address");
    }

    Ok(())
}