use std::fmt::{Debug, Formatter};
use ethers::types::{Address, Bytes, U256};
use std::sync::{LockResult, Mutex};
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use ethers::abi;
use ethers::abi::Token::{Address as TokenAddress, Array, Uint as TokenUint};
use ethers::prelude::H160;
use ethers::types::BlockId::Hash;
use ethers::utils::{format_bytes32_string, get_create2_address, get_create2_address_from_hash, hex, keccak256};
use crate::token::Token;
use hex_literal::hex;

pub struct Pair {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub chain_id: U256,
}

lazy_static! {
    static ref PAIR_ADDRESS_CACHE: Mutex<HashMap<Address, HashMap<Address, Address>>> = Mutex::new(HashMap::new());
}

impl Pair {
    pub fn get_address(
        token_0: &Token,
        token_1: &Token,
        chain_id: U256,
    ) -> H160 {
        let (token0, token1): (Token, Token);
        if token_0.sorts_before(&token_1) {
            token0 = Token {
                address: token_0.address,
                name: token_0.name.to_string(),
                symbol: token_0.symbol.to_string(),
                decimals: token_0.decimals,
                chain_id: token_0.chain_id,
            };
            token1 = Token {
                address: token_1.address,
                name: token_1.name.to_string(),
                symbol: token_1.symbol.to_string(),
                decimals: token_1.decimals,
                chain_id: token_1.chain_id,
            };
        } else {
            token0 = Token {
                address: token_1.address,
                name: token_1.name.to_string(),
                symbol: token_1.symbol.to_string(),
                decimals: token_1.decimals,
                chain_id: token_1.chain_id,
            };
            ;
            token1 = Token {
                address: token_0.address,
                name: token_0.name.to_string(),
                symbol: token_0.symbol.to_string(),
                decimals: token_0.decimals,
                chain_id: token_0.chain_id,
            };
        }

        match PAIR_ADDRESS_CACHE.lock() {
            Ok(mut map) => {
                if !map.contains_key(&token0.address) {
                    map.insert(
                        token0.address,
                        HashMap::new(),
                    );
                }

                if !map.get(&token0.address).unwrap().contains_key(&token1.address) {
                    map.get_mut(&token0.address).unwrap().insert(
                        token1.address,
                        Address::zero(),
                    );
                }

                if map.get(&token0.address).unwrap().contains_key(&token1.address) &&
                    map.get(&token0.address).unwrap().get(&token1.address).unwrap().eq(&Address::zero()) {
                    // ToDo Move those values in a separate files for constants. ALso make it configurable so we can use the sdk on Fuji.
                    let TRADER_JOE_POOL_INIT_CODE_HASH =
                        hex::decode("0bbca9af0511ad1a1da383135cf3a8d2ac620e549ef9f6ae3a4c33c2fed0af91").unwrap();
                    let factory: Address = "0x9ad6c38be94206ca50bb0d90783181662f0cfa10"
                        .parse()
                        .unwrap();

                    let pool_address =
                        get_create2_address_from_hash(factory, keccak256([token_0.address.0, token_1.address.0].concat()), TRADER_JOE_POOL_INIT_CODE_HASH);

                    map.get_mut(
                        &token0.address).unwrap().insert(
                        token1.address,
                        pool_address);
                }

                map.get(&token0.address).unwrap().get(&token1.address).unwrap().clone()
            }
            Err(err) => {
                eprintln!("Issue while attempting to lock PAIR_ADDRESS_CACHE -> {:?}", err);
                Address::zero()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use ethers::abi::Address;
    use ethers::prelude::U256;
    use ethers::types::{H160};
    use crate::pair::Pair;
    use crate::token::Token;

    static JOE_TOKEN_ADDRESS: &'static str = "0x6e84a6216ea6dacc71ee8e6b0a5b7322eebc0fdd";
    static WAVAX_TOKEN_ADDRESS: &'static str = "0xb31f66aa3c1e785363f0875a1b74e27b85fd66c7";
    static JOE_WAVAX_LP_ADDRESS: &'static str = "0x454e67025631c065d3cfad6d71e6892f74487a15";

    #[tokio::test]
    async fn it_returns_correct_pair_address() {
        let token_a = Token {
            address: Address::from_str(JOE_TOKEN_ADDRESS).unwrap(),
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0,
            chain_id: Default::default(),
        };

        let token_b = Token {
            address: Address::from_str(WAVAX_TOKEN_ADDRESS).unwrap(),
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0,
            chain_id: Default::default(),
        };


        let result = Pair::get_address(
            &token_a,
            &token_b,
            U256::from(43114_u64),
        );

        assert_eq!(result, Address::from_str(JOE_WAVAX_LP_ADDRESS).unwrap());
    }
}