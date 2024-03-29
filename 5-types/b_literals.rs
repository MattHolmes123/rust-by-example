
fn main() {
    println!("****************");

    // Suffixed literals, their types are known at initialisation
    let x = 1;
    let y = 2u64;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used.
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));


    let a = 1;
    let b = 1;

    let c = a == b;

    println!("is a = b: {}", c)
}