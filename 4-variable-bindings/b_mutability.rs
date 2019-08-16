
fn main() {
    println!("****************");
    let _immutable_binding = 1;

    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);

    //OK
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Can't assign to immutable binding
    // _immutable_binding += 1;
}