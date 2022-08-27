use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Fraction {
    pub numerator: i128,
    pub denominator: i128,
}

impl Fraction {
    pub fn quotient(&self) -> i128 {
        self.numerator.div(self.denominator)
    }

    pub fn remainder(&self) -> Fraction {
        Fraction { numerator: self.numerator % &self.denominator, denominator: self.denominator }
    }

    pub fn invert(&self) -> Fraction {
        Fraction { numerator: self.denominator, denominator: self.numerator }
    }

    pub fn add(&self, other: Fraction) -> Fraction {
        if self.denominator.eq(&other.denominator) {
            return Fraction { numerator: self.numerator.add(other.numerator), denominator: self.denominator };
        }

        Fraction {
            numerator: self.numerator.mul(other.denominator).add(self.denominator.mul(other.numerator)),
            denominator: self.denominator.mul(other.denominator),
        }
    }

    pub fn substract(&self, other: Fraction) -> Fraction {
        if self.denominator.eq(&other.denominator) {
            return Fraction { numerator: self.numerator.sub(other.numerator), denominator: self.denominator };
        }

        Fraction {
            numerator: self.numerator.mul(other.denominator).sub(self.denominator.mul(other.numerator)),
            denominator: self.denominator.mul(other.denominator),
        }
    }

    pub fn less_than(&self, other: Fraction) -> bool {
        self.numerator.mul(other.denominator).lt(&other.numerator.mul(self.denominator))
    }

    pub fn equal_to(&self, other: Fraction) -> bool {
        self.numerator.mul(other.denominator).eq(&other.numerator.mul(self.denominator))
    }

    pub fn greater_than(&self, other: Fraction) -> bool {
        self.numerator.mul(other.denominator).gt(&other.numerator.mul(self.denominator))
    }

    pub fn multiply(&self, other: &Fraction) -> Fraction {
        Fraction {
            numerator: self.numerator.mul(other.numerator),
            denominator: self.denominator.mul(other.denominator),
        }
    }

    pub fn divide(&self, other: Fraction) -> Fraction {
        Fraction {
            numerator: self.numerator.mul(other.denominator),
            denominator: self.denominator.mul(other.numerator),
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator &&
            self.denominator == other.denominator
    }
}

#[cfg(test)]
mod tests {
    use crate::fraction::Fraction;

    #[tokio::test]
    async fn quotient() {
        assert_eq!(Fraction {
            numerator: 8,
            denominator: 3,
        }.quotient(), 2_i128, "Quotient should have returned 2");
        assert_eq!(Fraction {
            numerator: 12,
            denominator: 4,
        }.quotient(), 3_i128, "Quotient should have returned 2");
        assert_eq!(Fraction {
            numerator: 16,
            denominator: 5,
        }.quotient(), 3_i128, "Quotient should have returned 2");
    }

    #[tokio::test]
    async fn remainder() {
        assert_eq!(Fraction {
            numerator: 8,
            denominator: 3,
        }.remainder(), Fraction {
            numerator: 2_i128,
            denominator: 3_i128,
        }, "Quotient should have returned 2");
        assert_eq!(Fraction {
            numerator: 12,
            denominator: 4,
        }.remainder(), Fraction {
            numerator: 0_i128,
            denominator: 4_i128,
        }, "Quotient should have returned 2");
        assert_eq!(Fraction {
            numerator: 16,
            denominator: 5,
        }.remainder(), Fraction {
            numerator: 1_i128,
            denominator: 5_i128,
        }, "Quotient should have returned 2");
    }

    #[tokio::test]
    async fn invert() {
        assert_eq!(Fraction {
            numerator: 8,
            denominator: 3,
        }.invert().denominator, 8_i128, "Quotient should have returned 2");
        assert_eq!(Fraction {
            numerator: 12,
            denominator: 4,
        }.invert().numerator, 4_i128, "Quotient should have returned 2");
    }

    #[tokio::test]
    async fn add_with_fraction_having_different_nums() {
        assert_eq!(Fraction {
            numerator: 8,
            denominator: 4,
        }.add(Fraction {
            numerator: 8,
            denominator: 3,
        }), Fraction {
            numerator: 56,
            denominator: 12,
        }, "Quotient should have returned 2");
    }

