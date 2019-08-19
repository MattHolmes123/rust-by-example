use std::convert::From;


#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {value: item}
    }
}

fn main() {
    let my_str = "Hello";
    println!("my_str {:?}", my_str);

    let my_string = String::from(my_str);
    println!("my_string {:?}", my_string);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    // This doesn't work as it hasn't been implemented for Number.
    // let unsigned_num:u32 = 30;
    // let unum = Number::from(unsigned_num);
    // println!("My number is {:?}", unum);

    // Example of into

    let int = 5;

    // Type declaration needed (it cannot infer the type)
    let num:Number = int.into();

    println!("My number is {:?}", num);
}