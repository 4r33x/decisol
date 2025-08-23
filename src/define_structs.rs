macro_rules! define_structs{
    (
        $( $variant:ident : $decimals:expr ),* $(,)?
    ) => {
        $(
            #[derive(
                Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default,
                serde::Deserialize, serde::Serialize,
            )]
            pub struct $variant(pub u64);

            impl $variant {
                pub const fn new(amount: u64) -> Self {
                    Self(amount)
                }
            }

            impl Decimals for $variant {
                const DECIMALS: u8 = $decimals;
            }
            impl LamportsKind for $variant {
                const KIND: TokenLamportsKind = TokenLamportsKind::$variant;
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
                fn kind(&self) -> TokenLamportsKind {
                    Self::KIND
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
                            Default::default()
                        },
                    }

                    #[cfg(not(feature = "conv_checks"))]
                    Self(value.digits().try_into().unwrap_or_default())

                }
            }

        )*
        #[derive(
            Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
            serde::Deserialize, serde::Serialize,
        )]
        pub enum TokenLamportsKind {
            $(
                $variant,
            )*
        }
        impl TokenLamportsKind {
            pub const fn decimals(&self) -> u8 {
                match self {
                $( TokenLamportsKind::$variant => $variant::DECIMALS, )*
                }
            }
        }
    };
}
