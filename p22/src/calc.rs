pub fn print() {
    println!("config!");
}

pub fn celsius2farenheit(celsius: i32) -> i32 {
    let farenheit: i32 = (celsius * 9 / 5) + 32;
    return farenheit;
}

pub fn farenheit2celsius(farenheit: i32) -> i32 {
    let celsius: i32 = (farenheit - 32) * 5 / 9;
    return celsius;
}

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
        mycounter = mycounter + 1;
    }
    return fibonacci_number;
}

pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => return 0,
        1 => return 1,
        _ => return fibonacci_rec(n - 2) + fibonacci_rec(n - 1),
    }
}
