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
    TokenLamports10: 10,
    TokenLamports11: 11,
    TokenLamports12: 12,
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
    TokenLamports9,
    TokenLamports10,
    TokenLamports11,
    TokenLamports12,
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
    TokenLamports10,
    TokenLamports11,
    TokenLamports12,
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
    use {
        super::*,
        num_traits::ConstOne,
        std::{cmp, cmp::Ordering},
    };
    #[test]
    fn cpm_less_test_f() {
        let max = udec128!(35_000);
        let curr = udec128!(19205.1716384861040546579908080116424889);
        assert!(max > curr);
    }
    #[test]
    #[should_panic]
    fn cpm_less_test() {
        let max = udec128!(35_000);
        let curr = udec128!(19205.1716384861040546579908080116424889);
        assert!(max < curr);
    }
    #[test]
    fn cmp_test() {
        let min = udec128!(0);
        let max = udec128!(35_000);
        let curr = udec128!(19205.1716384861040546579908080116424889);
        //OK
        assert!(min < max);
        //OK
        assert_eq!(min.cmp(&max), Ordering::Less);
        //OK
        assert!(min < curr);
        //OK
        assert_eq!(min.cmp(&curr), Ordering::Less);
        //OK
        assert!(curr < max);
        //OK
        assert_eq!(curr.cmp(&max), Ordering::Less);

        //SHOULD BE OK BUT PANIC INSTEAD
        assert_eq!(max.cmp(&curr), Ordering::Greater);

        //actual case how this was found
        if !max.is_zero() && (min > curr || max < curr) {
            panic!()
        }
    }
    #[test]
    fn convert() {
        let dec = udec128!(0.999405000);
        let lamports: Lamports = dec.into();
        let dec2: UD128 = lamports.into();
        println!("dec: {dec}, lamports: {lamports}, dec2: {dec2}");
        assert_eq!(dec, dec2);
    }

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
    fn display_after_op() {
        // assertion `left == right` failed
        // left: "0"
        // right: "0E-7"
        assert_eq!("0", format!("{:.0}", (UD128::ZERO * udec128!(1.4326236))));
    }
    #[test]
    fn display_after_round() {
        let num = UD128::ONE.round(9);
        //OK
        assert_eq!("1.000000000", num.to_string());
        let num = udec128!(0.00000432).round(9);
        //OK
        assert_eq!("0.000004320", num.to_string());

        //add another zero after dot
        // assertion `left == right` failedq
        // left: "0.000000432"
        // right: "4.32E-7"
        let num = udec128!(0.000000432).round(9);
        assert_eq!("0.000000432", num.to_string());
    }
    #[test]
    fn display_rounding() {
        let dec = udec128!(0.999405000);
        // assertion `left == right` failed
        // left: "0.99"
        // right: "0.10"
        assert_eq!("0.99", format!("{dec:.2}"));
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
