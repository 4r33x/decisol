macro_rules! overflow {
    ($variant:ident, $ops:ident, $left:expr, $right:expr) => {
        #[cfg(any(feature = "overflow_errors", feature = "overflow_panics"))]
        let loc = std::panic::Location::caller();

        #[cfg(feature = "overflow_errors")]
        error!(
            "{} {} failed at {}:{}:{} left: {:?}, right:{:?}",
            stringify!($variant),
            stringify!($ops),
            loc.file(),
            loc.line(),
            loc.column(),
            $left,
            $right
        );

        #[cfg(feature = "overflow_panics")]
        panic!(
            "{} {} failed at {}:{}:{} left: {}, right:{}",
            stringify!($variant),
            stringify!($ops),
            loc.file(),
            loc.line(),
            loc.column(),
            $left,
            $right
        );
    };
}
