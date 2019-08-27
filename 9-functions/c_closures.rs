
fn main() {
    closure_intro();

    closure_capturing();

    closure_as_input_parameters();
}

/*
The syntax and capabilities of closures make them very convenient for on the fly usage.
Calling a closure is exactly like calling a function.
However, both input and return types can be inferred and input variable names must be specified.

Other characteristics of closures include:
* using || instead of () around input variables.
* optional body delimination ({}) for a single expression (mandatory otherwise).
* the ability to capture the outer environment variables.
*/
fn closure_intro() {

    // Increment via closures and functions.
    fn function (i: i32) -> i32 { i + 1}

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    // call the finction and closures.
    println!("function:              {}", function(i));
    println!("closure_annotated:     {}", closure_annotated(i));
    println!("closure_inferred:      {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}

/*
Closures are inherently flexible and will do what the functionality requires to make
the closure work without annotation.
This allows capturing to flexibly adapt to the use case,
sometimes moving and sometimes borrowing.
Closures can capture variables:
    * by reference: &T
    * by mutable reference: &mut T
    * by value: T
They preferentially capture variables by reference and only go lower when required.
*/
fn closure_capturing() {
    use std::mem;

    let colour = "green";

    // A closure to print `colour` which immediately borrows (`&`)
    // `colour` and stores the borrow and closure in the `print`
    // variable. It will remain borrowed until `print` goes out of
    // scope. `println!` only requires `by reference` so it doesn't
    // impose anything more restrictive.
    let print = || println!("`colour: {}", colour);

    print();
    print();

    let mut count = 0;

    // A closure to increment `count` could take either `&mut count`
    // or `count` but `&mut count` is less restrictive so it takes
    // that. Immediately borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside.
    // Thus, calling the closure mutates the closure which requires
    // a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure.
    inc();
    inc();

    // let _reborrow = &mut count;
    // ^ TODO: try uncommenting this line.

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    //consume();
    // ^ TODO: Try uncommenting this line.

    // Using move before vertical pipes forces closure to take ownership of captured variables
    // `Vec` has a non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.

    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}


/*
While Rust chooses how to capture variables on the fly mostly without type annotation,
this ambiguity is not allowed when writing functions.
When taking a closure as an input parameter, the closure's complete type
must be annotated using one of a few traits.
In order of decreasing restriction, they are:

    * Fn: the closure captures by reference (&T)
    * FnMut: the closure captures by mutable reference (&mut T)
    * FnOnce: the closure captures by value (T)
    * On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.
*/
fn closure_as_input_parameters() {
    use std::mem;

    let greeting = "hello";

    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference requiring `Fn`.
        println!("I said {}", greeting);

        // Mutation forces `farewell` to be captured by mutable reference
        // Now requires `FnMut`
        farewell.push_str("!!!");

        // Manually calling drop forces `farewell` to be captured
        // by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

// A function which takes a closure as an argument and calls it.
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}