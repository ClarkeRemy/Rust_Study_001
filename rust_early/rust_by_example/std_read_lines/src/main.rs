use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename:P)->io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
let file=File::open(filename)?;
Ok(io::BufReader::new(file).lines())
}

fn main() {
if let Ok(lines) = read_lines("./hosts"){for line in lines {if let Ok(ip)=line{println!("{}",ip);}}}
}
