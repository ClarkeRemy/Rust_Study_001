fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum ChatMessage {
    Hello { id: i32 },
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let a = 1;

    match a {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //part 2

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // part 3

    let b = 1;

    match b {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let c = 5;

    match c {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let ascii_letter = 'c';

    match ascii_letter {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: e, y: f } = p;
    assert_eq!(0, e);
    assert_eq!(7, f);

    let np = Point { x: 1, y: 8 };

    let Point { x, y } = np;
    assert_eq!(1, x);
    assert_eq!(8, y);

    let nextp = Point { x: 0, y: 7 };

    match nextp {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // part 4

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // part 5

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_setting_value = Some(10);

    match (setting_value, new_setting_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_setting_value;
        }
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }

    let _t = 5;
    let u = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point3d { x: 0, y: 0, z: 0 };

    match origin {
        Point3d { x, .. } => println!("x is{}", x),
    }

    let numbers2 = (2, 4, 8, 16, 32);

    match numbers2 {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let h = Some(5);
    let j = 10;

    match h {
        Some(50) => println!("Got 50"),
        Some(n) if n == j => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x2 = 4;
    let y2 = false;

    match x2 {
        4 | 5 | 6 if y2 => println!("yes"),
        _ => println!("no"),
    }

    let msg = ChatMessage::Hello { id: 5 };

    match msg {
        ChatMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        ChatMessage::Hello { id: 10..=12 } => println!("found an id in another range"),
        ChatMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}
