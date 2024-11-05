use crate::{Decisol, TokenLamports};
use rust_decimal::Decimal;
use std::fmt::Display;
use std::ops::Sub;
use std::ops::{Add, Div, Mul};
impl TokenLamports {
    pub fn from_decimal(mut amount: Decimal, decimals: u8) -> Self {
        amount.rescale(decimals as u32);
        let amount = amount.mantissa();
        if amount.is_negative() {
            panic!("when trying to conver negative decimal to lamports")
        }
        Self(amount as u64, decimals)
    }
}

impl Add<Decimal> for TokenLamports {
    type Output = Self;

    fn add(self, rhs: Decimal) -> Self::Output {
        let rhs = Self::from_decimal(rhs, self.decimals());
        self + rhs
    }
}
impl Sub<Decimal> for TokenLamports {
    type Output = Self;

    fn sub(self, rhs: Decimal) -> Self::Output {
        let rhs = Self::from_decimal(rhs, self.decimals());
        self - rhs
    }
}

impl Mul<Decimal> for TokenLamports {
    type Output = TokenLamports;
    fn mul(self, mut rhs: Decimal) -> Self::Output {
        if rhs.is_sign_negative() {
            panic!("Multiplying TokenLamports by decimal failed: Decimal is negative");
        }
        let scale = rhs.scale();
        if scale > 9 {
            rhs.rescale(9);
        }
        Self::new(
            ((self.amount() as u128 * rhs.mantissa() as u128) / (10u128.pow(scale))) as u64,
            self.decimals(),
        )
    }
}
impl Div<Decimal> for TokenLamports {
    type Output = TokenLamports;
    fn div(self, mut rhs: Decimal) -> Self::Output {
        if rhs.is_sign_negative() {
            panic!("Multiplying TokenLamports by decimal failed: Decimal is negative");
        }
        let scale = rhs.scale();
        if scale > 9 {
            rhs.rescale(9);
        }
        Self::new(
            (self.amount() as u128 / rhs.mantissa() as u128 * 10u128.pow(scale)) as u64,
            self.decimals(),
        )
    }
}

impl Div<TokenLamports> for u128 {
    type Output = u64;
    fn div(self, rhs: TokenLamports) -> Self::Output {
        (self / rhs.amount() as u128) as u64
    }
}
impl Div for TokenLamports {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.decimals() != rhs.decimals() {
            panic!()
        }
        Self::new(
            self.amount().checked_div(rhs.amount()).unwrap(),
            self.decimals(),
        )
    }
}
impl Display for TokenLamports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dec: Decimal = self.into();
        write!(f, "{}", dec)
    }
}

impl Sub for TokenLamports {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.decimals() != rhs.decimals() {
            panic!()
        }
        Self::new(
            self.amount().checked_sub(rhs.amount()).unwrap(),
            self.decimals(),
        )
    }
}
impl Add for TokenLamports {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.decimals() != rhs.decimals() {
            panic!()
        }
        Self::new(
            self.amount().checked_add(rhs.amount()).unwrap(),
            self.decimals(),
        )
    }
}

impl From<TokenLamports> for Decimal {
    fn from(value: TokenLamports) -> Self {
        Decimal::from_i128_with_scale(value.amount() as i128, value.decimals().into())
    }
}
impl From<&TokenLamports> for Decimal {
    fn from(value: &TokenLamports) -> Self {
        Decimal::from_i128_with_scale(value.amount() as i128, value.decimals().into())
    }
}

impl From<TokenLamports> for u64 {
    fn from(value: TokenLamports) -> Self {
        value.amount()
    }
}

impl Decisol for TokenLamports {
    fn amount(&self) -> u64 {
        self.0
    }

    fn decimals(&self) -> u8 {
        self.1
    }
}
impl<T: Decisol> Mul<T> for TokenLamports {
    type Output = u128;
    fn mul(self, rhs: T) -> Self::Output {
        self.amount() as u128 * rhs.amount() as u128
    }
}
