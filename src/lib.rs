mod lamports;
mod token_lamports;
#[macro_use]
mod define_structs;
#[macro_use]
mod define_math;
#[macro_use]
mod define_math_common;
#[macro_use]
mod define_math_enum;
#[macro_use]
mod define_enum;
#[macro_use]
mod define_common;
#[macro_use]
mod overflow;
#[macro_use]
mod conv_fail;
pub use fastnum::{
    D128, UD128, dec128,
    decimal::{Context, ParseError},
    int::UInt,
    udec128,
};
use {
    log::error,
    std::{
        fmt::Display,
        ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
        str::FromStr,
    },
};
define_structs! {
    TokenLamports0: 0,
    TokenLamports1: 1,
    TokenLamports2: 2,
    TokenLamports3: 3,
    TokenLamports4: 4,
    TokenLamports5: 5,
    TokenLamports6: 6,
    TokenLamports7: 7,
    TokenLamports8: 8,
    TokenLamports9: 9,
    Lamports: 9,
    Usdc: 6,
    Usdt: 6,
    Usd: 6,
}
define_enum! {
    TokenLamports,
    TokenLamports0,
    TokenLamports1,
    TokenLamports2,
    TokenLamports3,
    TokenLamports4,
    TokenLamports5,
    TokenLamports6,
    TokenLamports7,
    TokenLamports8,
    TokenLamports9
}
define_enum! {
    SolanaLamports,
    TokenLamports0,
    TokenLamports1,
    TokenLamports2,
    TokenLamports3,
    TokenLamports4,
    TokenLamports5,
    TokenLamports6,
    TokenLamports7,
    TokenLamports8,
    TokenLamports9,
    Lamports,
    Usdc,
    Usdt,
    Usd,
}
define_enum! {
    QuoteLamports,
    Lamports,
    Usd,
    Usdc,
    Usdt,
}
pub trait LamportsKind {
    const KIND: TokenLamportsKind;
}
pub trait Decisol {
    fn amount(&self) -> u64;
    fn decimals(&self) -> u8;
    fn kind(&self) -> TokenLamportsKind;
}
pub trait Decimals {
    const DECIMALS: u8;
}

#[cfg(test)]
mod tests {
    use {super::*, fastnum::UD128};
    #[test]
    fn conv() {
        const LAMPORTS_MAX_DECIMAL: UD128 = udec128!(18446744073.11111111111111111111111);
        let decimal_max = UD128::MAX;
        let res = Lamports::from(LAMPORTS_MAX_DECIMAL);
        println!("{res}")
        //Lamports::from(decimal_max);
    }

    #[test]
    fn mult() {
        let sol: Lamports = 100.into();
        let token = TokenLamports::new(100, 9);
        let another_token = TokenLamports::new(100, 6);
        let _liq = token * another_token;
        let _liq = sol * token;
        let _liq = sol * sol;
        let _liq = token * token;
    }
    #[test]
    fn sub() {
        let sol: Lamports = 100.into();
        let _token = TokenLamports::new(100, 9);
        let another_token = TokenLamports::new(100, 6);
        let _amount = sol + sol;
        let _amount_zero = another_token - another_token;
        let _should_panic = Lamports::new(1) - Lamports::new(10);
    }
    #[test]
    fn add() {
        let sol: Lamports = 100.into();
        let _token = TokenLamports::new(100, 9);
        let _another_token = TokenLamports::new(100, 6);
        let _amount = sol + sol;
    }
}
