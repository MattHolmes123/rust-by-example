#![allow(unreachable_code)]

fn main() {
    intro_to_loop();

    nesting_and_loop_labels();

    returning_from_loops();
}


fn intro_to_loop() -> () {
    let mut count = 0u32;

    println!("Lets count until infinity!");

    let limit = 7;

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("Three");

            // skip the rest of the iteration
            continue;
        }

        println!("{}", count);

        if count == limit {
            println!("Ok, stop at {}", count);

            // Exit the loop
            break;
        }
    }
}


fn nesting_and_loop_labels() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // this would break only the inner loop
            // break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}


fn returning_from_loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}