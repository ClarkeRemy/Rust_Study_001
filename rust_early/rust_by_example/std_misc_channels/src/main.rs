use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;
use std::thread;

use std::path::Path;

static NTHREADS: i32=3;

fn main() {
let(tx,rx):(Sender<i32>,Receiver<i32>)=mpsc::channel();
let mut children=Vec::new();
for id in 0..NTHREADS{
    let thread_tx=tx.clone();let child=thread::spawn(move||{
        thread_tx.send(id).unwrap();println!("thread {} finished",id);});children.push(child);}
let mut ids=Vec::with_capacity(NTHREADS as usize);
for _ in 0..NTHREADS {ids.push(rx.recv());}
for child in children{child.join().expect("oops! the child thread panicked");}
println!("{:?}", ids);

// Path

let path=Path::new(".");
let _display=path.display();
let new_path=path.join("a").join("b");
match new_path.to_str(){
    None=>panic!("new path is not a valid UTF-8 sequence"),
    Some(s)=>println!("new path is {}",s),}
}
