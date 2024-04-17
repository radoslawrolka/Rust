mod spaceship;

use std::fmt::Display;
use std::ops::{Add, Sub};

struct Vec2D {
    x: f64,
    y: f64,
}

impl Vec2D {
    fn new(x: f64, y: f64) -> Self {
        Vec2D { x, y }
    }

    fn equals(&self, other: &Vec2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vec2d=(x: {}, y: {})", self.x, self.y)
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, other: Vec2D) -> Vec2D {
        Vec2D { x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Vec2D) -> Vec2D {
        Vec2D { x: self.x - other.x, y: self.y - other.y}
    }
}

fn main() {
    spaceship::main();
    /*
    let v1 = Vec2D { x: 1.0, y: 2.0 };
    println!("v1 = {}", v1);
    let v2 = Vec2D::new(1.0, 2.0);
    println!("v2 = {}", v2);
    assert!(v1.equals(&v2));
    println!("v1 equals v2: {}", v1.equals(&v2));

    let v3 = v1 + v2;
    println!("v3 = v1 + v2 = {}", v3);
    let v4 = Vec2D::new(2.0, 4.0);
    let v5 = v4 - v3;
    println!("v5 = v4 - v3 = {}", v5);

     */
}