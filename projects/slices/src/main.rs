fn main() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

/*fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    //s.clear();

    println!("the first word is: {}", word);

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice0 =&s[0..2];
    let slice1 =&s[..2];

    let len =s.len();

    let slice2 = &s[3..len];
    let slice3 = &s[3..];

    let slice4 = &s[0..len];
    let slice5 = &s[..];
}*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
