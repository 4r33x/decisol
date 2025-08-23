macro_rules! define_math_enum {
    (
        $( $variant:ident ),* $(,)?
    ) => {
        $(
            impl Sub for $variant {
                type Output = Self;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn sub(self, rhs: Self) -> Self::Output {
                    check_kind_and_decimals!($variant, SubSelf, self, rhs);
                    #[cfg(feature = "overflow_checks")]
                    if self<rhs {
                        overflow!($variant, SubSelf, self, rhs);
                        return Self::new_from_kind(0, self.kind());
                    }
                    Self::new_from_kind(self.amount() - rhs.amount(), self.kind())
                }
            }
            impl Sub<u64> for $variant {
                type Output = Self;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn sub(self, rhs: u64) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    if self.amount() < rhs {
                        overflow!($variant, SubU64, self, rhs);
                        return Self::new_from_kind(0, self.kind());
                    }
                    Self::new_from_kind(self.amount() - rhs, self.kind())
                }
            }
            impl Sub<$variant> for u64 {
                type Output = $variant;
                #[cfg_attr(feature = "track_caller", track_caller)]
                fn sub(self, rhs: $variant) -> Self::Output {
                    #[cfg(feature = "overflow_checks")]
                    if self < rhs.amount() {
                        overflow!($variant, U64Sub, self, rhs);
                        return $variant::new_from_kind(0, rhs.kind());
                    }
                    $variant::new_from_kind(self - rhs.amount(), rhs.kind())
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
                        *self = Self::new_from_kind(0, self.kind());
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
                    check_kind_and_decimals!($variant, AddSelf, self, rhs);
                    #[cfg(feature = "overflow_checks")]
                    {
                        let (res, over) = self.amount().overflowing_add(rhs.amount());
                        if over {
                            overflow!($variant, AddSelf, self, rhs);
                        }
                        return Self::new_from_kind(res, self.kind());
                    }

                    #[cfg(not(feature = "overflow_checks"))]
                    Self::new(self.amount() + rhs.amount(), self.kind())
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
                            overflow!($variant, AddU64, self, rhs);
                        }
                        return Self::new_from_kind(res, self.kind());
                    }
                    #[cfg(not(feature = "overflow_checks"))]
                    Self::new_from_kind(self.amount() + rhs, self.kind())
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
                            overflow!($variant, AddU64, self, rhs);
                        }
                        return $variant::new_from_kind(res, rhs.kind());
                    }
                    #[cfg(not(feature = "overflow_checks"))]
                    $variant::new_from_kind(self + rhs.amount(), rhs.kind())
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
                        *self = Self::new_from_kind(res, self.kind());
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
