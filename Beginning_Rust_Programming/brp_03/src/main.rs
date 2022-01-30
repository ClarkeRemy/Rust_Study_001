extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize,Clone)]
struct Dvd {
	name: String,
	year: u16,
	cast: String,
	length: u16,
}
fn json_from_str(raw: &str) -> Dvd {
	serde_json::from_str(raw).unwrap()
}
fn str_from_json(dvd: &Dvd) -> String {
	serde_json::to_string(dvd).unwrap()
}
fn dvds_to_file(f: &String, d: Dvd) {
	serde_json::to_writer(OpenOptions::new().append(true).open(f).unwrap(), &d);
}
fn dvds_from_file(f: &String) -> Dvd {
	serde_json::from_reader(File::open(f).unwrap()).unwrap()
}
fn main() {
	let rawdata = r#"
	{
		"name": "La La Land",
		"year": 2016,
		"cast": "Emma Stone, Ryan Gosling",
		"length": 128
	}
	"#;
	let d = json_from_str(&rawdata);
	println!("{}", str_from_json(&d));
	let filename = String::from("file.json");
	dvds_to_file(&filename, d);
	println!("{}", str_from_json(&dvds_from_file(&filename)));

	let dvds:Vec<Dvd>=vec![json_from_str(&rawdata);4];
	for dvd in dvds.iter() {
		dvds_to_file(&filename, dvd.clone());
		println!("{}",str_from_json(&dvds_from_file(&filename)));
	}

}
