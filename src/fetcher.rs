//! Will contains code related to on chain data fetching

use std::error::Error;
use std::sync::Arc;
use ethers::abi::{Address};
use ethers_contract::abigen;
use ethers_providers::{Middleware, Provider, Ws};
use crate::token::Token;

abigen!(
    ERC20Token,
    "./abi/erc20.abi"
);

abigen!(
    Pair,
    "./abi/pair.abi"
);

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
        chain_id: provider.get_chainid().await.unwrap()
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use ethers::types::U256;
    use ethers::utils::Anvil;
    use super::*;

    static JOE_TOKEN_ADDRESS: &'static str = "0x6e84a6216ea6dacc71ee8e6b0a5b7322eebc0fdd";
    static NOT_A_TOKEN_ADDRESS: &'static str = "0x123456789abcdef0000000000000000000000000";

    fn setup_token(
        address: &str,
        name: &str,
        symbol: &str,
        decimals: u8,
        chain_id: U256,
    ) -> Token {
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
        let anvil =
            Anvil::at(dotenv!("ANVIL_PATH"))
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
        let result = fetch_token_data(joe_token_address, &arc_provider).await.unwrap();

        assert_eq!(result.eq(&expected), true);
    }

    #[tokio::test]
    async fn it_fails_fetching_from_a_random_address() {
        let anvil =
            Anvil::at(dotenv!("ANVIL_PATH"))
                .fork(dotenv!("RPC_ENDPOINT"))
                .spawn();
        let ws = Ws::connect(anvil.ws_endpoint().as_str()).await.unwrap();
        let provider = Provider::new(ws);
        let arc_provider = Arc::new(provider);

        let joe_token_address: Address = Address::from_str(NOT_A_TOKEN_ADDRESS).unwrap();
        let result = fetch_token_data(joe_token_address, &arc_provider).await;
        assert_eq!(result.is_err(), true);
    }
}
