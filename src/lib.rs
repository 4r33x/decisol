#![allow(unreachable_code)]
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
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::SubAssign;
use std::str::FromStr;

pub use fastnum::D128;
pub use fastnum::UD128;
pub use fastnum::bint::UInt;
pub use fastnum::dec128;
pub use fastnum::decimal::Context;
pub use fastnum::decimal::ParseError;
pub use fastnum::decimal::UnsignedDecimal;
pub use fastnum::udec128;
use log::error;
#[cfg(feature = "proto")]
use proto_rs::proto_message;
#[cfg(feature = "proto")]
pub mod proto;

define_structs! {
    TokenLamports0: 0, Base;
    TokenLamports1: 1, Base;
    TokenLamports2: 2, Base;
    TokenLamports3: 3, Base;
    TokenLamports4: 4, Base;
    TokenLamports5: 5, Base;
    TokenLamports6: 6, Base;
    TokenLamports7: 7, Base;
    TokenLamports8: 8, Base;
    TokenLamports9: 9, Base;
    TokenLamports10: 10, Base;
    TokenLamports11: 11, Base;
    TokenLamports12: 12, Base;
    TokenLamports13: 13, Base;
    TokenLamports14: 14, Base;
    TokenLamports15: 15, Base;
    TokenLamports16: 16, Base;
    TokenLamports17: 17, Base;
    TokenLamports18: 18, Base;
    Lamports: 9, Base, Quote;
    Wsol: 9, Base, Quote;
    Usdc: 6, Base, Quote;
    Usd1: 6, Base, Quote;
    Usdt: 6, Base, Quote;
    Usd: 6;
    Sol: 9;
}

define_enum! {
    TokenLamports, TokenLamportsKind,
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
    TokenLamports13,
    TokenLamports14,
    TokenLamports15,
    TokenLamports16,
    TokenLamports17,
    TokenLamports18,
}
define_enum! {
    SolanaLamports,  SolanaLamportsKind,
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
    TokenLamports13,
    TokenLamports14,
    TokenLamports15,
    TokenLamports16,
    TokenLamports17,
    TokenLamports18,
    Lamports,
    Wsol,
    Usdc,
    Usd1,
    Usdt,

}
impl From<QuoteLamportsKind> for SolanaLamportsKind {
    fn from(value: QuoteLamportsKind) -> Self {
        match value {
            QuoteLamportsKind::Lamports => Self::Lamports,
            QuoteLamportsKind::Wsol => Self::Wsol,
            QuoteLamportsKind::Usdc => Self::Usdc,
            QuoteLamportsKind::Usd1 => Self::Usd1,
            QuoteLamportsKind::Usdt => Self::Usdt,
        }
    }
}
impl From<QuoteLamports> for SolanaLamports {
    fn from(value: QuoteLamports) -> Self {
        Self::new(value.amount(), value.kind())
    }
}
impl SolanaLamportsKind {
    pub fn is_quote(&self) -> bool {
        match self {
            Self::TokenLamports0
            | Self::TokenLamports1
            | Self::TokenLamports2
            | Self::TokenLamports3
            | Self::TokenLamports4
            | Self::TokenLamports5
            | Self::TokenLamports6
            | Self::TokenLamports7
            | Self::TokenLamports8
            | Self::TokenLamports9
            | Self::TokenLamports10
            | Self::TokenLamports11
            | Self::TokenLamports12
            | Self::TokenLamports13
            | Self::TokenLamports14
            | Self::TokenLamports15
            | Self::TokenLamports16
            | Self::TokenLamports17
            | Self::TokenLamports18 => false,
            Self::Lamports | Self::Wsol | Self::Usdc | Self::Usd1 | Self::Usdt => true,
        }
    }
    pub fn quote_kind(&self) -> Option<QuoteKind> {
        match self {
            Self::TokenLamports0
            | Self::TokenLamports1
            | Self::TokenLamports2
            | Self::TokenLamports3
            | Self::TokenLamports4
            | Self::TokenLamports5
            | Self::TokenLamports6
            | Self::TokenLamports7
            | Self::TokenLamports8
            | Self::TokenLamports9
            | Self::TokenLamports10
            | Self::TokenLamports11
            | Self::TokenLamports12
            | Self::TokenLamports13
            | Self::TokenLamports14
            | Self::TokenLamports15
            | Self::TokenLamports16
            | Self::TokenLamports17
            | Self::TokenLamports18 => None,
            Self::Lamports | Self::Wsol => Some(QuoteKind::Sol),
            Self::Usdc | Self::Usd1 | Self::Usdt => Some(QuoteKind::Usd),
        }
    }
}
impl QuoteLamportsKind {
    pub fn quote_kind(&self) -> QuoteKind {
        match self {
            Self::Lamports | Self::Wsol => QuoteKind::Sol,
            Self::Usdc | Self::Usd1 | Self::Usdt => QuoteKind::Usd,
        }
    }
}
impl QuoteLamports {
    pub fn quote_kind(&self) -> QuoteKind {
        match self {
            Self::Lamports(_) | Self::Wsol(_) => QuoteKind::Sol,
            Self::Usdc(_) | Self::Usd1(_) | Self::Usdt(_) => QuoteKind::Usd,
        }
    }
}
pub enum QuoteKind {
    Usd,
    Sol,
}
impl QuoteKind {
    pub const fn decimals(&self) -> u8 {
        match self {
            QuoteKind::Usd => 6,
            QuoteKind::Sol => 9,
        }
    }
}

