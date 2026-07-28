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

const USD_TO_WSOL_POW_ARG: u8 = Sol::DECIMALS.checked_sub(Usd::DECIMALS).unwrap();
const USD_TO_WSOL_NUM: u64 = 10u64.checked_pow(USD_TO_WSOL_POW_ARG as u32).unwrap();

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
    pub fn to_quote(self) -> Option<QuoteLamportsKind> {
        match self {
            SolanaLamportsKind::TokenLamports0
            | SolanaLamportsKind::TokenLamports1
            | SolanaLamportsKind::TokenLamports2
            | SolanaLamportsKind::TokenLamports3
            | SolanaLamportsKind::TokenLamports4
            | SolanaLamportsKind::TokenLamports5
            | SolanaLamportsKind::TokenLamports6
            | SolanaLamportsKind::TokenLamports7
            | SolanaLamportsKind::TokenLamports8
            | SolanaLamportsKind::TokenLamports9
            | SolanaLamportsKind::TokenLamports10
            | SolanaLamportsKind::TokenLamports11
            | SolanaLamportsKind::TokenLamports12
            | SolanaLamportsKind::TokenLamports13
            | SolanaLamportsKind::TokenLamports14
            | SolanaLamportsKind::TokenLamports15
            | SolanaLamportsKind::TokenLamports16
            | SolanaLamportsKind::TokenLamports17
            | SolanaLamportsKind::TokenLamports18 => None,
            SolanaLamportsKind::Lamports => Some(QuoteLamportsKind::Lamports),
            SolanaLamportsKind::Wsol => Some(QuoteLamportsKind::Wsol),
            SolanaLamportsKind::Usdc => Some(QuoteLamportsKind::Usdc),
            SolanaLamportsKind::Usd1 => Some(QuoteLamportsKind::Usd1),
            SolanaLamportsKind::Usdt => Some(QuoteLamportsKind::Usdt),
        }
    }
    pub fn from_dec(dec: u8) -> Option<Self> {
        match dec {
            0 => Some(SolanaLamportsKind::TokenLamports0),
            1 => Some(SolanaLamportsKind::TokenLamports1),
            2 => Some(SolanaLamportsKind::TokenLamports2),
            3 => Some(SolanaLamportsKind::TokenLamports3),
            4 => Some(SolanaLamportsKind::TokenLamports4),
            5 => Some(SolanaLamportsKind::TokenLamports5),
            6 => Some(SolanaLamportsKind::TokenLamports6),
            7 => Some(SolanaLamportsKind::TokenLamports7),
            8 => Some(SolanaLamportsKind::TokenLamports8),
            9 => Some(SolanaLamportsKind::TokenLamports9),
            10 => Some(SolanaLamportsKind::TokenLamports10),
            11 => Some(SolanaLamportsKind::TokenLamports11),
            12 => Some(SolanaLamportsKind::TokenLamports12),
            13 => Some(SolanaLamportsKind::TokenLamports13),
            14 => Some(SolanaLamportsKind::TokenLamports14),
            15 => Some(SolanaLamportsKind::TokenLamports15),
            16 => Some(SolanaLamportsKind::TokenLamports16),
            17 => Some(SolanaLamportsKind::TokenLamports17),
            18 => Some(SolanaLamportsKind::TokenLamports18),
            _ => None,
        }
    }
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
    pub fn is_lamports(&self) -> bool {
        matches!(self, QuoteLamportsKind::Lamports)
    }
    pub fn is_wsol(&self) -> bool {
        matches!(self, QuoteLamportsKind::Wsol)
    }
    pub fn quote_kind(&self) -> QuoteKind {
        match self {
            Self::Lamports | Self::Wsol => QuoteKind::Sol,
            Self::Usdc | Self::Usd1 | Self::Usdt => QuoteKind::Usd,
        }
    }
}

