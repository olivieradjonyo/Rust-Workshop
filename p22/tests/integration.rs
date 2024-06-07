use p22::calc::{celsius2farenheit, farenheit2celsius, fibonacci_loop, fibonacci_rec};
use p22::figures::{Circle, Point, Rectangle, Triangle};
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

#[test]
fn test_point_area_perimeter() {
    let point = Point { x: 1.0, y: 1.0 };
    assert_eq!(point.area(), 0.0);
    assert_eq!(point.perimeter(), 0.0);
}

#[test]
fn test_circle_area_perimeter() {
    let circle = Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 1.0,
    };
    assert!((circle.area() - std::f64::consts::PI).abs() < 1e-6);
    assert!((circle.perimeter() - 2.0 * std::f64::consts::PI).abs() < 1e-6);
}

#[test]
fn test_triangle_area_perimeter() {
    let triangle = Triangle {
        point1: Point { x: 0.0, y: 0.0 },
        point2: Point { x: 1.0, y: 0.0 },
        point3: Point { x: 0.0, y: 1.0 },
    };
    assert!((triangle.area() - 0.5).abs() < 1e-6);
    let perimeter = 1.0 + 1.0 + (2.0_f64).sqrt();
    assert!((triangle.perimeter() - perimeter).abs() < 1e-6);
}

#[test]
fn test_rectangle_area_perimeter() {
    let rectangle = Rectangle {
        top_left: Point { x: 0.0, y: 1.0 },
        bottom_right: Point { x: 1.0, y: 0.0 },
    };
    assert!((rectangle.area() - 1.0).abs() < 1e-6);
    assert!((rectangle.perimeter() - 4.0).abs() < 1e-6);
}
