use std::io;

extern crate fib;
use fib::fib::fibonacci_number;

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
