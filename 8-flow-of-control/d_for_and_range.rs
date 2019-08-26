
fn main() {
    fizz_buzz_for_in_example();

    for_in_iter_example();
    for_in_into_iter_example();
    for_in_iter_mut_example();
}

fn fizz_buzz_for_in_example() {
    // `n` will take the values, 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Alternatively, a..=b can be used for a range that is inclusive on both ends.
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn for_in_iter_example() {
    /* iter:
    This borrows each element of the collection through each iteration.
    Thus leaving the collection untouched and available for reuse after the loop.
    */
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a Rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("{:?}", names)
}


fn for_in_into_iter_example() {
    /* into_iter:
    This consumes the collection so that on each iteration the exact data is provided.
    Once the collection has been consumed it is no longer available for reuse as it has
    been 'moved' within the loop.
    */

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // This would throw an error
    // println!("{:?}", names)
}


fn for_in_iter_mut_example() {
    /* iter_mut:
    This mutably borrows each element of the collection,
    allowing for the collection to be modified in place.
    */
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

