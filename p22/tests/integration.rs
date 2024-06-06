use p22::calc::{celsius2farenheit, farenheit2celsius, fibonacci_loop, fibonacci_rec};
use p22::song::print_twelve_days_of_christmas;

#[test]

fn test_celsius2farenheit2celsius() {
    let celsius: i32 = 40;
    let farenheit: i32 = 104;
    assert_eq!(celsius, farenheit2celsius(celsius2farenheit(celsius)));
    assert_eq!(farenheit, celsius2farenheit(farenheit2celsius(farenheit)));
}

#[test]
fn test_compare_fibo_loop_and_fibo_rec() {
    let n: u32 = 40;
    assert_eq!(fibonacci_loop(n), fibonacci_rec(n));
}

#[test]
fn test_print_twelve_days_of_christmas() {
    print_twelve_days_of_christmas();
}
