/*Shape Trait
Create:
trait Shape {
    fn area(&self) -> f64;
}

Implement it for circles, rectangles, and triangles. */
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };
    let triangle = Triangle {
        base: 3.0,
        height: 7.0,
    };

    println!("Circle area: {:.2}", circle.area());
    println!("Rectangle area: {:.2}", rectangle.area());
    println!("Triangle area: {:.2}", triangle.area());
}
