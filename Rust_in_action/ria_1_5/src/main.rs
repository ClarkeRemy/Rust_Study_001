
#[rustfmt::skip]
mod lib {

#[derive(Debug)]
pub enum Cerial {Barley, Millet, Rice, Rye, Spelt, Wheat,}
}

#[rustfmt::skip]
fn main() {
    use lib::Cerial as C;
    let g=vec![].push(C::Rye);
    // drop(g);
    println!("{:?}", g);
}
