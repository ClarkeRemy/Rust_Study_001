fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_lenght(s1);

    println!("The lenght of '{}' is {}.", s2, len);
}

fn calculate_lenght(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}