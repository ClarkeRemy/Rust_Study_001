#![allow(dead_code)]
use std::mem;

pub fn 起動() {
	let ビッグエンディアン:[u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
	let リトルエンディアン:[u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

	let ア:i32 = unsafe { mem::transmute(ビッグエンディアン) };
	let イ:i32 = unsafe { mem::transmute(リトルエンディアン) };

	println!("{} vs {}", ア, イ);
}
