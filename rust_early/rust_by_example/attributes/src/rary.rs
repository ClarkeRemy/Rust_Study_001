// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "rary"]

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
pub fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux");
}

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_funcion()`");
}

pub fn indirecr_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