    #[tokio::test]
    async fn add_with_fraction_having_same_nums() {
        assert_eq!(Fraction {
            numerator: 8,
            denominator: 2,
        }.add(Fraction {
            numerator: 8,
            denominator: 3,
        }), Fraction {
            numerator: 56,
            denominator: 2,
        }, "Quotient should have returned 2");
    }

    #[tokio::test]
    async fn substract_with_fraction_having_different_nums() {
        assert_eq!(Fraction {
            numerator: 1,
            denominator: 10,
        }.substract(Fraction {
            numerator: 4,
            denominator: 12,
        }), Fraction {
            numerator: -28,
            denominator: 120,
        }, "Quotient should have returned 2");
    }

    #[tokio::test]
    async fn sub_with_fraction_having_same_nums() {
        assert_eq!(Fraction {
            numerator: 3,
            denominator: 5,
        }.substract(Fraction {
            numerator: 2,
            denominator: 5,
        }), Fraction {
            numerator: 1,
            denominator: 5,
        }, "Quotient should have returned 2");
    }

    #[tokio::test]
    async fn less_than() {
        assert_eq!(Fraction {
            numerator: 1,
            denominator: 10,
        }.less_than(Fraction {
            numerator: 4,
            denominator: 12,
        }), true, "Quotient should have returned 2");

        assert_eq!(Fraction {
            numerator: 1,
            denominator: 3,
        }.less_than(Fraction {
            numerator: 4,
            denominator: 12,
        }), false, "Quotient should have returned 2");

        assert_eq!(Fraction {
            numerator: 5,
            denominator: 12,
        }.less_than(Fraction {
            numerator: 4,
            denominator: 12,
        }), false, "Quotient should have returned 2");
    }

    #[tokio::test]
    async fn greater_than() {
        assert_eq!(Fraction {
            numerator: 1,
            denominator: 10,
        }.greater_than(Fraction {
            numerator: 4,
            denominator: 12,
        }), false, "Quotient should have returned 2");

        assert_eq!(Fraction {
            numerator: 1,
            denominator: 3,
        }.greater_than(Fraction {
            numerator: 4,
            denominator: 12,
        }), false, "Quotient should have returned 2");

        assert_eq!(Fraction {
            numerator: 5,
            denominator: 12,
        }.greater_than(Fraction {
            numerator: 4,
            denominator: 12,
        }), true, "Quotient should have returned 2");
    }

    #[tokio::test]
    async fn multiply() {
        assert_eq!(Fraction {
            numerator: 1,
            denominator: 10,
        }.multiply(&Fraction {
            numerator: 4,
            denominator: 12,
        }), Fraction {
            numerator: 4,
            denominator: 120,
        }, "Quotient should have returned 2");

        assert_eq!(Fraction {
            numerator: 1,
            denominator: 3,
        }.multiply(&Fraction {
            numerator: 4,
            denominator: 12,
        }), Fraction {
            numerator: 4,
            denominator: 36,
        }, "Quotient should have returned 2");

        assert_eq!(Fraction {
            numerator: 5,
            denominator: 12,
        }.multiply(&Fraction {
            numerator: 4,
            denominator: 12,
        }), Fraction {
            numerator: 20,
            denominator: 144,
        }, "Quotient should have returned 2");
    }

    #[tokio::test]
    async fn divide() {
        assert_eq!(Fraction {
            numerator: 1,
            denominator: 10,
        }.divide(Fraction {
            numerator: 4,
            denominator: 12,
        }), Fraction {
            numerator: 12,
            denominator: 40,
        }, "Quotient should have returned 2");

        assert_eq!(Fraction {
            numerator: 1,
            denominator: 3,
        }.divide(Fraction {
            numerator: 4,
            denominator: 12,
        }), Fraction {
            numerator: 12,
            denominator: 12,
        }, "Quotient should have returned 2");

        assert_eq!(Fraction {
            numerator: 5,
            denominator: 12,
        }.divide(Fraction {
            numerator: 4,
            denominator: 12,
        }), Fraction {
            numerator: 60,
            denominator: 48,
        }, "Quotient should have returned 2");
    }
}