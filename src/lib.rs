pub mod fib {
    pub fn fibonacci_number(n: u32) -> u64 {
        let GoldenRatio : f64 = (1. + 5.0_f64.sqrt()) / 2.;
        return match n {
            0 => 1,
            _ => ((GoldenRatio.powi(n as i32) - (1. - GoldenRatio).powi(n as i32))
                    / 5.0_f64.sqrt()).round() as u64
        }
    }
}
