
fn main() {
    println!("****************");
    let an_integer = 1u32;
    let a_bool:bool = true;
    // inferred type
    let another_bool = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_bool);
    println!("A boolean: {:?}", another_bool);

    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning

    let an_int = 4u32;
    let another_int = 4_u32;

    let ints_equal = an_int == another_int;
    println!("Are the ints equal: {}", ints_equal);
}