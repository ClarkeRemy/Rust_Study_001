fn main() {
    use std::{rc::Rc,sync::{Arc, Mutex}};
    let (a,b,c,d)=(
        10,
        Box::new(20),
        Rc::new(Box::new(30)),
        Arc::new(Mutex::new(40)));
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}",a,b,c,d);

}
