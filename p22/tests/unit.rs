#[test]
fn unit_farenhiet2celsius() {
    let celsuis: i32 = 40;
    assert_eq!(celsuis, p22::calc::farenheit2celsius(104));
}

#[test]
fn unit_celsius2farenhiet() {
    let farenheit: i32 = 104;
    assert_eq!(farenheit, p22::calc::celsius2farenheit(40));
}

#[test]
fn unit_fibonacci_rec(){
    let result: u64 = 102334155;
    assert_eq!(result, p22::calc::fibonacci_rec(40));
}

#[test]
fn unit_fibonacci_loop(){
    let result: u64 = 102334155;
    assert_eq!(result, p22::calc::fibonacci_loop(40));
}
