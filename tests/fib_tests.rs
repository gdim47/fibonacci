#[cfg(test)]
mod tests {
    use fib::fib::fibonacci_number;
    #[test]
    fn test_0() {
        assert_eq!(fibonacci_number(0), 1);
    }

    #[test]
    fn test_1() {
        assert_eq!(fibonacci_number(1), 1);
    }

    #[test]
    fn test_12() {
        assert_eq!(fibonacci_number(12), 144);
    }

    #[test]
    fn test_72() {
        assert_eq!(fibonacci_number(72), 498454011879264);
    }
}
