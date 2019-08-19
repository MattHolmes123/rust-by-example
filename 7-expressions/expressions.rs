
fn main() {

    // expression assignment example
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;

        // this expression will be assigned to `y`
        x_cubed + x_squared + x
    };

    let z = get_z(x);

    let no_int = {
        // The semicolon suppresses this expression and `()` is assigned to `no_int`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("no_int is {:?}", no_int);
    println!("z is {:?}", z);
}

fn get_z(x: u32) -> u32 {
    let x_squared = x * x;
    let x_cubed = x_squared * x;

    // this expression will be returned to `z`
    // the last line doesn't have a `;`
    x_cubed + x_squared + x
}