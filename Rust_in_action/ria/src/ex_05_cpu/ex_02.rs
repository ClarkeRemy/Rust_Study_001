#![allow(dead_code)]
pub type 整数 = i32;
pub fn 猫(数量:整数) -> 整数 { 数量 }

pub fn 起動() {
	use println as 活字;
	let ア:u16 = 50115;
	let イ:i16 = -15421;

	活字!("a: {:016b}  {}", ア, ア);
	活字!("b: {:016b}  {}", イ, イ);
}
