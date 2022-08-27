//! Will contains code related to on chain data fetching

use crate::pair::Pair;
use crate::token::Token;
use ethers::abi::Address;
use ethers::prelude::U256;
use ethers_contract::abigen;
use ethers_providers::{Middleware, Provider, Ws};
use std::error::Error;
use std::sync::Arc;

abigen!(ERC20Token, "./abi/erc20.abi");

abigen!(PairContract, "./abi/pair.abi");

/// Will fetch data for the provided Address, granted it is a ERC20 token address.
pub async fn fetch_token_data(
    address: Address,
    provider: &Arc<Provider<Ws>>,
) -> Result<Token, Box<dyn Error>> {
    let erc20_token = ERC20Token::new(address, provider.clone());

    let decimals = erc20_token.decimals().call().await?;
    let name = erc20_token.name().call().await?;
    let symbol = erc20_token.symbol().call().await?;

    Ok(Token {
        address,
        name,
        symbol,
        decimals,
        chain_id: provider.get_chainid().await.unwrap(),
    })
}

pub async fn fetch_pair_data(
    token_a: &Token,
    token_b: &Token,
    provider: &Arc<Provider<Ws>>,
) -> Result<Pair, Box<dyn Error>> {
    assert_eq!(token_a.chain_id, U256::from(43114_u64));
    assert_eq!(token_b.chain_id, U256::from(43114_u64));

    let pair_address = Pair::get_address(&token_a, &token_b, provider.get_chainid().await.unwrap());

    let reserves = PairContract::new(pair_address, provider.clone())
        .get_reserves()
        .call()
        .await
        .unwrap();

    Ok(Pair {
        address: pair_address,
        token_0: token_a.sorts_before(&token_b).then(|| { token_a.clone() }).unwrap_or(token_b.clone()),
        token_1: token_a.sorts_before(&token_b).then(|| { token_b.clone() }).unwrap_or(token_a.clone()),
        name: "Joe Liquidity".to_string(),
        symbol: "JLP".to_string(),
        decimals: 18,
        reserves: (reserves.0 as i128, reserves.1 as i128),
        chain_id: provider.get_chainid().await.unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::U256;
    use ethers::utils::Anvil;
    use std::str::FromStr;

    static JOE_TOKEN_ADDRESS: &str = "0x6e84a6216ea6dacc71ee8e6b0a5b7322eebc0fdd";
    static WAVAX_TOKEN_ADDRESS: &str = "0xb31f66aa3c1e785363f0875a1b74e27b85fd66c7";
    static NOT_A_TOKEN_ADDRESS: &str = "0x123456789abcdef0000000000000000000000000";
    static JOE_WAVAX_LP_ADDRESS: &str = "0x454e67025631c065d3cfad6d71e6892f74487a15";

    fn setup_token(address: &str, name: &str, symbol: &str, decimals: u8, chain_id: U256) -> Token {
        Token {
            address: Address::from_str(address).unwrap(),
            name: name.to_string(),
            symbol: symbol.to_string(),
            decimals,
            chain_id,
        }
    }

    #[tokio::test]
    async fn it_fetched_joetoken_info() {
        let anvil = Anvil::at(dotenv!("ANVIL_PATH"))
            .fork(dotenv!("RPC_ENDPOINT"))
            .spawn();
        let ws = Ws::connect(anvil.ws_endpoint().as_str()).await.unwrap();
        let provider = Provider::new(ws);
        let arc_provider = Arc::new(provider);
        let expected = setup_token(
            JOE_TOKEN_ADDRESS,
            "JoeToken",
            "JOE",
            18_u8,
            U256::from(43114_u64),
        );

        let joe_token_address: Address = Address::from_str(JOE_TOKEN_ADDRESS).unwrap();
        let result = fetch_token_data(joe_token_address, &arc_provider)
            .await
            .unwrap();

        assert_eq!(result.eq(&expected), true);
    }

    #[tokio::test]
    async fn it_fails_fetching_from_a_random_address() {
        let anvil = Anvil::at(dotenv!("ANVIL_PATH"))
            .fork(dotenv!("RPC_ENDPOINT"))
            .spawn();
        let ws = Ws::connect(anvil.ws_endpoint().as_str()).await.unwrap();
        let provider = Provider::new(ws);
        let arc_provider = Arc::new(provider);

        let joe_token_address: Address = Address::from_str(NOT_A_TOKEN_ADDRESS).unwrap();
        let result = fetch_token_data(joe_token_address, &arc_provider).await;
        assert_eq!(result.is_err(), true);
    }

    #[tokio::test]
    async fn it_fetched_pairs_info_for_joe_wavax_pool() {
        //ToDO Pin to a specific block, atm having issues with missing trie node each time I specify a block
        let anvil = Anvil::at(dotenv!("ANVIL_PATH"))
            .fork(dotenv!("RPC_ENDPOINT"))
            .spawn();
        let ws = Ws::connect(anvil.ws_endpoint().as_str()).await.unwrap();
        let provider = Provider::new(ws);
        let arc_provider = Arc::new(provider);

        let joe_token = setup_token(
            JOE_TOKEN_ADDRESS,
            "JoeToken",
            "JOE",
            18_u8,
            U256::from(43114_u64),
        );
        let wavax_token = setup_token(
            WAVAX_TOKEN_ADDRESS,
            "Wrapped Avax",
            "WAVAX",
            18_u8,
            U256::from(43114_u64),
        );

        let result = fetch_pair_data(&joe_token, &wavax_token, &arc_provider).await;
        assert_eq!(result.is_err(), false);
        let result = result.unwrap();
        // ToDo WIll have to fix the pinning to a specific block to be more specific in those assertion
        assert_ne!(result.reserves.0, 0);
        assert_ne!(result.reserves.1, 0);
        assert_eq!(
            result.address,
            Address::from_str(JOE_WAVAX_LP_ADDRESS).unwrap()
        );
        assert_eq!(
            result.token_0,
            joe_token
        );
        assert_eq!(
            result.token_1,
            wavax_token
        );
    }
}
