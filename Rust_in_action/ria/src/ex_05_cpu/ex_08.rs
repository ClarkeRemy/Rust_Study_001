#![allow(dead_code)]
const BIAS:i32 = 127;
const RADIX:f32 = 2.0;

pub fn 起動() {
	let 数:f32 = 42.42;

	let (符号ビット, 指数, 分数) = 脱構成_f32(数);
	let (符号, 指数, 仮数) = 逆符号化_f32_部分(符号ビット, 指数, 分数);
	let 再構成の数 = f32_から_部分(符号, 指数, 仮数);

	println!(
	         "{} -> [sign:{}, exponent:{}, mantissa:{:?}] ->  {}",
	         数, 符号ビット, 指数, 仮数, 再構成の数
	);
}

fn 脱構成_f32(数:f32) -> (u32, u32, u32) {
	let 数_:u32 = unsafe { std::mem::transmute(数) };

	let 符号 = (数_ >> 31) & 1;
	let 指数 = (数_ >> 23) & 0xff;
	let 分数 = 0b00000000_01111111_11111111_11111111 & 数_;

	(符号, 指数, 分数)
}

fn 逆符号化_f32_部分(符号:u32, 指数:u32, 分数:u32) -> (f32, f32, f32) {
	let 符号付_1 = (-1.0_f32).powf(符号 as f32);

	let 指数 = RADIX.powf(((指数 as i32) - BIAS) as f32);

	let mut 仮数:f32 = 1.0;
	for i in 0..23_u32 {
		let one_at_bit_i = 1 << i;
		match one_at_bit_i & 分数 {
			0 => (),
			_ => 仮数 += 2_f32.powf((i as f32) - 23.0),
		}
	}
	(符号付_1, 指数, 仮数)
}

#[inline]
fn f32_から_部分(符号:f32, 指数:f32, 仮数:f32) -> f32 { 符号 * 指数 * 仮数 }
