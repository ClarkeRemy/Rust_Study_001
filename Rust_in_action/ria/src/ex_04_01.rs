macro_rules! array_map {
  ($closure:expr, [ $($index:expr),* $(,)? ]) => {
    [$($closure($index),)*]
  };
}

#[allow(dead_code)]
pub fn run() {
	#[derive(Debug)]
	struct CubeSat {
		id:u64,
	}
	impl From<u64> for CubeSat {
		fn from(id:u64) -> Self { CubeSat { id } }
	}
	#[derive(Debug)]
	enum StatusMessage {
		Ok,
	}
	fn check_status(sat_id:CubeSat) -> CubeSat {
		println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
		sat_id
	}

	use CubeSat as CS;
	let [sat_a, sat_b, sat_c]:[CS; 3] = array_map!(|x| CS::from(x), [0, 1, 2]);

	let [sat_a, sat_b, sat_c]:[CS; 3] = array_map!(|x| check_status(x), [sat_a, sat_b, sat_c]);

	let wait = || {
		println!("waiting...");
		std::thread::sleep(std::time::Duration::from_secs(1));
	};
	wait();
	#[allow(unused_variables)]
	let [sat_a, sat_b, sat_c]:[CS; 3] = array_map!(|x| check_status(x), [sat_a, sat_b, sat_c]);
	wait();

	fn use_value<T>(_val:T) {}
	let a = 123;
	use_value(a);
	println!("{}", a);
	struct Demo {
		a:i32,
	}
	let demo = Demo { a };
	use_value(&demo);
	println!("{}", demo.a);
}
