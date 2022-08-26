use std::ops::Mul;
use crate::fraction::Fraction;
use crate::route::Route;
use crate::token::Token;

#[derive(Clone)]
pub struct Price {
    pub base_currency: Token,
    pub quote_currency: Token,
    pub token_0_reserve: u128,
    pub token_1_reserve: u128,
}

impl Price {
    pub fn price_from_route(
        route: &Route
    ) -> Price {
        let mut prices: Vec<Price> = Vec::new();

        route.pairs.iter().enumerate().for_each(|(i, pair)| {
            prices.push(
                route.path.get(i).unwrap().address
                    .eq(&pair.token_0.address)
                    .then(|| Price {
                        base_currency: pair.token_0.clone(),
                        quote_currency: pair.token_1.clone(),
                        token_0_reserve: pair.reserves.0,
                        token_1_reserve: pair.reserves.1,
                    })
                    .unwrap_or(Price {
                        base_currency: pair.token_1.clone(),
                        quote_currency: pair.token_0.clone(),
                        token_0_reserve: pair.reserves.1,
                        token_1_reserve: pair.reserves.0,
                    })
            )
        });

        let price = prices.iter().reduce(|acc, curr|
            &acc.multiply(curr)
        ).unwrap();

        *price
    }

    pub fn multiply(
        &self,
        other: &Price,
    ) -> Price {
        assert_eq!(self.quote_currency.eq(&other.base_currency), true, "Mismatch in the token of price");
        let fraction = Fraction {
            numerator: self.token_0_reserve.mul(other.token_0_reserve),
            denominator: self.token_1_reserve.mul(other.token_1_reserve),
        };

        Price {
            base_currency: self.base_currency.clone(),
            quote_currency: self.quote_currency.clone(),
            token_0_reserve: fraction.denominator,
            token_1_reserve: fraction.numerator,
        }
    }
}