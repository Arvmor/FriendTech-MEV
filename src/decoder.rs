use std::str::FromStr;

use ethers::types::{Bytes, H160, U256};

pub fn decode_buy_share(
    input: Bytes
) -> (H160, H160, bool, U256, U256, U256, U256, U256) {
    // Parse to string
    let data_hex = hex::encode(&input);
    let buyer = H160::from_str(&data_hex[24..64]).unwrap();
    let owner = H160::from_str(&data_hex[88..128]).unwrap();
    let is_buy = if U256::from_str(&data_hex[128..192]).unwrap() == U256::one() {
        true
    } else {
        false
    };
    let share_amount = U256::from_str(&data_hex[192..256]).unwrap();
    let eth_amount = U256::from_str(&data_hex[256..320]).unwrap();
    let protocol_eth_amount = U256::from_str(&data_hex[320..384]).unwrap();
    let subject_eth_amount = U256::from_str(&data_hex[384..448]).unwrap();
    let supply = U256::from_str(&data_hex[448..]).unwrap();
    (buyer, owner, is_buy, share_amount, eth_amount, protocol_eth_amount, subject_eth_amount, supply)
}


#[cfg(test)]
mod test_case {
    use super::*;

    #[test]
    fn decode_buy() {
        let input: Bytes = "0x000000000000000000000000bb36dae5010ff593ef3cfb09a50b5907abb26eaf000000000000000000000000f968e9baf94d5b01f7abf9db1d4cb582711eaa620000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000a1deda33942800000000000000000000000000000000000000000000000000000817f14f610200000000000000000000000000000000000000000000000000000817f14f610200000000000000000000000000000000000000000000000000000000000000001c".parse().unwrap();
        let result = decode_buy_share(input);
        println!("{:#?}", result);
        assert_ne!(result, (H160::default(), H160::default(), bool::default(), U256::default(), U256::default(), U256::default(), U256::default(), U256::default()));
    }
}