use contracts::*;



	#[requires(x > 0, "x was not sufficiently large")]
	#[ensures(x < 15, "x is too large")]
	fn do_something(mut x:i32){
		println!("{}", x);
		x=25;
	}

fn main() {
	do_something(4);
}
