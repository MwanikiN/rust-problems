/*Rectangle
Create a Rectangle struct.
Implement methods for:
area
perimeter
checking whether it is a square */
struct Rectangle {
    length: f64,
    width: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.width)
    }

    fn square(&self) -> bool {
        self.width == self.length
    }
}

fn main() {
    let object = Rectangle {
        length: 5.0,
        width: 3.0,
    };

    println!("Area: {}", object.area());
    println!("Perimeter: {}", object.perimeter());
    println!("Is square: {}", object.square());
}