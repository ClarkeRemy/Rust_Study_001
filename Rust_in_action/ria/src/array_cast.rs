#[allow(dead_code)]
pub fn arr_print() {
	let x:[u8; 5] = [1, 3, 4, 5, 9];
	let y = x.map(|x:u8| x as u16);
	println!("{:?}", y);
}
