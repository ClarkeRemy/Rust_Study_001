use std::fmt;

struct City {
    name: &'static str,
    //Latitude
    lat: f32,
    //Longitude
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}: {:.3}\u{00B0}{} {:.3}\u{00B0}{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGB ({r}, {g}, {b}) 0x{r:>0width$X}{g:>0width$X}{b:>0width$X}",
            r = self.red,
            g = self.green,
            b = self.blue,
            width = 2
        )
    }
}

struct _Unprintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
#[allow(dead_code)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    println!("{} days", 31i64);
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
    println!("{number:>width$}", number = 1, width = 6);
    println!("{n:>0w$}", n = 1, w = 6);
    println!("My name is {0}, {1} {0}", "Bond", "James");

    println!("This struct `{:?}` won't print...", Structure(3));

    let pi: f64 = 3.141592;
    println!("Pi is roughly {}", pi);

    //part 2: Debug

    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);

    //part 3: Display

    let minmax = MinMax(0, 14);
    println!("Compare structures:\nDisplay: {0}\nDebug: {0:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "\nThe big range is {big} and the small is {small}\n",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:\nDisplay: {0}\nDebug: {0:?}", point);

    let complex = Complex {
        real: 5.6,
        imag: 14.0,
    };
    println!("Compare complex:\nDisplay: {0}\nDebug: {0:?}", complex);

    //part 4: Display List

    let v = List(vec![1, 2, 3]);
    println!("Display: {0}\nDebug:   {0:?}", v);

    //part 5: Formatting

    let dublin = City {
        name: "Dublin",
        lat: 53.347778,
        lon: -6.259722,
    };
    let oslo = City {
        name: "Oslo",
        lat: 59.95,
        lon: 10.75,
    };
    let vancouver = City {
        name: "Vancouver",
        lat: 49.25,
        lon: -123.1,
    };

    for city in [dublin, oslo, vancouver].iter() {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{:}", *color);
    }
}
