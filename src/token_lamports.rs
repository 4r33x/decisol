#[cfg(test)]
mod tests {
    use crate::Decisol;
    use crate::TokenLamports;
    use fastnum::udec128;

    #[test]
    fn token_lamports_mult_test() {
        let _tokens = TokenLamports::new(999999999032878, 6);
        //let res = tokens * price;
        //println!("Res {res}");
        let _tokens = TokenLamports::new(1_000_000_000_000_000, 6);
        let _price_in_sol = udec128!(0.0000416618051001150689138382);
        let _sol_price = udec128!(240.02800589099727313714931554);
        //let mc = tokens * price_in_sol * sol_price;
        //println!("MC {mc:.0}");
    }
    #[test]
    fn token_lamports_conv_test() {
        let price = udec128!(0.00007562495021987651776628);
        let res: TokenLamports = TokenLamports::from_udec(price, 6);
        println!("Res {}", res.amount());
    }
}
