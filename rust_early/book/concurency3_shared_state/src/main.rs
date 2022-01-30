use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handeles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handeles.push(handle);
    }

    for handle in handeles {
        handle.join().unwrap()
    }

    println!("Result: {}", *counter.lock().unwrap());
}
