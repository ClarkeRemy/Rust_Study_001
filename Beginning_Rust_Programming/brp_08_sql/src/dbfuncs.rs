use sqlite;
use sqlite::{Connection, State};
use std::io;

pub fn addrecord(conn: &Connection) -> io::Result<()> {
	println!("Title");
	let mut title = String::new();
	io::stdin().read_line(&mut title)?;

	println!("Finding text");
	let mut finding = String::new();
	io::stdin().read_line(&mut finding)?;

	println!("Details of the finding");
	let mut details = String::new();
	io::stdin().read_line(&mut details)?;

	println!("Justification");
	let mut justification = String::new();
	io::stdin().read_line(&mut justification)?;

	let commandstring = format!(
		"INSERT INTO findings (title, finding, details, justification) VALUES (\"{}\", \"{}\", \"{}\", \"{}\")",
		title.trim(), finding.trim(),details.trim(),justification.trim()
	);
	conn.execute(&commandstring).unwrap();
	Ok(())
}

pub fn listrecords(conn: &Connection){
	let mut statement = conn.prepare("SELECT * FROM findings").unwrap();
	while State::Row==statement.next().unwrap(){
		println!("-----------------------------");
		println!("Title         = {}", statement.read::<String>(1).unwrap());
		println!("Finding       = {}", statement.read::<String>(1).unwrap());
		println!("Details       = {}", statement.read::<String>(1).unwrap());
		println!("Justification = {}", statement.read::<String>(1).unwrap());
	}
}