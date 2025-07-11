// use fastnum::D128;
// use fastnum::UD128;
// use fastnum::dec128;
fn main() {}
// fn main() {
//     let max = D128::MAX;
//     let min = D128::MIN;
//     let zero0 = D128::ZERO;
//     let zero1 = D128::ZERO;
//     let perc0 = dec128!(0.12214512651236125);
//     let perc1 = dec128!(-0.12214512651236125);
//     let perc2 = perc0 * perc1;
//     let perc3 = dec128!(500.1251235125) / dec128!(100);
//     let perc4 = dec128!(-500.1251235125) / dec128!(100);
//     let perc5 = perc3 * perc4;
//     let nan = D128::NAN;
//     let div_zero = perc5 / zero0;
//     let inf = D128::INFINITY;
//     let neg_inf = -D128::INFINITY;
//     let neg_nan = -nan;
//     let neg_div_zer = -div_zero;
//     let mut v = vec![
//         max, min, inf,
//         neg_inf,
//         //nan, //zero0,
//         // zero1, perc0, perc1, perc2, perc3, perc4,
//         // perc5,
//         //nan,
//         // div_zero, inf,
//         // neg_inf,
//         //neg_nan,
//         // neg_div_zer,
//         // max,
//         // min,
//         // zero0,
//         // zero1,
//         // perc0,
//         // perc1,
//         // perc2,
//         // perc3,
//         // perc4,
//         // perc5,
//         // nan,
//         // div_zero,
//         // inf,
//         // neg_inf,
//         //neg_nan,
//         //neg_div_zer,
//     ]
//     .into_iter()
//     .map(create_enum)
//     .collect::<Vec<_>>();
//     println!("Len is {}", v.len());
//     v.sort_by(|a, b| a.0.cmp(&b.0));

//     println!("Sorted: {v:?}")
// }

// #[derive(Debug)]
// enum StringEnum {
//     Some,
//     SomeString { value: String },
//     SomeOtherString(String),
//     AnotherOhterString(String),
//     None,
// }

// fn create_enum(dec: D128) -> (D128, StringEnum) {
//     if dec.is_zero() {
//         (
//             dec,
//             StringEnum::SomeString {
//                 value: "Zero".to_string(),
//             },
//         )
//     } else if dec.is_infinite() {
//         return (dec, StringEnum::SomeOtherString("Infinity".to_string()));
//     } else if dec.is_negative() {
//         return (dec, StringEnum::AnotherOhterString("Negative".to_string()));
//     } else {
//         return (
//             dec,
//             StringEnum::SomeString {
//                 value: "Finit_Positive_Decimal".to_string(),
//             },
//         );
//     }
// }
