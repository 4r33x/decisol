use {
    crate::{Decimals, Decisol, Lamports, TokenLamports},
    fastnum::{
        U128, UD128,
        decimal::{Context, ParseError},
        udec128,
    },
    std::{
        fmt::Display,
        ops::{Div, Mul},
        str::FromStr,
        u64,
    },
};

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
