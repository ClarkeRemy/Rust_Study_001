#![allow(dead_code,unused)]
use std::thread;

pub fn run() {
	use std::{thread, time};
	let start = time::Instant::now();
	let handler = thread::spawn(move || {
		let pause = time::Duration::from_millis(300);
		thread::sleep(pause.clone());
	});

	handler.join().unwrap();
	let finish = time::Instant::now();
	println!("{:02?}", &finish.duration_since(start));
}
