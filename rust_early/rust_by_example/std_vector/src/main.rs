use std::str;

    fn main() {
let collected_iterator: Vec<i32>=(0..10).collect();
println!("Collected (0..10) into: {:?}",collected_iterator);

let mut xs=vec![1i32,2,3];
println!("Initial vector: {:?}",xs);

println!("Push 4 into the vector");xs.push(4);
println!("Vector: {:?}",xs);

//collected_iterator.push(0);

println!("Vector length: {}",xs.len());
println!("Second element: {}",xs[1]);
println!("Pop last element: {:?}",xs.pop());
//println!("Fourth element: {}",xs[3]);
println!("Contents of xs:");for x in xs.iter(){println!("> {}",x);}

for (i,x) in xs.iter().enumerate(){println!("In position {} we have value {}",i,x);}
for x in xs.iter_mut(){*x*=3;};println!("Updated vector:{:?}",xs);

// String is Vec<u8> of valid UTF-8
// &str is &[u8] of valid UTF-8

let pangram:&'static str="the quick brown fox jumps over the lazy dog";
println!("Pangram: {}",pangram);

println!("Words in reverse");for word in pangram.split_whitespace().rev(){println!("> {}",word);}

let mut chars:Vec<char>=pangram.chars().collect();
chars.sort();chars.dedup();

let mut string=String::new();for c in chars {string.push(c);string.push_str(", ");}

let chars_to_trim:&[char]=&[' ',','];
let trimmed_str:&str=string.trim_matches(chars_to_trim);println!("Used characters: {}",trimmed_str);

let alice=String::from("I like dogs");println!("Alice says: {}",alice);
let bob: String=alice.replace("dog","cat");println!("Bob says: {}",bob);

// literals and escapes

let byte_escape = "I'm writing \x52\x75\x73\x74!"; // hex
println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

let unicode_codepoint="\u{211D}"; // unicode
let character_name="\"DOUBLE-STRUCK CAPITAL R\"";

println!("Unicode character {} (U+211D) is called {}",unicode_codepoint,character_name);

let long_string="String literals \
                  can span multiple lines.\n\
                  The linebreak and indentation here ->\

                  <- can be escaped too!";
println!("{}",long_string);

let raw_str=r"Escapes don't work here: \x3F \u{211D}";println!("{}", raw_str);

let quotes=r#"And then I said: "There is no escape!""#;println!("{}",quotes);

let longer_delimiter=r###"A string with "# in it. And even "##!"###;println!("{}",longer_delimiter);

// for non-UTF-8

let bytestring:&[u8;21]=b"this is a byte string";
println!("A byte string: {:?}",bytestring); // no display

let escaped=b"\x52\x75\x73\x74 as bytes"; // no unicode excapes
println!("Some escaped bytes: {:?}", escaped);

let raw_bytestring=br"\u{211D} is not escaped here";println!("{:?}",raw_bytestring);

if let Ok(my_str)=str::from_utf8(raw_bytestring) {println!("And the same as text: '{}'",my_str);}

let _quotes=br#"You can also use "fancier formating, \
            like with normal raw strings"#;

// SHIFT-JIS
let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // ようこそ

match str::from_utf8(shift_jis) {
    Ok(my_str)=>println!("Conversion successful: '{}'",my_str),
    Err(e)=>println!("Conversion failed: {:?}",e),
};
    }
