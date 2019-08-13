// Import (via `use`) the `fmt` module to make it available.
//use std::fmt;
use std::fmt::{self, Formatter, Display};

// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function
fn main() {

    // Each function is a sub-section in rust-by-example

//    hello_world();
//
//    print_formatting();
//
//    debugging();
//
//    displaying();
//
//    displaying_testcase_list();

    formatting()
}

#[allow(dead_code)]
fn hello_world() {
    println!("Hello World!");
    println!("I'm a Rustacean");
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

#[allow(dead_code)]
fn print_formatting() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);
    // Without a suffix, 31 becomes an i32. You can change what type 31 is,
    // with a suffix.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number = 1, width = 6);

    // It will even check to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Create a structure which contains an `i32`. Name it `Structure`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));

    // Formatting pie to 3DP
    let pi = 3.141592;

    println!("{:.3}", pi)
}

#[allow(dead_code)]
fn debugging() {

    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`
    // struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    // #[derive(Debug)]
    // struct DebugPrintable(i32);

    // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
    // is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printable
    // also.
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    // Pretty print
    println!("{:?}", peter);
    println!("{:#?}", peter);

}


// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

#[allow(dead_code)]
fn displaying() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small range is {small}",
        small=small_range,
        big=big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);

    let comp = Complex { real: 3.3, imag: 7.2 };
    println!("Display: {}", comp);
    println!("Debug: {:?}", comp);
}

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing and create
        // a reference to `vec`.
        let vec = &self.0;

        write!(f, "List(Vec[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value
        write!(f, "])")
    }
}

#[allow(dead_code)]
fn displaying_testcase_list() {
    // Try `write!` to see if it errors. If it errors, return
    // the error. Otherwise continue.
    // write!(f, "{}", value)?;

    let v = List(vec![1, 2, 3]);
    println!("{}", v)
}


struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32
}

impl Display for City {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({red}, {green}, {blue}) 0x{red:>02X}{green:>02X}{blue:>02X}",
            red = self.red,
            green = self.green,
            blue = self.blue
        )
    }
}

fn formatting() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city)
    }

    for colour in [
        Colour { red: 128, green: 255, blue: 90 },
        Colour { red: 0, green: 3, blue: 254 },
        Colour { red: 0, green: 0, blue: 0 },
    ].iter() {
        // debug
        println!("{:?}", *colour);
        // display
        println!("{}", *colour);
    }
}