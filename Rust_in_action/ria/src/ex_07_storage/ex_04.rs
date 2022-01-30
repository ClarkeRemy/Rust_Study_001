#![allow(dead_code,unused)]
pub fn 起動() {
	use std::{env, fs::File, io::Read};
	const 行当たりのバイト:usize = 16;

	let 引数1 = env::args().nth(1);
	let ファイル名 = 引数1.expect("usage: fview FILENAME");

	let mut f = File::open(&ファイル名).expect("ファイルを開けません");
	let mut 緩衝記憶 = [0; 行当たりのバイト];
	let mut 位置 = 0;

	loop {
		match f.read_exact(&mut 緩衝記憶) {
			Ok(_) => {
				print!("\n[0x{:08x}]", 位置);
				for バイト in &緩衝記憶 {
					match *バイト {
						0x00 => print!(".  "),
						0xff => print!("## "),
						_ => print!(" {:02x}", バイト),
					}
				}
				位置 += 行当たりのバイト
			}
			_ => break,
		}
	}
}
