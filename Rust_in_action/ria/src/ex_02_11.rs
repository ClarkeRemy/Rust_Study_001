#[allow(dead_code)]
pub fn run() {
	use clap::{App, Arg};
	use std::{
		fs::File,
		io::{prelude::*, BufReader},
	};

	let args = App::new("grep-lite").version("0.1")
	                                .about("searches for patterns")
	                                .arg(Arg::with_name("pattern").help("The pattern to search for")
	                                                              .takes_value(true)
	                                                              .required(true))
	                                .get_matches();

	let pattern = args.value_of("pattern").unwrap();
	let re = regex::Regex::new(pattern).unwrap();

	let f = File::open(args.value_of("input").unwrap()).unwrap();
	let reader = BufReader::new(f);

	for line_ in reader.lines() {
		let line = line_.unwrap();
		match re.find(&line) {
			Some(_) => println!("{}", line),
			None => (),
		}
	}
}
