use std::rc::Rc;
use std::sync::Arc;
use std::thread;

    fn main() {
let rc_examples="Rc examples".to_string();
{
let rc_a: Rc<String>=Rc::new(rc_examples);
println!("Reference Count of rc_a: {}",Rc::strong_count(&rc_a));
{
println!("--- rc_a is cloned to rc_b ---");
let rc_b: Rc<String>=Rc::clone(&rc_a);
println!("Reference Count of rc_b: {}",Rc::strong_count(&rc_b));
println!("Reference Count of rc_a: {}",Rc::strong_count(&rc_a));
println!("rc_a and rc_b are equal: {}",rc_a.eq(&rc_b));
println!("Length of the value inside rc_a: {}\nValue of rc_b: {}",rc_a.len(),rc_b);
println!("--- rc_b is dropped out of scope ---");
}
println!("Reference Count of rc_a: {}",Rc::strong_count(&rc_a));
println!("--- rc_a is dropped out of scope ---");
}
//println!("rc_examples: {}",rc_examples);

// Arc

let apple=Arc::new("the same apple");
for _ in 0..10 {let apple=Arc::clone(&apple);
    thread::spawn(move||{println!("{:?}",apple); }); }
    }
