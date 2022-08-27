use ethers::prelude::U256;
use crate::currency::Currency;
use crate::fraction::Fraction;
use crate::pair::Pair;
use crate::price::Price;
use crate::token::Token;

pub struct Route {
    pub pairs: Vec<Pair>,
    pub path: Vec<Token>,
    pub input: Token,
    pub output: Token,
    pub mid_price: Price,
}

impl Route {
    pub fn new(pairs: Vec<Pair>, input: Token, output: Token) -> Route {
        assert_eq!(pairs.len() > 0, true, "We need at least one pair to create a route.");
        for pair in &pairs {
            assert_eq!(pair.chain_id, U256::from(43114_u64), "A pair is not using the correct chain_id");
        }

        assert_eq!(pairs.get(0).unwrap().involves_token(&input), true, "First pair in the route does not contains the input token");
        assert_eq!(pairs.get(pairs.len() - 1).unwrap().involves_token(&output), true, "Last pair in the route does not contains the output token");

        let mut path: Vec<Token> = Vec::new();
        path.push(input.clone());

        let _ = pairs.iter().enumerate().for_each(|(i, pair): (usize, &Pair)| {
            let current_input: &Token = path.get(0).unwrap();
            assert_eq!(current_input.address.eq(&pair.token_0.address) || current_input.address.eq(&pair.token_1.address), true, "Issue with path");
            let output = current_input.address.eq(&pair.token_0.address).then(|| pair.token_1.clone()).unwrap_or(pair.token_0.clone());
            path.push(output);
        });

        let mut route = Route {
            pairs,
            path: vec![],
            input,
            output,
            mid_price: Price {
                base_currency: Token {
                    address: Default::default(),
                    name: "".to_string(),
                    symbol: "".to_string(),
                    decimals: 0,
                    chain_id: Default::default(),
                },
                quote_currency: Token {
                    address: Default::default(),
                    name: "".to_string(),
                    symbol: "".to_string(),
                    decimals: 0,
                    chain_id: Default::default(),
                },
                scalar: Fraction { numerator: 0, denominator: 0 },
                fraction: Fraction { numerator: 0, denominator: 0 },
            },
        };

        route.mid_price = Price::price_from_route(&route);

        route
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use ethers::prelude::{Address, U256};
    use crate::fraction::Fraction;
    use crate::pair::Pair;
    use crate::price::Price;
    use crate::route::Route;
    use crate::token::Token;

    // const token0 = new Token(ChainId.FUJI, '0x0000000000000000000000000000000000000001', 18, 't0')
    // const token1 = new Token(ChainId.FUJI, '0x0000000000000000000000000000000000000002', 18, 't1')
    // const weth = WAVAX[ChainId.FUJI]
    // const pair_0_1 = new Pair(new TokenAmount(token0, '100'), new TokenAmount(token1, '200'), ChainId.FUJI)
    // const pair_0_weth = new Pair(new TokenAmount(token0, '100'), new TokenAmount(weth, '100'), ChainId.FUJI)
    // const pair_1_weth = new Pair(new TokenAmount(token1, '175'), new TokenAmount(weth, '100'), ChainId.FUJI)
    static JOE_TOKEN_ADDRESS: &str = "0x6e84a6216ea6dacc71ee8e6b0a5b7322eebc0fdd";
    static WAVAX_TOKEN_ADDRESS: &str = "0xb31f66aa3c1e785363f0875a1b74e27b85fd66c7";

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
    async fn construct_path_from_tokens() {
        let token_0 = setup_token(
            WAVAX_TOKEN_ADDRESS,
            "Wrapped AVAX",
            "WAVAX",
            18_u8,
            U256::from(43114_u64),
        );

        let token_1 = setup_token(
            JOE_TOKEN_ADDRESS,
            "JOE TOKEN",
            "JOE",
            18_u8,
            U256::from(43114_u64),
        );

        let pair = Pair {
            address: Default::default(),
            token_0: token_0.clone(),
            token_1: token_1.clone(),
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0,
            reserves: (0, 0),
            chain_id: U256::from(43114_u64)
        };
        let route = Route::new(vec![pair], token_0, token_1);
        println!("ssss");
        // const route = new Route([pair_0_1], token0)
        // expect(route.pairs).toEqual([pair_0_1])
        // expect(route.path).toEqual([token0, token1])
        // expect(route.input).toEqual(token0)
        // expect(route.output).toEqual(token1)
        // expect(route.chainId).toEqual(ChainId.FUJI)
    }
}