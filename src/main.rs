use {fastnum::UD128, rust_decimal::Decimal};

fn main() {
    let fastnum_unsigned_max = UD128::MAX;
    let rust_dec_max = Decimal::MAX;
    let std_u128_max = u128::MAX;
    let fastnum_from_u128_max = UD128::from(std_u128_max);
    let fastnum_div_zero = fastnum_from_u128_max / UD128::ZERO;
    println!("fastnum_unsigned_max: {fastnum_unsigned_max}");
    println!("rust_dec_max: {rust_dec_max}");
    println!("std_u128_max: {std_u128_max}");
    println!("fastnum_from_std_u128_max: {fastnum_from_u128_max}");
    println!(
        "fastnum_diff: {}",
        fastnum_unsigned_max - fastnum_from_u128_max
    );
    println!("fastnum_div_zero: {fastnum_div_zero}")
}
