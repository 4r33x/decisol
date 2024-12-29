use rust_decimal::Decimal;

mod lamports;
mod token_lamports;

pub trait Decisol: Into<Decimal> {
    fn amount(&self) -> u64;
    fn decimals(&self) -> u8;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TokenLamports(pub u64, pub u8);
impl TokenLamports {
    const fn new(amount: u64, decimals: u8) -> Self {
        Self(amount, decimals)
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Lamports(pub u64);
impl Lamports {
    pub const DECIMALS: u8 = 9;
    const fn new(amount: u64) -> Self {
        Self(amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mult() {
        let sol: Lamports = 100.into();
        let token = TokenLamports(100, 9);
        let another_token = TokenLamports(100, 6);
        let _liq = token * another_token;
        let _liq = sol * token;
        let _liq = sol * sol;
        let _liq = token * token;
    }
    #[test]
    fn sub() {
        let sol: Lamports = 100.into();
        let _token = TokenLamports(100, 9);
        let another_token = TokenLamports(100, 6);
        let _amount = sol + sol;
        let _amount_zero = another_token - another_token;
    }
    #[test]
    fn add() {
        let sol: Lamports = 100.into();
        let _token = TokenLamports(100, 9);
        let _another_token = TokenLamports(100, 6);
        let _amount = sol + sol;
    }
}
