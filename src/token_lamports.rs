use {
    crate::{Decisol, TokenLamports},
    rust_decimal::Decimal,
    std::{
        fmt::Display,
        ops::{Add, AddAssign, Div, Mul, Sub},
    },
};
impl TokenLamports {
    pub fn to_decimal(&self) -> Decimal {
        Decimal::from_i128_with_scale(self.amount() as i128, self.decimals().into())
    }
    // pub fn from_decimal(amount: Decimal, decimals: u8) -> Self {
    //     let amount: u64 = (amount * Decimal::from(10u64.pow(decimals as u32)))
    //         .trunc()
    //         .try_into()
    //         .unwrap();
    //     Self(amount, decimals)
    // }
    pub fn from_decimal(mut amount: Decimal, decimals: u8) -> Self {
        if amount.is_sign_negative() {
            panic!("When trying to convert negative decimal to lamports in fn from_decimal")
        }
        amount.rescale(decimals as u32);
        let amount = amount.mantissa();
        //println!("{amount}");
        Self::new(amount as u64, decimals)
    }
}

// impl Add<Decimal> for TokenLamports {
//     type Output = Self;

//     fn add(self, rhs: Decimal) -> Self::Output {
//         let rhs = Self::from_decimal(rhs, self.decimals());
//         self + rhs
//     }
// }
// impl Sub<Decimal> for TokenLamports {
//     type Output = Self;

//     fn sub(self, rhs: Decimal) -> Self::Output {
//         let rhs = Self::from_decimal(rhs, self.decimals());
//         self - rhs
//     }
// }

impl Mul<Decimal> for TokenLamports {
    type Output = TokenLamports;
    fn mul(self, mut rhs: Decimal) -> Self::Output {
        if rhs.is_sign_negative() {
            panic!("Multiplying TokenLamports by decimal failed: Decimal is negative");
        }
        let mut scale = rhs.scale();
        if scale > 9 {
            scale = 9;
            rhs.rescale(scale);
        }
        //let non_dec = rhs
        Self::new(
            ((self.amount() as u128 * rhs.mantissa() as u128) / (10u128.pow(scale))) as u64,
            self.decimals(),
        )
    }
}

// impl Div<Decimal> for TokenLamports {
//     type Output = TokenLamports;
//     fn div(self, mut rhs: Decimal) -> Self::Output {
//         if rhs.is_sign_negative() {
//             panic!("Multiplying TokenLamports by decimal failed: Decimal is negative");
//         }
//         let mut scale = rhs.scale();
//         if scale < 9 {
//             scale = 9;
//             rhs.rescale(9);
//         }
//         Self::new(
//             (self.amount() as u128 / rhs.mantissa() as u128 * 10u128.pow(scale)) as u64,
//             self.decimals(),
//         )
//     }
// }

impl Div for TokenLamports {
    type Output = Decimal;

    fn div(self, rhs: Self) -> Self::Output {
        let lhs: Decimal = self.into();
        let rhs: Decimal = rhs.into();
        lhs.checked_div(rhs).unwrap()
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

#[cfg(test)]
mod tests {
    use {
        crate::{Decisol, TokenLamports},
        rust_decimal_macros::dec,
    };

    #[test]
    fn token_lamports_mult_test() {
        let tokens = TokenLamports::new(999999999032878, 6);
        let price = dec!(0.0000007562495021987651776628);
        println!("Price mantissa {}", price.mantissa());
        //let res = tokens * price;
        //println!("Res {res}");
        let tokens = TokenLamports::new(1_000_000_000_000_000, 6);
        let price_in_sol = dec!(0.0000416618051001150689138382);
        let sol_price = dec!(240.02800589099727313714931554);
        //let mc = tokens * price_in_sol * sol_price;
        //println!("MC {mc:.0}");
    }
    #[test]
    fn token_lamports_conv_test() {
        let price = dec!(0.00007562495021987651776628);
        let res: TokenLamports = TokenLamports::from_decimal(price, 6);
        println!("Res {}", res.amount());
    }
}
