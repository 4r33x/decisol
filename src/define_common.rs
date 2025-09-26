macro_rules! define_common{
  (
      $($variant:ident),* $(,)?
  ) => {
      $(
        impl From<$variant> for u64 {
            fn from(value: $variant) -> Self {
                value.amount()
            }
        }
        impl PartialEq<u64> for $variant {
            fn eq(&self, other: &u64) -> bool {
                self.amount() == *other
            }
        }
        impl PartialEq<$variant> for u64  {
            fn eq(&self, other: &$variant ) -> bool {
                *self == other.amount()
            }
        }
        impl PartialOrd<u64> for $variant {
             fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
                 Some(self.amount().cmp(other))
             }
        }

        impl PartialOrd<$variant> for u64 {
            fn partial_cmp(&self, other: &$variant) -> Option<Ordering> {
                Some(self.cmp(&other.amount()))
            }
        }


        impl Display for $variant {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let decimals = f.precision().unwrap_or(4);
                let dec: UD128 = self.into();
                let dec = dec.round(decimals as i16);

                // Forward *all* formatting flags (width, align, fill, precision, etc.)
                std::fmt::Display::fmt(&dec, f)
            }
        }
        impl From<$variant> for UD128 {
            fn from(value: $variant) -> Self {
                Self::from_parts(UInt::from(value.amount()), -(value.decimals() as i32), Context::default())
                //Self::from(value.amount()).div(Self::quantum(value.decimals() as i32, Context::default()))
            }
        }
        impl From<&$variant> for UD128 {
            fn from(value: &$variant) -> Self {
                Self::from_parts(UInt::from(value.amount()), -(value.decimals() as i32), Context::default())
                //Self::from(value.amount()).div(Self::quantum(value.decimals() as i32, Context::default()))
            }
        }
        impl From<$variant> for D128 {
            fn from(value: $variant) -> Self {
                Self::from_parts(UInt::from(value.amount()),  -(value.decimals() as i32), fastnum::decimal::Sign::Plus, Context::default())
                //Self::from(value.amount()).div(Self::quantum(value.decimals() as i32, Context::default()))
            }
        }
        impl From<&$variant> for D128 {
            fn from(value: &$variant) -> Self {
                Self::from_parts(UInt::from(value.amount()), -(value.decimals() as i32), fastnum::decimal::Sign::Plus,  Context::default())
                //Self::from(value.amount()).div(Self::quantum(value.decimals() as i32, Context::default()))
            }
        }
      )*
  };
}
