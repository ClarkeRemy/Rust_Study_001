#![allow(dead_code)]

macro_rules! println_all {
	($($arg:tt)*) => {
		$(println!$arg ;)*
	};
}

static 大域的:i32 = 1000;

fn 無演算命令() -> *const i32 {
	let ローカルの無演算命令 = 12345;
	&ローカルの無演算命令 as *const i32
}

pub fn 起動() {
	let (ローカル_文字列, ローカル_整数) = ("a", 123);
	let (ボックス_文字列, ボックス_整数) = (Box::new('b'), Box::new(789));
	let 関数_整数 = 無演算命令();
 let 関数_ポインター =無演算命令;

	println_all! {
		["大域的　 　　　: {: >16p}",&大域的 as *const i32]
		["ローカル_文字列: {: >16p}",ローカル_文字列 as *const str]
		["ローカル_整数　: {: >16p}",&ローカル_整数 as *const i32]
		["ボックス_文字列: {: >16p}",Box::into_raw(ボックス_文字列)]
		["ボックス_整数　: {: >16p}",Box::into_raw(ボックス_整数)]
		["関数_整数　　　: {: >16p}",関数_整数]
		["関数_ポインター: {: >16p}",関数_ポインター as *const ()]
	}
}
