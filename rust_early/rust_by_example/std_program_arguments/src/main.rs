use std::env;

fn increase(number: i32){println!("{}",number+1);}
fn decrease(number: i32){println!("{}",number-1);}
fn help(){println!(
    "usage:\nmatch_args <string>\n    Check whether given string is the answer.\n\
    match_args {{increase|decrease}} <integer>\n    Increase or decrease given interger by one")}

fn main() {
let args:Vec<String>=env::args().collect();
//println!("My path is {}.",args[0]);
//println!("I got {:?} arguments: {:?}.",args.len()-1, &args[1..]);
match args.len() {
    1=>println!("My name is 'match_args'. Try passing some arguments!"),
    2=>match args[1].parse(){Ok(42)=>println!("This is the answer!"),
    _=>println!("This is not the answer."),},
    3=>{let cmd=&args[1];let num=&args[2];
        let number:i32=match num.parse(){Ok(n)=>{n},Err(_)=>{
            eprintln!("error: second argument not an integer");
            help();return;},};
        match &cmd[..]{
            "increase" => increase(number),"decrease"=>decrease(number),
            _=>{eprintln!("error: invalid command");help()},}},
    _=>help(),}
}
