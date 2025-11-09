macro_rules! define_math_enum {
    (
        $( $variant:ident ),* $(,)?
    ) => {
        $(
            impl Sub for $variant {
                type Output = Self;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn sub(mut self, rhs: Self) -> Self::Output {
                    check_kind_and_decimals!($variant, SubSelf, self, rhs);
                    #[cfg(feature = "overflow_checks")]
                    if self.amount() < rhs.amount() {
                        overflow!($variant, SubSelf, self, rhs);
                        *self.amount_mut() = 0;
                        return self;
                    }
                    *self.amount_mut() = self.amount() - rhs.amount();
                    self
                }
            }
            impl Sub<u64> for $variant {
                type Output = Self;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn sub(mut self, rhs: u64) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    if self.amount() < rhs {
                        overflow!($variant, SubU64, self, rhs);
                        *self.amount_mut() = 0;
                        return self;
                    }
                    *self.amount_mut() = self.amount() - rhs;
                    self
                }
            }
            impl Sub<$variant> for u64 {
                type Output = $variant;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn sub(self, mut rhs: $variant) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    if self < rhs.amount() {
                        overflow!($variant, U64Sub, self, rhs);
                        *rhs.amount_mut() = 0;
                        return rhs;
                    }
                    *rhs.amount_mut() = self - rhs.amount();
                    rhs
                }
            }
            impl SubAssign for $variant {
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn sub_assign(&mut self, rhs: Self) {
                    check_kind_and_decimals!($variant, SubAssign, self, rhs);
                    #[cfg(feature = "overflow_checks")]
                    {
                        if self.amount() < rhs.amount() {
                            overflow!($variant, SubAssign, self, rhs);
                        }
                        *self.amount_mut() = 0;
                    }
                    #[cfg(not(feature = "overflow_checks"))]
                    {
                        *self = *self - rhs;
                    }
                }
            }
            impl Add for $variant {
                type Output = Self;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn add(mut self, rhs: Self) -> Self::Output {
                    check_kind_and_decimals!($variant, AddSelf, self, rhs);
                    #[cfg(feature = "overflow_checks")]
                    {
                        let (res, over) = self.amount().overflowing_add(rhs.amount());
                        if over {
                            overflow!($variant, AddSelf, self, rhs);
                        }
                        *self.amount_mut() = res;
                        return self;
                    }

                    #[cfg(not(feature = "overflow_checks"))]
                    {
                        *self.amount_mut() = self.amount() + rhs.amount();
                        return self;
                    }

                }
            }
            impl Add<u64> for $variant {
                type Output = Self;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn add(mut self, rhs: u64) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    {
                        let (res, over) = self.amount().overflowing_add(rhs);
                        if over {
                            overflow!($variant, AddU64, self, rhs);
                        }
                        *self.amount_mut() = res;
                        return self;
                    }
                    #[cfg(not(feature = "overflow_checks"))]
                   {
                        *self.amount_mut() = self.amount() + rhs;
                        return self;
                    }
                }
            }

            impl Add<$variant> for u64 {
                type Output = $variant;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn add(self, mut rhs: $variant) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    {
                        let (res, over) = self.overflowing_add(rhs.amount());
                        if over {
                            overflow!($variant, AddU64, self, rhs);
                        }
                        *rhs.amount_mut() = res;
                        return rhs;
                    }
                    #[cfg(not(feature = "overflow_checks"))]
                    {
                        *rhs.amount_mut() = self+ rhs.amount() ;
                        return rhs;
                    }
                }
            }
            impl AddAssign for $variant {
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn add_assign(&mut self, rhs: Self) {
                    check_kind_and_decimals!($variant, AddAssign, self, rhs);
                    #[cfg(feature = "overflow_checks")]
                    {
                        let (res, over) = self.amount().overflowing_add(rhs.amount());
                        if over {
                            overflow!($variant, AddAssign, self, rhs);
                        }
                        *self.amount_mut() = res;
                    }
                    #[cfg(not(feature = "overflow_checks"))]
                    {
                        *self.amount_mut() = self.amount() + rhs.amount();
                    }

                }
            }
        )*
    };
}

macro_rules! check_kind_and_decimals {
    ($variant:ident, $op:ident, $self_:expr, $rhs:expr) => {
        #[cfg(feature = "check_enum_kind")]
        if $self_.kind() != $rhs.kind() {
            overflow!($variant, $op, $self_, $rhs);
        }

        #[cfg(feature = "check_enum_decimals")]
        if $self_.decimals() != $rhs.decimals() {
            overflow!($variant, $op, $self_, $rhs);
        }
    };
}
