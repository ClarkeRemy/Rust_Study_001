macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
    // Decompose multiple `eval`s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! {eval $e}
        calculate! { $(eval $es),+}
    }};
}
fn main() {
    calculate! {
        eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
    }
    calculate! {
        eval (1 + 2) * (3 / 4)
    }

    // 2

    calculate! { // look ma! Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 + 3) + 1
    }
}
