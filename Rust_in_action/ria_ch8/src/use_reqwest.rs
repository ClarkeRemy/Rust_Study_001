#![allow(unused,dead_code)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
let url="http://www.rustinaction.com/";
let mut response = reqwest::get(url)?;

let content = response.text()?;
println!("{}", content);

Ok(())}
