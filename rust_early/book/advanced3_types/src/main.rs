use std::fmt;
//use std::io::Error;

type Result<T> = std::result::Result<T, std::io::Error>;

trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn generic<T: ?Sized>(t: &T) {}

fn main() {
    //let guess: u32 = match guess.trim.parse() {
    //    Ok(num) => num,
    //    Err(_) => continue,
    //};

    //let guess2 = match guess2.trim(),parse() {
    //    Ok(_) => 5,
    //    Err(_) => "hello",
    //};
}
