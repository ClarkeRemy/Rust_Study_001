use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
    println!("'static value passed in is {:?}", input);
}

fn use_it() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // use_it(), so it's not 'static:
    //print_it(&i);
}

// Make a constant with `'static lifetime.`
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

// `elided_input` and `annotated_input` essentially have identical signatures
// because the lifetime of `elided_input` is inferred by the compiler:
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}
fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

// Similarly, `elided_pass` and `annotated_pass` have identical signatures
// because the lifetime is added implicitly to `elided_pass`:
fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn main() {
    // A reference with 'static lifetime:
    let s: &'static str = "hello world";

    // 'static as part of a trait bound:
    fn generic<T>(x: T)
    where
        T: 'static,
    {
    }

    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }
    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {} stays accessible!", NUM);

    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", annotated_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}
