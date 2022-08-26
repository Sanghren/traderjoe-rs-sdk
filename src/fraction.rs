use std::ops::Div;

pub struct Fraction {
    pub numerator: u128,
    pub denominator: u128
}

impl Fraction {

    pub fn quotient(&self) -> u128 {
        self.numerator.div(self.denominator)
    }
    
    pub fn remainder(&self) -> Fraction {
        Fraction{ numerator: self.numerator % &self.denominator, denominator: self.denominator }
    }
}