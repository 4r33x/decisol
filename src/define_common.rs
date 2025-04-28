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
          impl Display for $variant {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let dec: UD128 = self.into();
                let decimals = f.precision().unwrap_or(4);
                write!(f, "{dec:.decimals$}")
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