impl Default for QuoteLamports {
    fn default() -> Self {
        QuoteLamports::Lamports(Lamports::ZERO)
    }
}
impl QuoteLamports {
    pub fn quotes_from_sol_price(&self, sol_price: UD128) -> (Sol, Usd) {
        match self.quote_kind() {
            QuoteKind::Usd => (Sol::from(*self / sol_price), Usd::new(self.amount())),
            QuoteKind::Sol => (Sol::new((*self).amount()), Usd::from(UD128::from(*self * sol_price))),
        }
    }
    pub fn quotes_from_usd_price(&self, usd_price: UD128) -> (Sol, Usd) {
        match self.quote_kind() {
            QuoteKind::Usd => (Sol::from(UD128::from(*self * usd_price)), Usd::new(self.amount())),
            QuoteKind::Sol => (Sol::new((*self).amount()), Usd::from(*self / usd_price)),
        }
    }
    pub fn is_lamports(&self) -> bool {
        matches!(self.kind(), QuoteLamportsKind::Lamports)
    }
    pub fn is_wsol(&self) -> bool {
        matches!(self.kind(), QuoteLamportsKind::Wsol)
    }
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

impl SolanaLamports {
    pub fn to_quote(self) -> Option<QuoteLamports> {
        match self.kind() {
            SolanaLamportsKind::TokenLamports0
            | SolanaLamportsKind::TokenLamports1
            | SolanaLamportsKind::TokenLamports2
            | SolanaLamportsKind::TokenLamports3
            | SolanaLamportsKind::TokenLamports4
            | SolanaLamportsKind::TokenLamports5
            | SolanaLamportsKind::TokenLamports6
            | SolanaLamportsKind::TokenLamports7
            | SolanaLamportsKind::TokenLamports8
            | SolanaLamportsKind::TokenLamports9
            | SolanaLamportsKind::TokenLamports10
            | SolanaLamportsKind::TokenLamports11
            | SolanaLamportsKind::TokenLamports12
            | SolanaLamportsKind::TokenLamports13
            | SolanaLamportsKind::TokenLamports14
            | SolanaLamportsKind::TokenLamports15
            | SolanaLamportsKind::TokenLamports16
            | SolanaLamportsKind::TokenLamports17
            | SolanaLamportsKind::TokenLamports18 => None,
            SolanaLamportsKind::Lamports => Some(QuoteLamports::Lamports(Lamports::new(self.amount()))),
            SolanaLamportsKind::Wsol => Some(QuoteLamports::Wsol(Wsol::new(self.amount()))),
            SolanaLamportsKind::Usdc => Some(QuoteLamports::Usdc(Usdc::new(self.amount()))),
            SolanaLamportsKind::Usd1 => Some(QuoteLamports::Usd1(Usd1::new(self.amount()))),
            SolanaLamportsKind::Usdt => Some(QuoteLamports::Usdt(Usdt::new(self.amount()))),
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

fn usd_to_sol(a: impl Decisol, usd_price: UD128) -> u64 {
    (a * usd_price).amount().saturating_mul(USD_TO_WSOL_NUM)
}
fn sol_to_usd(a: impl Decisol, sol_price: UD128) -> u64 {
    (a * sol_price).amount() / USD_TO_WSOL_NUM
}

// Comparison keys deliberately use the target quote's u64 precision. Values
// that land in the same target-unit bucket must compare equal in every pairing.
#[inline]
fn quote_native_amount(quote: QuoteLamports, usd_price: UD128) -> u64 {
    match quote.quote_kind() {
        QuoteKind::Sol => quote.amount(),
        QuoteKind::Usd => usd_to_sol(quote, usd_price),
    }
}

#[inline]
fn quote_usd_amount(quote: QuoteLamports, sol_price: UD128) -> u64 {
    match quote.quote_kind() {
        QuoteKind::Sol => sol_to_usd(quote, sol_price),
        QuoteKind::Usd => quote.amount(),
    }
}

impl Display for QuoteLamportsKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteLamportsKind::Lamports => f.write_str("SOL"),
            QuoteLamportsKind::Wsol => f.write_str("WSOL"),
            QuoteLamportsKind::Usdc => f.write_str("USDC"),
            QuoteLamportsKind::Usd1 => f.write_str("USD1"),
            QuoteLamportsKind::Usdt => f.write_str("USDT"),
        }
    }
}

impl Display for QuoteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteKind::Sol => f.write_str(" SOL"),
            QuoteKind::Usd => f.write_str("$"),
        }
    }
}

impl QuoteLamportsKind {
    #[inline]
    pub fn normalize<SOL: FnOnce() -> UD128, USD: FnOnce() -> UD128>(&self, quote_in: QuoteLamports, sol_price: SOL, usd_price: USD) -> QuoteLamports {
        match (quote_in.quote_kind(), self.quote_kind()) {
            (QuoteKind::Usd, QuoteKind::Sol) => {
                let v = usd_to_sol(quote_in, usd_price());
                self.value(v)
            }
            (QuoteKind::Sol, QuoteKind::Usd) => {
                let v = sol_to_usd(quote_in, sol_price());
                self.value(v)
            }
            (QuoteKind::Usd, QuoteKind::Usd) | (QuoteKind::Sol, QuoteKind::Sol) => quote_in,
        }
    }
}

