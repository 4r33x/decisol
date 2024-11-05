use log::{error, info};
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;
use std::ops::Sub;
use std::ops::{Add, Div, Mul, Neg};
pub const LAMPORTS_PER_SOL_DEC: u8 = 9;

pub fn sol_to_u64(val: Decimal) -> Option<u64> {
    let pow: Decimal = Into::<Decimal>::into(10).checked_powu(LAMPORTS_PER_SOL_DEC.into())?;
    let res = val.checked_mul(pow).unwrap().trunc().try_into();
    res.ok()
}

pub fn sol_from_u64(val: u64) -> Decimal {
    Decimal::from_i128_with_scale(val as i128, LAMPORTS_PER_SOL_DEC.into())
}
pub fn token_from_u64(val: u64, dec: u8) -> Decimal {
    Decimal::from_i128_with_scale(val as i128, dec.into())
}

pub fn token_to_u64(val: Decimal, dec: u8) -> u64 {
    let pow = dec!(10).powu(dec.into());
    let res = (val * pow).trunc().try_into();
    match res {
        Ok(v) => v,
        Err(e) => {
            error!(
                "Error while converting to token u64 from decimal {e} value is {}",
                (val * pow).trunc()
            );
            u64::MAX
        }
    }
}
#[derive(Copy, Clone)]
struct TokenLamports<const D: u8> {
    amount: u64,
}

impl<const D: u8> TokenLamports<D> {
    pub const fn new(amount: u64) -> Self {
        Self { amount }
    }
}
#[derive(Copy, Clone)]
struct Lamports {
    amount: u64,
}
impl Lamports {
    const DECIMALS: u8 = 9;
}
impl From<u64> for Lamports {
    fn from(value: u64) -> Self {
        Self { amount: value }
    }
}
impl From<Lamports> for u64 {
    fn from(value: Lamports) -> Self {
        value.amount
    }
}
impl From<Decimal> for Lamports {
    fn from(mut value: Decimal) -> Self {
        value.rescale(Self::DECIMALS as u32);
        let value = value.mantissa();
        if value.is_negative() {
            panic!("when trying to conver negative decimal to lamports")
        }
        Self {
            amount: value as u64,
        }
    }
}
impl From<Lamports> for Decimal {
    fn from(value: Lamports) -> Self {
        Decimal::from_i128_with_scale(value.amount as i128, Lamports::DECIMALS.into())
    }
}
impl<const D: u8> From<u64> for TokenLamports<D> {
    fn from(value: u64) -> Self {
        Self { amount: value }
    }
}
impl<const D: u8> From<TokenLamports<D>> for u64 {
    fn from(value: TokenLamports<D>) -> Self {
        value.amount
    }
}
trait Decisol {
    fn amount(&self) -> u64;
    fn decimals(&self) -> u8;
    fn new(amount: u64) -> Self;
}
impl<const D: u8> Decisol for TokenLamports<D> {
    fn amount(&self) -> u64 {
        self.amount
    }
    fn new(amount: u64) -> Self {
        if D > 9 {
            panic!()
        }
        Self { amount }
    }
    fn decimals(&self) -> u8 {
        D
    }
}
impl Decisol for Lamports {
    fn amount(&self) -> u64 {
        self.amount
    }
    fn new(amount: u64) -> Self {
        Self { amount }
    }
    fn decimals(&self) -> u8 {
        Self::DECIMALS
    }
}

impl<T: Decisol> Mul<T> for Lamports {
    type Output = u128;
    fn mul(self, rhs: T) -> Self::Output {
        self.amount() as u128 * rhs.amount() as u128
    }
}
impl<T: Decisol, const D: u8> Mul<T> for TokenLamports<D> {
    type Output = u128;
    fn mul(self, rhs: T) -> Self::Output {
        self.amount() as u128 * rhs.amount() as u128
    }
}

impl Div for Lamports {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.amount() / rhs.amount())
    }
}
impl Div<Lamports> for u128 {
    type Output = u64;
    fn div(self, rhs: Lamports) -> Self::Output {
        (self / rhs.amount() as u128) as u64
    }
}
impl<const D: u8> Div<TokenLamports<D>> for u128 {
    type Output = u64;
    fn div(self, rhs: TokenLamports<D>) -> Self::Output {
        (self / rhs.amount() as u128) as u64
    }
}
impl<const D: u8> Div for TokenLamports<D> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.amount().checked_div(rhs.amount()).unwrap())
    }
}

impl Add for Lamports {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.amount().checked_add(rhs.amount()).unwrap())
    }
}
impl<const D: u8> Add for TokenLamports<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.amount().checked_add(rhs.amount()).unwrap())
    }
}

impl Sub for Lamports {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.amount().checked_sub(rhs.amount()).unwrap())
    }
}
impl<const D: u8> Sub for TokenLamports<D> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.amount().checked_sub(rhs.amount()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mult() {
        let sol: Lamports = 100.into();
        let token: TokenLamports<9> = 100.into();
        let another_token: TokenLamports<6> = 200.into();
        let liq = token * another_token;
        let liq = sol * token;
        let liq = sol * sol;
        let liq = token * token;
        let amount = sol + sol;
        let amount_zero = another_token - another_token;
    }
}
