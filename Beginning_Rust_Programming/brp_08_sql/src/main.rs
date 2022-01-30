extern crate sqlite;
use sqlite::Connection;
use std::{env, error::Error};

mod dbfuncs;

fn main() -> Result<(), Box<dyn Error>> {
	let conn = Connection::open("stratapp.db")?;

	conn.execute(
		"Create TABLE IF NOT EXISTS findings (
			findings_ID INTEGER PRIMARY KEY,
			title TEXT NOT NULL,
			finding TEXT NOT NULL,
			details TEXT,
			justification TEXT)",
	)?;
	let args: Vec<String> = env::args().collect();
	match args.len() > 1 {
		true => {
			let command: &str = &args[1];
			match command {
				"add" => dbfuncs::addrecord(&conn)?,
				"list" => dbfuncs::listrecords(&conn),
				_ => println!(">Didn't send a valid command in"),
			}
		},
		false=>println!("Please specify add or list as a command line parameter"),
	}
	Ok(())
}
