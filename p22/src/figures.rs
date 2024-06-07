/// Areas and perimeters of point, circle, triangle and rectangle.
///
/// # Example
///
/// ```
/// use p22::figures::{Circle, Point, Rectangle, Shape, Triangle};
/// fn example(){
/// let p1 = Point { x: 0.0, y: 0.0 };
/// let p2 = Point { x: 1.0, y: 1.0 };
/// let p3 = Point { x: 2.0, y: 2.0 };
/// let p4 = Point { x: 3.0, y: 3.0 };
/// let circle = Circle {
///     center: p1.clone(),
///     radius: 5.0,
/// };
/// let triangle = Triangle {
///     point1: p1.clone(),
///     point2: p2.clone(),
///     point3: p3.clone(),
/// };
/// let rectangle = Rectangle {
///     top_left: p1.clone(),
///     bottom_right: p4.clone(),
/// };
/// let point = p1.clone();
/// let shapes: Vec<Shape> = vec![
///     Shape::Point(point),
///     Shape::Circle(circle),
///     Shape::Triangle(triangle),
///     Shape::Rectangle(rectangle),
/// ];
/// for shape in shapes {
///     match shape {
///         Shape::Point(p) => {
///             println!(
///                 "Point at ({}, {}), Area: {}, Perimeter: {}",
///                 p.x,
///                 p.y,
///                 p.area(),
///                 p.perimeter()
///             );
///         }
///         Shape::Circle(c) => {
///             println!(
///                 "Circle with center at ({}, {}) and radius {}, Area: {}, Perimeter: {}",
///                 c.center.x,
///                 c.center.y,
///                 c.radius,
///                 c.area(),
///                 c.perimeter()
///             );
///         }
///         Shape::Triangle(t) => {
///             println!(
///                 "Triangle with points ({}, {}), ({}, {}), ({} {}), Area: {}, Perimeter: {}",
///                 t.point1.x,
///                 t.point1.y,
///                 t.point2.x,
///                 t.point2.y,
///                 t.point3.x,
///                 t.point3.y,
///                 t.area(),
///                 t.perimeter()
///             );
///         }
///         Shape::Rectangle(r) => {
///             println!("Rectangle with top-left at ({}, {}) and bottom-right at ({}, {}), Area: {}, Perimeter: {}",
///                      r.top_left.x, r.top_left.y, r.bottom_right.x, r.bottom_right.y,
///                      r.area(), r.perimeter());
///         }
///     }
/// }
/// }
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Triangle {
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Triangle {
    fn distance(p1: &Point, p2: &Point) -> f64 {
        ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
    }

    pub fn area(&self) -> f64 {
        let a = Self::distance(&self.point1, &self.point2);
        let b = Self::distance(&self.point2, &self.point3);
        let c = Self::distance(&self.point3, &self.point1);
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }

    pub fn perimeter(&self) -> f64 {
        let a = Self::distance(&self.point1, &self.point2);
        let b = Self::distance(&self.point2, &self.point3);
        let c = Self::distance(&self.point3, &self.point1);
        a + b + c
    }
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        let width = (self.bottom_right.x - self.top_left.x).abs();
        let height = (self.bottom_right.y - self.top_left.y).abs();
        width * height
    }

    pub fn perimeter(&self) -> f64 {
        let width = (self.bottom_right.x - self.top_left.x).abs();
        let height = (self.bottom_right.y - self.top_left.y).abs();
        2.0 * (width + height)
    }
}

impl Point {
    pub fn area(&self) -> f64 {
        0.0
    }

    pub fn perimeter(&self) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
