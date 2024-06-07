/// Creates a function that converts celsuis to farenheit.
///
/// # Example
///
/// ```
/// assert_eq!(68, p22::calc::celsius2farenheit(20));
/// ```
pub fn celsius2farenheit(celsius: i32) -> i32 {
    let farenheit: i32 = (celsius * 9 / 5) + 32;
    farenheit
}

/// Creates a function that converts farenheit to celsuis.
///
/// # Example
///
/// ```
/// assert_eq!(40, p22::calc::farenheit2celsius(104));
/// ```
pub fn farenheit2celsius(farenheit: i32) -> i32 {
    let celsius: i32 = (farenheit - 32) * 5 / 9;
    celsius
}

/// Fibonacci loop function.
///
/// # Example
///
/// ```
/// assert_eq!(102334155, p22::calc::fibonacci_rec(40));
/// ```
pub fn fibonacci_loop(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut fibonacci_number: u64 = 1;
    let mut f0: u64 = 0;
    let mut f1: u64 = 1;
    let mut mycounter: u32 = 1;
    while mycounter < n {
        fibonacci_number = f0 + f1;
        f0 = f1;
        f1 = fibonacci_number;
        mycounter += 1;
    }
    fibonacci_number
}

/// Recorsive Fibonacci function.
///
/// # Example
///
/// ```
/// assert_eq!(102334155, p22::calc::fibonacci_rec(40));
/// ```
pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_rec(n - 2) + fibonacci_rec(n - 1),
    }
}

#[cfg(test)]
mod tests {
    use crate::calc::{celsius2farenheit, farenheit2celsius, fibonacci_loop, fibonacci_rec};
    #[test]
    fn unit_farenhiet2celsius() {
        let celsuis: i32 = 40;
        assert_eq!(celsuis, farenheit2celsius(104));
    }

    #[test]
    fn unit_celsius2farenhiet() {
        let farenheit: i32 = 104;
        assert_eq!(farenheit, celsius2farenheit(40));
    }

    #[test]
    fn unit_fibonacci_rec() {
        let result: u64 = 102334155;
        assert_eq!(result, fibonacci_rec(40));
    }

    #[test]
    fn unit_fibonacci_loop() {
        let result: u64 = 102334155;
        assert_eq!(result, fibonacci_loop(40));
    }
}
