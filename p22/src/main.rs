fn main() {
    // public items inside module can be accessed outside the parent module
    // call public functions from display module
    let farenheit: i32 = p22::calc::celsius2farenheit(20);
    println!("{farenheit}");
    println!("{}", p22::calc::celsius2farenheit(40));
    println!("{}", p22::calc::farenheit2celsius(104));
    println!("{}", p22::calc::fibonacci_rec(40));
    println!("{}", p22::calc::fibonacci_loop(40));
}
