use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius as f64).powi(2)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.area());
    println!("{}", circle.to_string());

    let parsed: i8 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i8>().unwrap();
    println!("Parsed: {}, Turbo parsed: {}", parsed, turbo_parsed);

    println!("Sum: {}", parsed + turbo_parsed);
}