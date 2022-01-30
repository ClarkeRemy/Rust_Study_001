#![allow(dead_code)]
#![allow(unused)]

fn main() {
    let data_stack = Vec::<isize>::new();
    let code = "5 +4* _378 ";// answer will be 17  
    let char_vec=code.chars().collect::<Vec<_>>();

    let what_if = "_9+10";
println!("{}",parse_then_clear(&mut "45_".chars().collect::<Vec<_>>()));
    
}

fn interpret(x:&[char])->isize{
    let mut reader = x.len();

    const V:fn() -> Vec<char> = Vec::<char>::new;
    const P:fn(&mut Vec<char>, char) = Vec::<char>::push;

    let (ref mut l, ref mut f, ref mut r) = (V(),V(),V());

    let (mut lv, mut rv):(isize,isize);
    if reader==0{return 0} // this is bad

    let top =|x:&mut Vec<char>|x[x.len()-1];

    // first value and function
    rv = loop {
        reader -= 1;
        match x[reader] {
            ' ' if r.len() ==0 => (),
            val  @ '0'..='9' => P(r,val),
            sign @ ('_' | ' ') if top(r).is_digit(10) => P(r,sign),
            ' ' if (top(r) == ' ') || (top(r) == '_') => (),
            func @ ('+' | '-' | '*' | '%') => {
                P(f,func);
                if top(r) != '_' && top(r).is_digit(10) {P(r,' ')};
                break parse_then_clear(r) },
            err=> panic!("failed to parse : {}",err),
        }
    };
0 // this is bad
}

fn parse_then_clear(x:&mut Vec<char>)->isize{
    let sign = match x.pop().expect("Empty vectors are not allowed") {
        ' '=>  1,
        '_'=> -1,
        _  =>panic!("[ The head must be '_' for + or ' ' for - ]")};
    let mut acc = 0;
    loop {
    acc += x.pop().expect("not empty").to_digit(10).expect("must be 0-9")*10u32.pow(x.len() as u32);
    if x.len() == 0 {break sign*acc as isize}
    }
}