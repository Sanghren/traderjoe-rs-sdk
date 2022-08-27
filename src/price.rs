use std::ops::Mul;
use ethers::prelude::U256;
use crate::fraction::Fraction;
use crate::route::Route;
use crate::token::Token;

#[derive(Clone)]
pub struct Price {
    pub base_currency: Token,
    pub quote_currency: Token,
    pub scalar: Fraction,
    pub fraction: Fraction
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
                        scalar: Fraction { numerator: 0, denominator: 0 },
                        fraction: Fraction { numerator: 0, denominator: 0 }
                    })
                    .unwrap_or(Price {
                        base_currency: pair.token_1.clone(),
                        quote_currency: pair.token_0.clone(),
                        scalar: Fraction { numerator: 0, denominator: 0 },
                        fraction: Fraction { numerator: 0, denominator: 0 }
                    })
            )
        });

        prices.into_iter().reduce(|acc, curr|
            acc.multiply(&curr)
        ).unwrap().clone()
    }

    pub fn multiply(
        &self,
        other: &Price,
    ) -> Price {
        assert_eq!(self.quote_currency.eq(&other.base_currency), true, "Mismatch in the token of price");
        let fraction = self.fraction.multiply(&other.fraction);


        Price {
            base_currency: self.base_currency.clone(),
            quote_currency: self.quote_currency.clone(),
            scalar: self.scalar.clone(),
            fraction
        }
    }
}