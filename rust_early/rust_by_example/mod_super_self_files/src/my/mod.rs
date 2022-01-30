// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them under their respective
// modules
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access(), thats\n> `");

    private_function();
}

mod cool {
    pub fn function() {
        println!("called `my::cool::function()`");
    }
}

pub fn indirect_call() {
    // Let's access all the functions named `function` from this scope!
    print!("called `my::indirect_call()`, that\n> ");

    // The `self` keyword refers to the current module scope - in this case `my`.
    // Calling `self::function()` and calling `function()` directly both give
    // the same result, because they refer to the same function.
    self::function();
    function();

    // We can also use `self` to access another module inside `my`:
    self::cool::function();

    // The `super` keyword refers to the parent scope (outside the `my` module).
    super::function();

    // The `super` keyword refers to the parent scope (outside the `my` module).
    super::function();

    // This will bind to the `cool::function` in the *crate* scope.
    // In this case the crate scope is the outermost scope.
    {
        use crate::cool::function as root_function;
        root_function();
    }
}
