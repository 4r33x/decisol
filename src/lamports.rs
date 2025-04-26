use {
    crate::{Decisol, Lamports, TokenLamports},
    rust_decimal::{Decimal, Error},
    std::{
        fmt::Display,
        ops::{Add, AddAssign, Div, Mul, Sub},
        str::FromStr,
    },
};

impl PartialEq<u64> for Lamports {
    fn eq(&self, other: &u64) -> bool {
        self.amount() == *other
    }
}
impl Sub for Lamports {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.amount().checked_sub(rhs.amount()).unwrap())
    }
}
impl Sub<u64> for Lamports {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self::new(self.amount().checked_sub(rhs).unwrap())
    }
}
impl Sub<Lamports> for u64 {
    type Output = Lamports;

    fn sub(self, rhs: Lamports) -> Self::Output {
        Lamports::new(self.checked_sub(rhs.amount()).unwrap())
    }
}
impl Add<u64> for Lamports {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self::new(self.amount().checked_add(rhs).unwrap())
    }
}
impl Display for Lamports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dec: Decimal = self.into();
        let decimals = f.precision().unwrap_or(4);

        write!(f, "{dec:.decimals$}")
    }
}

impl FromStr for Lamports {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount = s.parse::<Decimal>()?;
        Ok(amount.into())
    }
}

impl From<u64> for Lamports {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<Lamports> for u64 {
    fn from(value: Lamports) -> Self {
        value.amount()
    }
}
impl From<Decimal> for Lamports {
    fn from(mut value: Decimal) -> Self {
        value.rescale(Self::DECIMALS as u32);
        let value = value.mantissa();
        if value.is_negative() {
            panic!("when trying to conver negative decimal to lamports")
        }
        Self(value as u64)
    }
}
impl Div<TokenLamports> for Lamports {
    type Output = Decimal;
    fn div(self, rhs: TokenLamports) -> Self::Output {
        let lhs: Decimal = self.into();
        let rhs: Decimal = rhs.into();
        lhs / rhs
    }
}
impl Div for Lamports {
    type Output = Decimal;
    fn div(self, rhs: Self) -> Decimal {
        let lhs: Decimal = self.into();
        let rhs: Decimal = rhs.into();
        lhs.checked_div(rhs).unwrap()
    }
}
impl From<Lamports> for Decimal {
    fn from(value: Lamports) -> Self {
        Decimal::from_i128_with_scale(value.amount() as i128, Lamports::DECIMALS.into())
    }
}
impl From<&Lamports> for Decimal {
    fn from(value: &Lamports) -> Self {
        Decimal::from_i128_with_scale(value.amount() as i128, Lamports::DECIMALS.into())
    }
}

impl Decisol for Lamports {
    fn amount(&self) -> u64 {
        self.0
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
impl Mul<u64> for Lamports {
    type Output = u128;
    fn mul(self, rhs: u64) -> Self::Output {
        self.amount() as u128 * rhs as u128
    }
}
impl Mul<Lamports> for u64 {
    type Output = u128;
    fn mul(self, rhs: Lamports) -> Self::Output {
        self as u128 * rhs.amount() as u128
    }
}

impl Mul<Decimal> for Lamports {
    type Output = Lamports;
    fn mul(self, mut rhs: Decimal) -> Self::Output {
        if rhs.is_sign_negative() {
            panic!("Multiplying Lamports by decimal failed: Decimal is negative");
        }
        let mut scale = rhs.scale();
        if scale > 9 {
            scale = 9;
            rhs.rescale(scale);
        }
        Self::new(((self.amount() as u128 * rhs.mantissa() as u128) / (10u128.pow(scale))) as u64)
    }
}

// impl Div<Decimal> for Lamports {
//     type Output = Lamports;
//     fn div(self, mut rhs: Decimal) -> Self::Output {
//         if rhs.is_sign_negative() {
//             panic!("Multiplying Lamports by decimal failed: Decimal is negative");
//         }
//         let mut scale = rhs.scale();
//         if scale < 9 {
//             scale = 9;
//             rhs.rescale(9);
//         }
//         Self::new(((self.amount() as u128 * rhs.mantissa() as u128) / (10u128.pow(scale))) as u64)
//     }
// }

impl Div<Lamports> for u128 {
    type Output = u64;
    fn div(self, rhs: Lamports) -> Self::Output {
        self.checked_div(rhs.amount() as u128).unwrap() as u64
    }
}

impl Add for Lamports {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.amount() + rhs.amount())
    }
}
impl AddAssign for Lamports {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
