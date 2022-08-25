use std::fmt::{Debug, Formatter};
use ethers::types::{Address, U256};

pub struct Token {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub chain_id: U256,
}

impl Token {
    pub fn sorts_before(&self, other: &Token) -> bool {
        assert_eq!(self.chain_id, other.chain_id);
        assert_ne!(self.address, other.address);
        return self.address.lt(&other.address);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address &&
            self.chain_id == other.chain_id &&
            self.name == other.name &&
            self.symbol == other.symbol &&
            self.decimals == other.decimals
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token Address: {:?} -- Name: {} -- Symbol: {} -- Decimals: {} -- Chain ID: {}", self.address, self.name, self.symbol, self.decimals, self.chain_id)
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    static JOE_TOKEN_ADDRESS: &'static str = "0x6e84a6216ea6dacc71ee8e6b0a5b7322eebc0fdd";
    static WAVAX_TOKEN_ADDRESS: &'static str = "0xb31f66aa3c1e785363f0875a1b74e27b85fd66c7";
    static USDC_TOKEN_ADDRESS: &'static str = "0xb97ef9ef8734c71904d8002f8b6bc66dd9c48a6e";

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
    async fn it_returns_false_as_joe_token_address_is_smaller_than_wavax_address() {
        let token0 = setup_token(
            WAVAX_TOKEN_ADDRESS,
            "Wrapped AVAX",
            "WAVAX",
            18_u8,
            U256::from(43114_u64),
        );

        let token1 = setup_token(
            JOE_TOKEN_ADDRESS,
            "JOE TOKEN",
            "JOE",
            18_u8,
            U256::from(43114_u64),
        );

        let result = token0.sorts_before(&token1);
        assert_eq!(result, false);
    }

    #[tokio::test]
    async fn it_returns_true_as_wavax_token_address_is_smaller_than_usdc_address() {
        let token0 = setup_token(
            WAVAX_TOKEN_ADDRESS,
            "Wrapped AVAX",
            "WAVAX",
            18_u8,
            U256::from(43114_u64),
        );

        let token1 = setup_token(
            USDC_TOKEN_ADDRESS,
            "USD Coin",
            "USDC",
            6_u8,
            U256::from(43114_u64),
        );

        let result = token0.sorts_before(&token1);
        assert_eq!(result, true);
    }

    #[tokio::test]
    #[should_panic]
    async fn it_panic_due_to_chain_id_being_different() {
        let token0 = setup_token(
            WAVAX_TOKEN_ADDRESS,
            "Wrapped AVAX",
            "WAVAX",
            18_u8,
            U256::from(1_u64),
        );

        let token1 = setup_token(
            JOE_TOKEN_ADDRESS,
            "JOE TOKEN",
            "JOE",
            18_u8,
            U256::from(43114_u64),
        );

        token0.sorts_before(&token1);
    }

    #[tokio::test]
    #[should_panic]
    async fn it_panic_due_to_both_token_being_the_same() {
        let token0 = setup_token(
            JOE_TOKEN_ADDRESS,
            "JOE TOKEN",
            "JOE",
            18_u8,
            U256::from(43114_u64),
        );

        let token1 = setup_token(
            JOE_TOKEN_ADDRESS,
            "JOE TOKEN",
            "JOE",
            18_u8,
            U256::from(43114_u64),
        );

        token0.sorts_before(&token1);
    }
}