#[inline]
pub fn compare_quotes_native<F: FnOnce() -> UD128>(usd_price: F, a: QuoteLamports, b: QuoteLamports) -> std::cmp::Ordering {
    if matches!((a.quote_kind(), b.quote_kind()), (QuoteKind::Sol, QuoteKind::Sol)) {
        return a.amount().cmp(&b.amount());
    }

    let usd_price = usd_price();
    quote_native_amount(a, usd_price).cmp(&quote_native_amount(b, usd_price))
}
#[inline]
pub fn compare_quotes_usd<F: FnOnce() -> UD128>(a: QuoteLamports, b: QuoteLamports, sol_price: F) -> std::cmp::Ordering {
    if matches!((a.quote_kind(), b.quote_kind()), (QuoteKind::Usd, QuoteKind::Usd)) {
        return a.amount().cmp(&b.amount());
    }

    let sol_price = sol_price();
    quote_usd_amount(a, sol_price).cmp(&quote_usd_amount(b, sol_price))
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
    // + Ord
    //+ PartialOrd<Self>
    + PartialOrd<u64>
    + PartialEq<Self>
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

    const QUOTE_SORT_REGRESSION_PATTERN: [Option<u64>; 21] = [
        Some(3),
        Some(5),
        Some(10),
        None,
        Some(4),
        None,
        Some(9),
        Some(6),
        None,
        Some(1),
        Some(9),
        Some(1),
        Some(7),
        Some(5),
        Some(1),
        None,
        Some(8),
        Some(1),
        None,
        Some(3),
        Some(1),
    ];

    fn quote(amount: u64, kind: QuoteLamportsKind) -> QuoteLamports {
        QuoteLamports::new(amount, kind)
    }

    fn quote_sort_regression_values(primary_kind: QuoteLamportsKind, zero_kind: QuoteLamportsKind) -> Vec<QuoteLamports> {
        QUOTE_SORT_REGRESSION_PATTERN
            .map(|amount| match amount {
                Some(amount) => quote(amount, primary_kind),
                None => quote(0, zero_kind),
            })
            .to_vec()
    }

    fn assert_cmp_pair<F>(cmp: &F, a: QuoteLamports, b: QuoteLamports, expected: Ordering)
    where
        F: Fn(QuoteLamports, QuoteLamports) -> Ordering,
    {
        assert_eq!(cmp(a, b), expected);
        assert_eq!(cmp(b, a), expected.reverse());
    }

    #[test]
    fn compare_quotes_usd_uses_consistent_target_precision() {
        let cmp = |a, b| compare_quotes_usd(a, b, || udec128!(100));
        let lamports_10 = quote(10, QuoteLamportsKind::Lamports);
        let lamports_11 = quote(11, QuoteLamportsKind::Lamports);
        let lamports_20 = quote(20, QuoteLamportsKind::Lamports);
        let usdc_1 = quote(1, QuoteLamportsKind::Usdc);

        assert_cmp_pair(&cmp, lamports_10, usdc_1, Ordering::Equal);
        assert_cmp_pair(&cmp, lamports_11, usdc_1, Ordering::Equal);
        assert_cmp_pair(&cmp, lamports_10, lamports_11, Ordering::Equal);
        assert_cmp_pair(&cmp, lamports_20, usdc_1, Ordering::Greater);
    }

    #[test]
    fn compare_quotes_native_uses_consistent_target_precision() {
        let cmp = |a, b| compare_quotes_native(|| udec128!(0.1), a, b);
        let usdc_10 = quote(10, QuoteLamportsKind::Usdc);
        let usdc_11 = quote(11, QuoteLamportsKind::Usdc);
        let usdc_20 = quote(20, QuoteLamportsKind::Usdc);
        let lamports_1_000 = quote(1_000, QuoteLamportsKind::Lamports);

        assert_cmp_pair(&cmp, usdc_10, lamports_1_000, Ordering::Equal);
        assert_cmp_pair(&cmp, usdc_11, lamports_1_000, Ordering::Equal);
        assert_cmp_pair(&cmp, usdc_10, usdc_11, Ordering::Equal);
        assert_cmp_pair(&cmp, usdc_20, lamports_1_000, Ordering::Greater);
    }

    #[test]
    fn compare_quotes_usd_sort_regression() {
        let mut values = quote_sort_regression_values(QuoteLamportsKind::Lamports, QuoteLamportsKind::Usdc);
        let cmp = |a: &QuoteLamports, b: &QuoteLamports| compare_quotes_usd(*a, *b, || udec128!(100));

        values.sort_by(cmp);
        assert!(values.windows(2).all(|pair| cmp(&pair[0], &pair[1]) != Ordering::Greater));
    }

    #[test]
    fn compare_quotes_native_sort_regression() {
        let mut values = quote_sort_regression_values(QuoteLamportsKind::Usdc, QuoteLamportsKind::Lamports);
        let cmp = |a: &QuoteLamports, b: &QuoteLamports| compare_quotes_native(|| udec128!(0.1), *a, *b);

        values.sort_by(cmp);
        assert!(values.windows(2).all(|pair| cmp(&pair[0], &pair[1]) != Ordering::Greater));
    }

    #[test]
    fn mul_ud128_handles_negative_scale() {
        let amount = Lamports::new(2_u64);
        let result = amount * udec128!(5e9);

        assert_eq!(result.amount(), 10_000_000_000);
    }

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
