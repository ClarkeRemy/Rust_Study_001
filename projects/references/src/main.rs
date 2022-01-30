fn main() {
    part1();

    part2();

    part3();
}

fn part1() {
    let s1 = String::from("hello");

    let len = calculate_lenght(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}

fn part2() {
    let mut s = String::from("hello");

    change(&mut s);

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}, and ", r1, r2);

    let r3 = &mut s;
    println!("{}", r3)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!")
}

fn part3(){
    let reference_to_nothing=dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}