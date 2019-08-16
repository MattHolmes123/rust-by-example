
fn main() {
    println!("****************");
    // this binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        #[allow(unused_variables)]
        // This binding only exists in this block
        let short_lived_binding = 2;

        #[allow(unused_variables)]
        // This binding *shadows* the outer one
        let long_lived_binding = 5_f32;

    }
    //end of block

    // error! `short_lived_binding` doesn't exist in this scope
    // println!("Outer short: {}", short_lived_scope);

    println!("Outer long: {}", long_lived_binding);

    // this binding also *shadows* the previous binding
    let long_lived_binding = 'a';

    println!("Outer long: {}", long_lived_binding);

    // Test if statements

    if true {
        let a_char = 'a';

        println!("`a_char` in the if: {a}", a = a_char)
    }

    // a_char is not in this scope.
    // println!("`a_char` out the if: {a}", a = a_char)



}