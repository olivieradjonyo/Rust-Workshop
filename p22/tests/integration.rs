#[test]
fn test_celsius2farenheit2celsius() {
    let celsius: i32 = 40;
    let farenheit: i32 = 104;
    assert_eq!(
        celsius,
        p22::calc::farenheit2celsius(p22::calc::celsius2farenheit(celsius))
    );
    assert_eq!(
        farenheit,
        p22::calc::celsius2farenheit(p22::calc::farenheit2celsius(farenheit))
    );
}

#[test]
fn test_compare_fibo_loop_and_fibo_rec() {
    let n: u32 = 40;
    assert_eq!(p22::calc::fibonacci_loop(n), p22::calc::fibonacci_rec(n));
}
