use std::collections::HashMap;

#[derive(PartialEq,Eq,Hash)]
struct Account<'a>{username:&'a str,password:&'a str,}
struct AccountInfo<'a>{name:&'a str,email:&'a str,}
type Accounts<'a>=HashMap<Account<'a>,AccountInfo<'a>>;

    fn try_logon<'a>(accounts:&Accounts<'a>,username:&'a str,password:&'a str){
println!("Username: {}\nPassword: {}\nAttepting logon...",username,password);
let logon=Account{username,password,};
match accounts.get(&logon){
    Some(account_info)=>{println!(
        "Successful logon!\nName {}\nEmail: {}",account_info.name,account_info.email)},
    _=>println!("Logon failed!"),}
}
    fn call(number:&str)->&str{
match number{
    "798-1364"=>
        "We're sorry, the call cannot be completed as dialed.\n  Please hang up and try again.",
    "645-7689"=>
        "Hello, this is Mr.Awesome's Pizza. My name is Fred.\n  What can I get for you today?",
    _=>"Hi! Who is this again?"}
}

    fn main() {
let mut contacts=HashMap::new();
contacts.insert("Daniel","798-1364");
contacts.insert("Ashley","646-7689");
contacts.insert("Katie","435-8291");
contacts.insert("Robert","956-1745");

match contacts.get(&"Daniel") {Some(&number)=>println!("Calling Daniel: {}",call(number)),
                               _=>println!("Don't have Daniel's number."),}
contacts.insert("Daniel","164-6743");
match contacts.get(&"Ashley"){Some(&number)=>println!("Calling Ashley: {}",call(number)),
                              _=>println!("Don't have Ashley's number."),}
contacts.remove(&"Ashley");
for (contact,&number) in contacts.iter() {println!("Calling {}: {}",contact,call(number));}

//HashMap
let mut accounts:Accounts=HashMap::new();
let account=Account{username:"j.everyman",password:"password123",};
let account_info=AccountInfo{name:"John Everyman",email:"j.everyman@gmail.com",};

accounts.insert(account,account_info);

try_logon(&accounts,"j.everyman","psasword123");
try_logon(&accounts,"j.everyman","password123");
}
