
fn main() {
    // awkward use of matching enums.
    awkward_match_enum_example();

    // same as above using `if let`
    // if let is cleaner for this use case and in addition
    // allows various failure options to be specified
    if_let_example();

    if_let_match_enum();

    if_let_enum_challenge();
}


fn awkward_match_enum_example() {
    // Make `optional` of type `Option<i32>`
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            // ^ Needed 2 indentations just so we could destructure
            // `i` from the option.
        },
        _ => {},
        // ^ Required because `match` is exhaustive. Doesn't it seem
        // like wasted space?
    };
}


fn if_let_example() {
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i)
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    };

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    };
}

// Our example enum
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn if_let_match_enum() {
    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}

// This enum purposely doesn't #[derive(PartialEq)],
// neither we implement PartialEq for it. That's why comparing Foo::Bar==a fails below.
enum Fooo {Bar}

fn if_let_enum_challenge() {
    let a = Fooo::Bar;

    // Variable a matches Foo::Bar
//    if Fooo::Bar == a {
//    // ^-- this causes a compile-time error. Use `if let` instead.
//        println!("a is foobar");
//    }

    if let Fooo::Bar = a {
        println!("a is fooobar");
    }
}