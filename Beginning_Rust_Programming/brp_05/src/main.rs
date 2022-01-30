use std::sync::mpsc;

fn main() {
	let (snd, rcv) = mpsc::channel();
	match snd.send("Wubble wubble foo") {Ok(()) => (),
		                                   Err(e) =>println!("{}",e),};
	println!("Message is: {}", rcv.recv().unwrap());
}
