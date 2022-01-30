extern crate lib;

fn main() {
    lib::public_function();

    // Error! `private_function` is private
    // rary::private_function();

    lib::indirect_access();
}
