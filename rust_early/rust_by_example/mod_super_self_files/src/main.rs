mod cool;
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::indirect_call();

    // split

    my::function();
    function();
    my::indirect_access();
    my::nested::function();
}
