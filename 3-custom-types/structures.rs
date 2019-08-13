//# https://stackoverflow.com/questions/47640550/what-is-a-in-rust-language
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(r: &Rectangle) -> f32 {
    let Rectangle {
        p1: Point { x: ref x1, y: ref y1 },
        p2: Point { x: ref x2, y: ref y2 }
    } = *r;

    ((x2 - x1) * (y2 - y1))
}

fn square(point: Point, i: f32) -> Rectangle {

    let Point {x,y} = point;

    Rectangle {
        p1: Point{ x, y },
        p2: Point { x: (x + i), y: (y + i) }
    }
}

fn main() {
    // create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.1, ..point };

    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: Point { x: 3.5, y : 4.1 },
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rect_area: {}", rect_area(&_rectangle));

    let point = Point { x: 1.0, y: 2.0 };
    println!("Point: {:?}", point);
    println!("Square: {:?}", square(point, 4.0))
}