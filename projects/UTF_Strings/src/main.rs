fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let data = "initial contents";
    let ss = data.to_string();
    let ss = "initial contents".to_string();

    let sss = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut a = String::from("lo");
    a.push('l');
    println!("{}", &a);

    let b1 = String::from("Hello, ");
    let b2 = String::from("world!");
    let b3 = b1 + &b2; //note that b1 has been moved here and can no longer be used
    println!("{}", &b3);

    let c1 = String::from("tic");
    let c2 = String::from("tac");
    let c3 = String::from("toe");

    let c = c1 + "-" + &c2 + "-" + &c3;
    println!("{}", &c);

    let d1 = String::from("tic");
    let d2 = String::from("tac");
    let d3 = String::from("toe");

    let d = format!("{}-{}-{}", d1, d2, d3);
    println!("{}", &d);

    let e1 = String::from("hello");
    //let h = s1[0]; (Rust won't allow this)

    let hello = "Здравствуйте";
    let f =&hello[0..4];
    println!("{}",&f);

    for c in "Здравствуйте".chars() {
        println!("{}", c);
    };


    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    };
}
