use std::fmt::Display;

struct Spaceship {
    x: f64,
    y: f64,
    direction: f64,
}

impl Spaceship {
    fn rotate(&mut self, angle: f64) {
        self.direction = (self.direction + angle) % 360.0;
    }

    fn move_forward(&mut self, distance: f64) {
        self.x += distance * self.direction.to_radians().cos();
        self.y += distance * self.direction.to_radians().sin();
    }

    fn distance1(&self, other: &Spaceship) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn distance2(&self, other: (f64, f64)) -> f64 {
        ((self.x - other.0).powi(2) + (self.y - other.1).powi(2)).sqrt()
    }
}

impl Display for Spaceship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Spaceship=(x: {}, y: {}, direction: {})", self.x, self.y, self.direction)?;
        Ok(())
    }
}

pub(crate) fn main() {
    let mut s1 = Spaceship { x: 0.0, y: 0.0, direction: 0.0 };
    println!("s1 = {}", s1);
    s1.rotate(90.0);
    assert_eq!(s1.direction, 90.0);
    println!("s1 after rotating 90 degrees = {}", s1);
    s1.move_forward(1.0);
    assert_eq!(s1.x, 1.0);
    assert_eq!(s1.y, 0.0);
    println!("s1 after moving forward 1 unit = {}", s1);
    s1.rotate(-45.0);
    assert_eq!(s1.direction, 45.0);
    println!("s1 after rotating -45 degrees = {}", s1);
    s1.move_forward(1.0);
    assert_eq!(s1.x, 1.0 + 1.0 / 2.0_f64.sqrt());
    assert_eq!(s1.y, 1.0 / 2.0_f64.sqrt());
    println!("s1 after moving forward 1 unit = {}", s1);
    assert_eq!(s1.distance1(&Spaceship { x: 0.0, y: 0.0, direction: 0.0 }), 1.0);
    assert_eq!(s1.distance2((0.0, 0.0)), 1.0);
}