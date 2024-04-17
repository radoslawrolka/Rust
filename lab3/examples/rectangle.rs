#[derive(Debug)] // allows to print the structure in debug mode (ie. to use {:?})
struct Rectangle {
    x : f32,
    y : f32
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.x * self.y
    }

    fn scale(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }

    fn new_square(side: f32) -> Rectangle {
        Rectangle{x, y: side}
    }
}

fn main() {
    let r1 = Rectangle{x: 1.0, y: 2.0}; // create a new instance using constructor
    println!{"{:?}", r1};
    println!("x: {}, y: {}", r1.x, r1.y); // access particular fields using . operator

    let r2 = Rectangle{y : 5f32, ..r1}; // the rest of r2 parameters are copied from r1
    println!{"{:?}", r2};
    println!("x: {}, y: {}", r2.x, r2.y);

    println!("Area of r1: {}", r1.area());

    let mut r3 = Rectangle{x: 3.0, y: 4.0};
    r3.scale(2.0);
    println!("Area of r3: {}", r3.area());

    let s1 = Rectangle::new_square(5.0);
    println!{"{:?}", s1};
}