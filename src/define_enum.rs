macro_rules! define_enum {
    (
        $enum_name:ident, $kind_name:ident, $( $variant:ident ),* $(,)?
    ) => {

        $(
        impl From<$variant> for $enum_name {
            fn from(value: $variant) -> Self {
                Self::$variant(value)
            }
        }
        )*

        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
            serde::Deserialize, serde::Serialize,
        )]
        pub enum $enum_name {
            $(
                $variant($variant),
            )*
        }

        #[derive(
            Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
            serde::Deserialize, serde::Serialize,
        )]
        pub enum $kind_name {
            $(
                $variant,
            )*
        }

        impl $kind_name {
            pub const fn decimals(&self) -> u8 {
                match self {
                    $( $kind_name ::$variant => $variant::DECIMALS, )*
                }
            }
            pub fn value<A: ValidAmount>(&self, value: A) -> $enum_name {
                $enum_name::new(value, *self)
            }
        }

        impl $enum_name {
            pub const fn kind(&self) -> $kind_name  {
                match self {
                    $( $enum_name::$variant(_) => $kind_name ::$variant, )*
                }
            }
            #[cfg_attr(feature = "track_caller", track_caller)]
            pub fn new<A: ValidAmount, T: Into<$kind_name>>(amount: A, kind: T) -> Self {
                let amount = amount.to_u64();
                let kind = kind.into();
                match kind {
                    $(
                        $kind_name ::$variant => Self::$variant($variant(amount)),
                    )*
                    #[allow(unreachable_patterns)]
                    _ => {
                        let loc = std::panic::Location::caller();
                        panic!("{}::new failed at {}:{}:{} - invalid kind: {:?}", stringify!($enum_name), loc.file(),
                        loc.line(), loc.column(), kind)
                    },
                }
            }
            #[cfg_attr(feature = "track_caller", track_caller)]
            pub fn from_udec(value: UD128, kind: $kind_name) -> Self {
                let decimals = kind.decimals();
                let value = value.trunc_with_scale(decimals as i16);
                #[cfg(feature = "conv_checks")]
                match value.digits().try_into() {
                    Ok(v) => Self::new::<u64, $kind_name>(v, kind),
                    Err(e) => {
                        conv_fail!($enum_name, FromUD128, value, e);
                        Self::new::<u64, $kind_name>(0, kind)
                    },
                }
                #[cfg(not(feature = "conv_checks"))]
                unsafe{Self::new(value.digits().try_into().unwrap_unchecked(), kind)}
            }

            #[cfg_attr(feature = "track_caller", track_caller)]
            pub fn new_from_decimals<T: ValidAmount>(amount: T, decimals: u8) -> Self {
                let amount = amount.to_u64();
                #[allow(unreachable_patterns)]
                match decimals {
                    $(
                        $variant::DECIMALS => Self::$variant($variant(amount)),
                    )*
                    _ => {
                        #[cfg(feature = "overflow_errors")]
                        {
                            conv_fail!(TokenLamports, TokenLamportsNew, decimals, format!("Failed due to invalid decimals, with inner value: {amount}"));
                        }
                        #[cfg(feature = "overflow_panics")]
                        {
                            let loc = std::panic::Location::caller();
                            panic!("{}::new failed at {}:{}:{} - invalid decimals: {}", stringify!($enum_name), loc.file(),loc.line(), loc.column(), decimals)
                        }
                        return Self::new_from_decimals(0u64, decimals);
                    },
                }
            }

            #[cfg_attr(feature = "track_caller", track_caller)]
            pub fn from_udec_and_decimals(value: UD128, decimals: u8) -> Self {
                let value = value.trunc_with_scale(decimals as i16);

                #[cfg(feature = "conv_checks")]
                match value.digits().try_into() {
                    Ok(v) => Self::new_from_decimals::<u64>(v, decimals),
                    Err(e) => {
                        conv_fail!($enum_name, FromUD128, value, e);
                        Self::new_from_decimals(0u64, decimals)
                    },
                }
                #[cfg(not(feature = "conv_checks"))]
                unsafe{Self::new_from_decimals(value.digits().try_into().unwrap_unchecked(), decimals)}
            }


        }

        impl Decisol for $enum_name {
            fn amount(&self) -> u64 {
                match self {
                    $( $enum_name::$variant(v) => v.0, )*
                }
            }
            fn amount_mut(&mut self) -> &mut u64 {
                match self {
                    $( $enum_name::$variant(v) => &mut v.0, )*
                }
            }

            fn decimals(&self) -> u8 {
                match self {
                    $( $enum_name::$variant(_) => $variant::DECIMALS, )*
                }
            }
            fn to_u128(&self) -> u128 {
                match self {
                    $( $enum_name::$variant(v) => v.to_u128(), )*
                }
            }
            fn as_u128(&self) -> u128 {
                match self {
                    $( $enum_name::$variant(v) => v.to_u128(), )*
                }
            }
            fn with_amount<A: ValidAmount>(&self, amount: A) -> Self {
                Self::new(amount, self.kind())
            }
        }
        define_common!($enum_name);
        define_math_common!($enum_name);
        define_math_enum!($enum_name);



    };
}
