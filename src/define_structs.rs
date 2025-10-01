macro_rules! define_structs{
    (
        $( $variant:ident : $decimals:expr ),* $(,)?
    ) => {
        $(
            #[derive(
                Copy, Clone, Debug,PartialEq, Eq, PartialOrd, Ord,  Hash,  Default,
                serde::Deserialize, serde::Serialize,
            )]
            pub struct $variant(u64);

            impl $variant {
                pub fn new<T: ValidAmount>(amount: T) -> Self {
                    Self(amount.to_u64())
                }
                pub const fn new_u64(amount: u64) -> Self {
                    Self(amount)
                }
            }

            impl $variant {
                pub const DECIMALS: u8 = $decimals;
                pub const ZERO: Self = Self::new_u64(0u64);
                pub const ONE: Self = Self::new_u64(1u64 * 10u64.pow(Self::DECIMALS as u32));
                pub const TWO: Self = Self::new_u64(2u64 * 10u64.pow(Self::DECIMALS as u32));
                pub const THREE: Self = Self::new_u64(3u64 * 10u64.pow(Self::DECIMALS as u32));
                pub const FOUR: Self = Self::new_u64(4u64 * 10u64.pow(Self::DECIMALS as u32));
                pub const FIVE: Self = Self::new_u64(5u64 * 10u64.pow(Self::DECIMALS as u32));
                pub const TEN: Self = Self::new_u64(10u64 * 10u64.pow(Self::DECIMALS as u32));
                pub const HUNDRED: Self = Self::new_u64(100u64 * 10u64.pow(Self::DECIMALS as u32));
                pub const THOUSAND: Self = Self::new_u64(1_000u64 * 10u64.pow(Self::DECIMALS as u32));
                pub const MILLION: Self = Self::new_u64(1_000_000u64 * 10u64.pow(Self::DECIMALS as u32));
                pub const BILLION: Self = Self::new_u64(1_000_000_000u64 * 10u64.pow(Self::DECIMALS as u32));

                pub const fn zero() -> Self {
                    Self::ZERO
                }

                pub const fn one() -> Self {
                    Self::ONE
                }

                pub const fn two() -> Self {
                    Self::TWO
                }

                pub const fn three() -> Self {
                    Self::THREE
                }

                pub const fn four() -> Self {
                    Self::FOUR
                }

                pub const fn five() -> Self {
                    Self::FIVE
                }

                pub const fn ten() -> Self {
                    Self::TEN
                }
            }

            impl Decisol for $variant {
                fn amount(&self) -> u64 {
                    self.0
                }
                fn amount_mut(&mut self) -> &mut u64 {
                    &mut self.0
                }
                fn decimals(&self) -> u8 {
                    Self::DECIMALS
                }
                fn to_u128(&self) -> u128 {
                    self.amount() as u128
                }
                fn as_u128(&self) -> u128 {
                    self.amount() as u128
                }
                fn with_amount<A: ValidAmount>(&self, amount: A) -> Self {
                    Self::new(amount)
                }
            }
            impl From<u64> for $variant {
                fn from(value: u64) -> Self {
                    Self(value)
                }
            }

            define_common!($variant);
            define_math!($variant);
            define_math_common!($variant);
            impl FromStr for $variant {
                type Err = ParseError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let amount = s.parse::<UD128>()?;
                    Ok(amount.into())
                }
            }
            impl From<UD128> for $variant {
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn from(value: UD128) -> Self {
                    let value = value.trunc_with_scale(Self::DECIMALS as i16);
                    #[cfg(feature = "conv_checks")]
                    match value.digits().try_into() {
                        Ok(v) => Self(v),
                        Err(e) => {
                            conv_fail!($variant, FromUD128, value, e);
                            Self(u64::MAX)
                        },
                    }

                    #[cfg(not(feature = "conv_checks"))]
                    Self(value.digits().try_into().unwrap_or_else(u64::MAX))

                }
            }

        )*
    };
}
