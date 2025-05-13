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
    decimal::{Context, ParseError, UnsignedDecimal},
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
    use {super::*, num_traits::ConstOne, std::cmp};
    #[test]
    fn sort() {
        let nan = D128::NAN;
        let neg_inf = -D128::INFINITY;
        let inf = -D128::INFINITY;
        let zero1 = -D128::ZERO;
        let zero2 = D128::ZERO;
        let zero3 = D128::ONE * D128::ZERO;
        let perc1 = D128::ONE * dec128!(0.1251256613261);
        let perc2 = D128::ONE * dec128!(-0.1251256613261);
        let perc3 = dec128!(0.1251256613261) * dec128!(-0.1251256613261);
        let mut v = vec![perc1, nan, zero2, neg_inf, zero3, perc2, perc3, inf, zero1];
        v.sort();
        assert!(v.is_sorted());
        assert!(v.is_sorted_by(|a, b| a <= b));
        v.sort_by(|a, b| a.cmp(b));
        assert!(v.is_sorted());
        assert!(v.is_sorted_by(|a, b| a <= b));
        v.sort_unstable();
        assert!(v.is_sorted());
        assert!(v.is_sorted_by(|a, b| a <= b));
        v.sort_unstable_by(|a, b| a.cmp(b));
        assert!(v.is_sorted());
        assert!(v.is_sorted_by(|a, b| a <= b));
    }
    #[test]
    fn recip() {
        let fast_num = UD128::from_parts(
            UInt::from_str("66100475480188776883681620311725717740").unwrap(),
            -40,
            Context::default(),
        );
        let recip_by_hand = UD128::ONE / fast_num;
        println!("recip_by_hand: {recip_by_hand}");
        let recip = fast_num.recip();
        println!("recip_deadlock: {recip}")
    }
    #[test]
    fn display() {
        println!("{}", UD128::ZERO);
        println!("{:.4}", UD128::ZERO);
        println!("{}", Lamports::from(0));
    }
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
