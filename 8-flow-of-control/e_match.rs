// `allow` required to silence warnings because only
// one variant is used in several exampleas below.
#[allow(dead_code)]

fn main () {
    match_example();

    // destructuring
    destructuring_enums();
    destructuring_pointers();
    destructuring_structures();
    destructuring_tuples();

    // Guards
    match_with_guards();

    // binding
    match_with_binding();
}

fn match_example() -> () {
    let numbers: [i32; 3] = [1, 2, 3];

    for number in numbers.iter() {
        match_number_example(number)
    }
}

fn match_number_example(number: &i32) -> () {
    println!("Tell me abount {}", number);
    match number {
        // Match a single value
        1 => println!("One"),

        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),

        // Match an inclusive range
        13..=19 => println!("A teen"),

        // Handle the rest of cases
        _ => println!("Ain't special"),
    }
}

enum Colour {
    // These 2 are specified solely by their name
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: colour models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn destructuring_enums() {
//    let colour = Colour::RGB(122, 17, 40);
    let colour = Colour::CMYK(1, 2, 3, 4);

    println!("What colour is it");

    // An enum can be destructured using a `match.
    match colour {
        Colour::Red => println!("The colour is Red"),
        Colour::Green => println!("The colour is Green"),
        Colour::Blue => println!("The colour is Blue"),
        Colour::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Colour::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Colour::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Colour::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Colour::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k
            ),
        // Don't need another arm because all variants have been examined.
    }
}

/*
For pointers,
a distinction needs to be made between "destructuring" and "dereferencing"
as they are different concepts which are used differently
from a language like C.
"Dereferencing" uses *
"Destructuring" uses &, ref, and ref mut
*/
fn destructuring_pointers() {
    // Assign a reference of type `i32`. the `&` signifies there is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val=> println!("Got a value via destructing: {:?}", val),
    }

    // to avoid the `&`, you dereference before matching.
    match *reference {
        val=> println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

}


fn destructuring_structures() {
    struct Foo {
        x: (u32, u32),
        y: u32
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 2 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        // Foo { y } => println!("y = {}", y);
    }
}


fn destructuring_tuples() {

    let pair = (0, -2);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);

    // Match can be used to destructure a tuple
    match pair {
        // Destructure the second
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }

}

fn match_with_guards() {
    // A match guard can be added to filter the arm.
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        // the ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

/*
Indirectly accessing a variable makes it impossible to branch and use that variable
without re-binding.
match provides the @ sigil for binding values to names
*/

fn match_with_binding() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I'm not born yet I guess;"),
        // Could `match` 1 ... 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 .. 12. Now the age can be reported.
        n @ 1 ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an adult of age {:?}", n)
    }
}

fn age() -> u32 {
    15
}