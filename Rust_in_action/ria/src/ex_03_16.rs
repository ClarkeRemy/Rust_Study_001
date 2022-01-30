#[allow(dead_code)]
pub fn run() {
	//setup
	#[derive(Debug)]
	enum Event {
		Update,
		Delete,
		Unknown,
	}
	type Message = String;

	fn parse_log(line:&'static str) -> (Event, Message) {
		use {Event::*, Message as M};
		let parts:Vec<&str> = line.splitn(2, ' ').collect();
		match parts.len() {
			1 => (Unknown, M::from(line)),
			_ => {
				let rest = M::from(parts[1]);
				match parts[0] {
					"UPDATE" | "update" => (Update, rest),
					"DELETE" | "delete" => (Delete, rest),
					_ => (Unknown, M::from(line)),
				}
			}
		}
	}

	//begin run
	let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

	for line in log.lines() {
		let (e, m) = parse_log(line);
		println!("({:?}, {})", e, m);
	}
}
