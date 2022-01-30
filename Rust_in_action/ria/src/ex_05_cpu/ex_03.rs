#![allow(dead_code)]
pub fn 起動() {
	use println as 活字;
	let ア:f32 = 42.42;
	let フランケンタイプ:u32 = unsafe { std::mem::transmute(ア) };

	活字!("{}", フランケンタイプ);
	活字!("{:032b}", フランケンタイプ);

	let イ:f32 = unsafe { std::mem::transmute(フランケンタイプ) };
	活字!("{}", イ);
}
