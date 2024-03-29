use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    // Converting to String
    let circle = Circle { radius: 6};
    println!("{}", circle.to_string());

    // Parsing a String
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let x = 120_u8;
    let y = "50".parse::<u8>().unwrap();

    let sum2 = x + y;
    println!("Sum 2: {:?}", sum2)
}