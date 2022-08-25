use crate::route::Route;
use crate::token::Token;

pub struct Price {
    pub base_currency: Token,
    pub quote_currency: Token,
    pub token_0_reserve: u128,
    pub token_1_reserve: u128,
    pub price: f64
}

impl Price {
    pub fn price_from_route(
        route: Route
    ) -> Price {
        let mut prices: Vec<Price> = Vec::new();

        route.pairs.iter().enumerate().for_each(|(i, pair)| {
            prices.push(
                route.path.get(i).unwrap().address
                    .eq(&pair.token_0.address)
                    .then(Price {
                        base_currency: pair.token_0.clone(),
                        quote_currency: pair.token_1.clone(),
                        token_0_reserve: pair.reserves.0,
                        token_1_reserve: pair.reserves.1,
                        price: 0.
                    })
                    .unwrap_or(Price {
                        base_currency: pair.token_1.clone(),
                        quote_currency: pair.token_0.clone(),
                        token_0_reserve: pair.reserves.1,
                        token_1_reserve: pair.reserves.0,
                        price: 0.
                    })
            )
        });

        let reserve0_with_decimals = (reserves.0 as f64 / (10_u64.pow(token0_decimals.into())) as f64) as f64;
        let reserve1_with_decimals = (reserves.1 as f64 / (10_u64.pow(token1_decimals.into())) as f64) as f64;

        prices.slice(1).iter().reduce(|acc, curr| {

        });

        Price {
            base_currency: Token {},
            quote_currency: Token {},
            token_0_reserve: 0,
            token_1_reserve: 0,
            price: 0.
        }
    }

    pub fn multiply(
        &self,
        other: Price
    ) -> Price {
        assert_eq!(self.quote_currency.eq(&other.base_currency), "Mismatch in the token of price");
        let fraction =
    }
}