define_enum! {
    QuoteLamports,  QuoteLamportsKind,
    Lamports,
    Wsol,
    Usdc,
    Usd1,
    Usdt,
}

pub trait Quote {
    fn kind(&self) -> QuoteLamportsKind;
}
pub trait Base {
    fn kind(&self) -> SolanaLamportsKind;
}

pub trait Decisol:
    Clone
    + Copy
    + Sub<u64, Output = Self>
    + Add<u64, Output = Self>
    + PartialEq<u64>
    + Eq
    + Ord
    + PartialOrd<u64>
    + PartialEq<Self>
    + PartialOrd<Self>
    + Sub<Self, Output = Self>
    + Add<Self, Output = Self>
    + Display
    + std::hash::Hash
    + Debug
    + Into<UD128>
    + Into<D128>
    + Mul<UD128, Output = Self>
{
    fn div_ceil(self, r: UD128) -> Self;
    fn amount(&self) -> u64;
    fn amount_mut(&mut self) -> &mut u64;
    fn decimals(&self) -> u8;
    fn to_u128(&self) -> u128;
    fn as_u128(&self) -> u128;
    fn with_amount<A: ValidAmount>(&self, amount: A) -> Self;
}

pub trait ValidAmount: Copy {
    fn to_u64(self) -> u64;
    fn to_u128(self) -> u128;
}

// Implement for u64
impl ValidAmount for u64 {
    #[inline(always)]
    fn to_u64(self) -> u64 {
        self
    }
    #[inline(always)]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

// Implement for u128
impl ValidAmount for u128 {
    #[cfg_attr(feature = "track_caller", track_caller)]
    fn to_u64(self) -> u64 {
        #[cfg(feature = "overflow_errors")]
        {
            match self.try_into() {
                Ok(v) => v,
                Err(_) => {
                    overflow!(ValidAmount, to_u64, self, self);
                    u64::MAX
                }
            }
        }
        #[cfg(not(feature = "overflow_errors"))]
        {
            return self as u64;
        }
    }
    #[inline(always)]
    fn to_u128(self) -> u128 {
        self
    }
}
#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use fastnum::bint::UInt;

