use ethers::prelude::U256;
use crate::currency::Currency;
use crate::pair::Pair;
use crate::price::Price;
use crate::token::Token;

pub struct Route {
    pub pairs: Vec<Pair>,
    pub path: Vec<Token>,
    pub input: Token,
    pub output: Token,
    pub mid_price: Price
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

        Route {
            pairs,
            path: vec![],
            input,
            output,
        }
    }
}