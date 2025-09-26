macro_rules! define_math {
    (
        $( $variant:ident ),* $(,)?
    ) => {
        $(
            impl Sub for $variant {
                type Output = Self;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn sub(self, rhs: Self) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    if self<rhs {
                        overflow!($variant, SubSelf, self.amount(), rhs.amount());
                        return Self::new(0u64);
                    }
                    Self::new(self.amount() - rhs.amount())
                }
            }
            impl Sub<u64> for $variant {
                type Output = Self;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn sub(self, rhs: u64) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    if self.amount() < rhs {
                        overflow!($variant, SubU64, self.amount(), rhs);
                        return Self::new(0u64);
                    }
                    Self::new(self.amount() - rhs)
                }
            }
            impl Sub<$variant> for u64 {
                type Output = $variant;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn sub(self, rhs: $variant) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    if self < rhs.amount() {
                        overflow!($variant, U64Sub, self, rhs.amount());
                        return $variant::new(0u64);
                    }
                    $variant::new(self - rhs.amount())
                }
            }
            impl SubAssign for $variant {
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn sub_assign(&mut self, rhs: Self) {
                    #[cfg(feature = "overflow_checks")]
                    {
                        if self.amount() < rhs.amount() {
                            overflow!($variant, SubAssign, self.amount(), rhs.amount());
                        }
                        *self = Self::new(0u64);
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
                fn add(self, rhs: Self) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    {
                        let (res, over) = self.amount().overflowing_add(rhs.amount());
                        if over {
                            overflow!($variant, AddSelf, self.amount(), rhs.amount());
                        }
                        return Self::new(res);
                    }

                    #[cfg(not(feature = "overflow_checks"))]
                    Self::new(self.amount() + rhs.amount())
                }
            }
            impl Add<u64> for $variant {
                type Output = Self;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn add(self, rhs: u64) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    {
                        let (res, over) = self.amount().overflowing_add(rhs);
                        if over {
                            overflow!($variant, AddU64, self.amount(), rhs);
                        }
                        return Self::new(res);
                    }
                    #[cfg(not(feature = "overflow_checks"))]
                    Self::new(self.amount() + rhs)
                }
            }
            impl Add<$variant> for u64 {
                type Output = $variant;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn add(self, rhs: $variant) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    {
                        let (res, over) = self.overflowing_add(rhs.amount());
                        if over {
                            overflow!($variant, U64Add, self, rhs);
                        }
                        return $variant::new(res);
                    }
                    #[cfg(not(feature = "overflow_checks"))]
                    $variant::new(self + rhs.amount())
                }
            }
            impl AddAssign for $variant {
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn add_assign(&mut self, rhs: Self) {
                    #[cfg(feature = "overflow_checks")]
                    {
                        let (res, over) = self.amount().overflowing_add(rhs.amount());
                        if over {
                            overflow!($variant, AddAssign, self, rhs);
                        }
                        *self = Self::new(res);
                    }
                    #[cfg(not(feature = "overflow_checks"))]
                    {
                        *self = *self + rhs;
                    }

                }
            }
        )*
    };
}
