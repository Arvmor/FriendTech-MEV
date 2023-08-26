use std::{sync::Arc, ops::{Add, Mul, Div}, error::Error};

use ethers::{
    types::{Eip1559TransactionRequest, BlockNumber, U64, transaction::eip2930::AccessList, U256},
    providers::{Http, Provider, Middleware}, signers::Wallet, prelude::{*, k256::ecdsa::SigningKey}
};
use eyre::Result;

use crate::{api::{get_user, get_user_followers}, builder::build_buy_transaction};

pub async fn send_trx(
    provider: Arc<Provider<Http>>,
    client: SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
    mut transaction: Eip1559TransactionRequest,
    block_number: U64,
    base_fee: U256
) -> Result<()> {
    let frontrun_access_list = match provider.create_access_list(&transaction.clone().into(), Some(BlockNumber::Latest.into())).await {
        Ok(access_list) => access_list.access_list,
        Err(_error) => {
            eprintln!("{}", _error);
            AccessList::default()
        }
    };

    transaction = transaction.access_list(frontrun_access_list).max_fee_per_gas(base_fee).max_priority_fee_per_gas(base_fee);

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
    results: (H160, H160, bool, U256, U256, U256, U256, U256),
    provider: Arc<Provider<Http>>,
    client: SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
    nonce: U256,
    block_number: U64,
    base_fee: U256
) -> Result<(), Box<dyn Error + Send>> {

    println!("Victim {:#?}", results);
    let username = get_user(results.1, 7).await.unwrap();
    println!("ID: @{:#?}", username);
    let follower_count = get_user_followers(&username).await.unwrap();

    println!("Follower Count: {:#?}", follower_count);
    if follower_count > 1000 {
        println!("SENDING !");
        tokio::spawn(
            send_trx(provider.clone(), client.clone(), build_buy_transaction(results.1, U256::from(1), results.7, nonce), block_number, base_fee.mul(700u16).div(10))
        );
    }

    Ok(())
}