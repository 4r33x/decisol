macro_rules! conv_fail {
    ($variant:ident, $ops:ident, $value:expr, $error:expr) => {
        #[cfg(any(feature = "conv_errors", feature = "conv_panics"))]
        let loc = std::panic::Location::caller();

        #[cfg(feature = "conv_errors")]
        error!(
            "{} {} failed at {}:{}:{} value: {} error: {}",
            stringify!($variant),
            stringify!($ops),
            loc.file(),
            loc.line(),
            loc.column(),
            $value,
            $error,
        );

        #[cfg(feature = "conv_panics")]
        panic!(
            "{} {} failed at {}:{}:{} value: {} error: {}",
            stringify!($variant),
            stringify!($ops),
            loc.file(),
            loc.line(),
            loc.column(),
            $value,
            $error,
        );
    };
}
