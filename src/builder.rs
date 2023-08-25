use std::ops::{Sub, Mul, Add, Div};

use ethers::{types::{
    U256, H160, Bytes, Eip1559TransactionRequest
}, utils::{to_checksum, parse_ether}};

use crate::credentials::my_address;

pub fn build_buy_transaction(
    buy_from: H160,
    amount: U256,
    supply: U256,
    nonce: U256,
) -> Eip1559TransactionRequest {
    let data = format!("0x6945b123{:0>64}{:0>64}", &to_checksum(&buy_from, Some(1))[2..], amount);
    let value = calculate_summation(supply, amount);
    
    Eip1559TransactionRequest::default()
        .from(my_address().parse::<H160>().unwrap())
        .to("0xcf205808ed36593aa40a44f10c7f7c2f67d4a4d4".parse::<H160>().unwrap())
        .value(value)
        .gas(200_000)
        .nonce(nonce)
        .max_fee_per_gas(0)
        .max_priority_fee_per_gas(0)
        .chain_id(8453)
        .data(data.parse::<Bytes>().unwrap())
}

fn calculate_summation(supply: U256, amount: U256) -> U256 {
    let sum1 = if supply == U256::zero() {
        U256::zero()
    } else {
        supply.sub(1).mul(supply).mul(U256::from(2).mul(supply.sub(1)).add(1)).div(6)
    };

    let sum2 = if supply == U256::zero() && amount == U256::one() {
        U256::zero()
    } else {
        supply.sub(1).add(amount).mul(supply.add(amount)).mul(U256::from(2).mul(supply.sub(1).add(amount)).add(1)).div(6)
    };

    let summation = sum2 - sum1;
    let price = summation.mul(parse_ether("1").unwrap()).div(16000);
    let fee = price.mul(50000000000000000u64).div(parse_ether("1").unwrap());
    price.add(fee).add(fee)
}

pub fn is_new_share (
    info: (H160, H160, bool, U256, U256, U256, U256, U256)
) -> bool {
    if info.0 == info.1 && info.3 == U256::one() && info.7 == U256::one() && info.2 {
        return true
    } 

    false
}

#[cfg(test)]
mod test_case {
    use super::*;

    #[test]
    fn get_fee() {
        let result = calculate_summation(U256::one(), U256::from(3));
        println!("Result {:#?}\n", result);

        assert_ne!(result, U256::zero());
    }
    #[test]
    fn get_fee2() {
        let result = calculate_summation(U256::from(4), U256::one());
        println!("Result {:#?}", result);

        assert_ne!(result, U256::zero());
    }
    #[test]
    fn get_fee3() {
        let result = calculate_summation(U256::one(), U256::from(5));
        println!("Result {:#?}", result);

        assert_ne!(result, U256::zero());
    }
}