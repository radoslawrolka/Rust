use std::fmt::Display;

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;

    fn print(&self) {
        println!("Area: {}, Perimeter: {}", self.area(), self.perimeter());
    }
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height + 1.0
    }
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Rectangle: width: {}, height: {}", self.width, self.height)?;
        Ok(())
    }
}

fn main() {
    let r1 = Rectangle { width: 10.0, height: 20.0 };
    println!("Area: {}", r1.area());
    r1.print();
    println!("{}", r1);

}