    use super::*;
    #[test]
    fn test_rust_decimal() {
        let rust_dec = rust_decimal::Decimal::from_f64_retain(0.999405000).unwrap();
        assert_eq!("0.99", format!("{rust_dec:.2}"));
    }

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
        let fast_num = UD128::from_parts(UInt::from_str("66100475480188776883681620311725717740").unwrap(), -40, Context::default());
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
        //assert_eq!("1.00", format!("{dec:.2}"));
    }
    #[test]
    fn conv() {
        const LAMPORTS_MAX_DECIMAL: UD128 = udec128!(18446744073.11111111111111111111111);
        let res = Lamports::from(LAMPORTS_MAX_DECIMAL);
        println!("{res}");
    }
    #[cfg(feature = "conv_panics")]
    #[should_panic]
    #[test]
    fn conv_should_panic() {
        let decimal_max = UD128::MAX;
        let _v = Lamports::from(decimal_max);
    }

    #[test]
    fn digits() {
        let e = udec128!(5e9);

        let digits = e.digits();
        let fractional = e.fractional_digits_count();
        assert_eq!(5, digits.to_u128().unwrap());
        assert_eq!(-9, fractional)
    }

    #[test]
    fn mult() {
        let sol: Lamports = 100.into();
        let token = TokenLamports::new_from_decimals(100u64, 9);
        let another_token = TokenLamports::new_from_decimals(100u64, 6);
        let _liq = token * another_token;
        let _liq = sol * token;
        let _liq = sol * sol;
        let _liq = token * token;
    }

    #[test]
    fn div_recip() {
        let sol: Lamports = Lamports::new(Lamports::ten() * 1000);
        let div = sol.div_ceil((udec128!(5000) / udec128!(10000)).recip());
        assert_eq!(20_000_000_000_000, div.amount());
    }
    #[test]
    fn mult2() {
        //amount_out 267800158, swap_fee 2678002, cf 133901
        let amount_out = TokenLamports::new_from_decimals(267800158u64, 9);
        let swap_fee = 2678001;
        let cf = 133900;
        let swap_fee_res = amount_out * udec128!(0.0100);
        let cf_res = amount_out * udec128!(0.0005);
        assert_eq!(swap_fee_res.amount(), swap_fee);
        assert_eq!(cf_res.amount(), cf)
    }
    #[test]
    fn mult3() {
        //amount_out 267800158, swap_fee 2678002, cf 133901
        let amount_out = TokenLamports::new_from_decimals(u64::MAX, 9);
        let coef = udec128!(0.99999999999999999999999999999999999999);
        let _res = amount_out * coef;
    }
    #[test]
    fn div_ceil2() {
        //amount_out 267800158, swap_fee 2678002, cf 133901
        let amount_out = TokenLamports::new_from_decimals(u64::MAX, 9);
        let coef = udec128!(0.99999999999999999999999999999999999999);
        let _res = amount_out.div_ceil(coef);
    }
    #[test]
    fn div_ceil() {
        //amount_out 267800158, swap_fee 2678002, cf 133901
        let amount_out = TokenLamports::new_from_decimals(267800158u64, 9);
        let swap_fee = 2678002;
        let cf = 133901;
        let swap_fee_res = amount_out.div_ceil(udec128!(0.0100));
        let cf_res = amount_out.div_ceil(udec128!(0.0005));
        assert_eq!(swap_fee_res, swap_fee);
        assert_eq!(cf_res, cf)
    }

    #[test]
    fn sub() {
        let sol: Lamports = 100.into();
        let _token = TokenLamports::new_from_decimals(100u64, 9);
        let another_token = TokenLamports::new_from_decimals(100u64, 6);
        let _amount = sol + sol;
        let _amount_zero = another_token - another_token;
    }
    #[cfg(feature = "conv_panics")]
    #[should_panic]
    #[test]
    fn sub_should_panic() {
        let _should_panic = Lamports::new(1u64) - Lamports::new(10u64);
    }
    #[test]
    fn add() {
        let sol: Lamports = 100.into();
        let _token = TokenLamports::new_from_decimals(100u64, 9);
        let _another_token = TokenLamports::new_from_decimals(100u64, 6);
        let _amount = sol + sol;
    }
    #[test]
    fn rescale() {
        let r = udec128!(321.123);
        assert_eq!(udec128!(321.123), r.rescale(3));
        assert_eq!(udec128!(321.12), r.rescale(2));
        assert_eq!(udec128!(321.1230), r.rescale(4));
    }
}
