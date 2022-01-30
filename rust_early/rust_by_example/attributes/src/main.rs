// rustc --cfg some_condition main.rs --extern rary=library.rlib  && ./main
extern crate rary;
fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}
// FIXME ^ Add an attribute to supress the warning

use ::rary::are_you_on_linux;

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    println!("Hello, world!");
    used_function();

    are_you_on_linux();

    println!("are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. definately linux!");
    } else {
        println!("Yes. It's definately *not* linux!");
    }

    conditional_function();
}
