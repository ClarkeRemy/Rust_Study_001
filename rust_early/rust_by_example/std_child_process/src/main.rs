use std::io::prelude::*;
use std::process::{Command,Stdio};

static PANGRAM:&'static str="the quick brown fox jumped over the lazy dog\n";

fn main() {
let output=Command::new("rustc").arg("--version").output().unwrap_or_else(|e| {
    panic!("failed to execute process: {}", e)});
if output.status.success() {let s=String::from_utf8_lossy(&output.stdout);
    print!("rustc succeeded and stdout was:\n{}",s);
}else{let s=String::from_utf8_lossy(&output.stderr);print!("rustc failed and stderr was:\n{}",s);}

// Pipes

let process=match Command::new("wc").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn(){
    Err(why)=>panic!("couldn't spawn wc: {}", why),Ok(process)=>process,};
match process.stdin.unwrap().write_all(PANGRAM.as_bytes()){
    Err(why)=>panic!("couldn't write to wc stdin: {}",why),Ok(_)=>println!("sent pangram to wc"),}
let mut s=String::new();
match process.stdout.unwrap().read_to_string(&mut s){
    Err(why)=>panic!("couldn't read wc stdout: {}",why),
    Ok(_)=>print!("wc responded with:\n{}",s),}

// wait

let mut child = Command::new("sleep").arg("5").spawn().unwrap();
let _result = child.wait().unwrap();
println!("reached end of main");
}
