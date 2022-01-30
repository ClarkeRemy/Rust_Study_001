// An attribute to hide warnings for unsused code.
#![allow(dead_code)]

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // concrete type `T` is prefferedover a match on a reference `&T`
        match *self {
            // Can't take ownership of the trait, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}

// enum wth implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// Create a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    //Acess the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    //Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Acess the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let corner1 = Point { x: 1.2, y: 3.7 };
    let corner2 = Point { x: 15.3, y: -1.1 };
    let rec = Rectangle {
        top_left: corner1,
        bottom_right: corner2,
    };
    println!("Area of rectangle rec: {}", rect_area(rec));

    let sq: Rectangle = square(Point { x: 32.5, y: 19.2 }, 5.25f32);

    println!("Area of square sq: {}", rect_area(sq));

    // Beginning of Enums

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned 'String' from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;

    // Begin of use chapter

    // Explicitly `use` each name so they are availible without
    // manual scoping.
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside 'Work'.
    use crate::Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    let work = Civilian;

    match status {
        //Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    // C-like `enums`

    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    print!("roses are #{:06x}", Color::Red as i32);
    print!("violets are #{:06x}", Color::Blue as i32);

    // Testcase linked list

    // Create an empty linked list
    let mut list = List::new();

    // Prepend some element
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    let n = 16;

    //Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    //Error! Cannot modify a `const`.
    // `THRESHOLD = 5;`
}
// End of main

fn rect_area(rec: Rectangle) -> f32 {
    let Rectangle {
        top_left: func_top_left,
        bottom_right: func_bottom_right,
    } = rec;
    let Point { x: x1, y: y1 } = func_top_left;
    let Point { x: x2, y: y2 } = func_bottom_right;
    ((x2 - x1) * (y2 - y1)).abs()
}

fn square(p: Point, side_length: f32) -> Rectangle {
    let Point {
        x: lower_left_x,
        y: lower_left_y,
    } = p;
    Rectangle {
        top_left: Point {
            x: lower_left_x,
            y: lower_left_y + side_length,
        },
        bottom_right: Point {
            x: lower_left_x + side_length,
            y: lower_left_y,
        },
    }
}

// A function which takes a `WebEvent` as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into 'x' and 'y'.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}
