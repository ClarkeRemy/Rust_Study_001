struct Point<T,U> {
    x: T,
    y: U,
}

impl<T,U> Point<T,U> {
    fn x(&self) -> &T {
        &self.x
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T,E> {
    Ok(T),
    Err(E),
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0};

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
