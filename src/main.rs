use std::io;

fn fibonacci_number(n: u32) -> u64 {
    let GoldenRatio : f64 = (1. + 5.0_f64.sqrt()) / 2.;
    return match n {
        0 => 1,
        _ => ((GoldenRatio.powi(n as i32) - (1. - GoldenRatio).powi(n as i32))
                / 5.0_f64.sqrt()) as u64
    }
}

fn main() {
    println!("Enter N for calculating N-th fibonacci number: ");
    let mut in_str = String::new();
    io::stdin().read_line(&mut in_str).expect("Failed to get input string");
    let n = in_str.trim().parse::<u32>();
    match n {
        Ok(num) => println!("Result: {}", fibonacci_number(num)),
        Err(_) => eprintln!("Input is not valid integer number")
    }
}
