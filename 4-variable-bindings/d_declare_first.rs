
fn main() {
    println!("****************");

    // Declare a variable binding
    let a_binding;

    {
        let x = 2;
        // initialise the binding
        a_binding = x * x;
    }

    println!("a_binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
