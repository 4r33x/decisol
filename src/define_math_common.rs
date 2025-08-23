macro_rules! define_math_common {
    (
        $( $variant:ident ),* $(,)?
    ) => {
        $(
            impl<T: Decisol> Mul<T> for $variant {
                type Output = u128;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn mul(self, rhs: T) -> Self::Output {
                    self.amount() as u128 * rhs.amount() as u128
                }
            }
            impl Mul<u64> for $variant {
                type Output = u128;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn mul(self, rhs: u64) -> Self::Output {
                    self.amount() as u128 * rhs as u128
                }
            }
            impl Mul<$variant> for u64 {
                type Output = u128;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn mul(self, rhs: $variant) -> Self::Output {
                    self as u128 * rhs.amount() as u128
                }
            }
            impl Div<$variant> for u128 {
                type Output = u128;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn div(self, rhs: $variant) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    if rhs.amount() == 0 {
                        overflow!($variant, Divu128, self, rhs);
                        return 0;
                    }
                    self / rhs.amount() as u128
                }
            }
            impl Div<UD128> for $variant {
                type Output = UD128;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn div(self, rhs: UD128) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    if rhs == UD128::ZERO {
                        overflow!($variant, DivUD128, self, rhs);
                        return UD128::NAN;
                    }
                    let lhs: UD128 = self.into();
                    lhs / rhs
                }
            }
            impl Mul<UD128> for $variant {
                type Output = $variant;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn mul(mut self, r: UD128) -> Self::Output {
                    let r = r.trunc_with_scale(9);
                    let f = r.fractional_digits_count();
                    let raw = r.digits();
                    let (raw, scale) = if f < 0 {  (raw * UInt::TEN.pow( f as u32), UInt::ONE) } else { (raw, UInt::TEN.pow(f as u32)) };
                    let lhs = UInt::from_u64(self.amount());
                    let res = (lhs * raw) / scale;

                    #[cfg(feature = "overflow_checks")]
                    let prod: u64 = match res.try_into() {
                        Ok(v) => v,
                        Err(_) => {
                            overflow!($variant, MultConvFromUint, self, r);
                            0
                        }
                    };

                    #[cfg(not(feature = "overflow_checks"))]
                    let prod = res.try_into().unwrap_or_default();
                    *self.amount_mut() = prod;
                    self
                }
            }
            impl $variant {
                #[cfg_attr(feature = "track_caller", track_caller)]
                pub fn div_ceil(mut self, r: UD128) -> Self {
                    let r = r.trunc_with_scale(9);
                    let f = r.fractional_digits_count();
                    let raw = r.digits();
                    let (raw, scale) = if f < 0 { (raw * UInt::TEN.pow((-f) as u32), UInt::ONE) } else { (raw, UInt::TEN.pow(f as u32)) };
                    let lhs = UInt::from_u64(self.amount());
                    let res = (lhs * raw).div_ceil(scale);

                    #[cfg(feature = "overflow_checks")]
                    let prod: u64 = match res.try_into() {
                        Ok(v) => v,
                        Err(_) => {
                            overflow!($variant, DivCeilConvFromUint, self, r);
                            0
                        }
                    };

                    #[cfg(not(feature = "overflow_checks"))]
                    let prod = res.try_into().unwrap_or_default();
                    *self.amount_mut() = prod;
                    self
                }
            }
        )*


    };
}
