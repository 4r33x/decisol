use {rust_decimal::Decimal, rust_decimal_macros::dec};

fn main() {
    let decimal = dec!(0.0000010000);
    let s = "0.0(5)1";
    let s2 = price_formatter(decimal, 3);
    assert_eq!(s, s2);
    println!("{decimal}");
    println!("{s2}");
}
pub fn price_formatter(num: Decimal, scale: u32) -> String {
    let decimal_str = num.to_string();
    let parts: Vec<&str> = decimal_str.split('.').collect();
    if parts.len() != 2 {
        return decimal_str;
    }
    let integer_part: u64 = parts[0].parse().unwrap();
    if integer_part != 0 {
        return num.trunc_with_scale(scale).to_string();
    };
    let decimal_part = parts[1];
    let mut i = 0;
    let mut i_rev = decimal_part.len();
    let mut iter = decimal_part.chars();
    while let Some('0') = iter.next() {
        i += 1;
    }
    let mut iter = decimal_part.chars();
    while let Some('0') = iter.next_back() {
        i_rev -= 1;
    }
    let decimal_part = &decimal_part[i..i_rev];
    if i > 3 {
        format!("{integer_part}.0({i}){decimal_part}")
    } else {
        format!("{integer_part}.{decimal_part}")
    }
}
