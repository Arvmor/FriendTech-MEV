use std::{sync::Arc, ops::Add};

use ethers::{
    types::{Eip1559TransactionRequest, BlockNumber, U64, transaction::eip2930::AccessList, U256},
    providers::{Http, Provider, Middleware}, signers::Wallet, prelude::{*, k256::ecdsa::SigningKey}
};
use eyre::Result;

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
        Ok(info) => println!("Success: {:#?}", info),
        Err(error) => eprintln!("failed {:#?}", error)
    }
    Ok(())
}