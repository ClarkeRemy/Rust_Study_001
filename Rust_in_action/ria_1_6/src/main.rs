fn main() {
    use std::{thread::{spawn,sleep},time::Duration as D};
    let mut data = 100u32;
    spawn(move||{data=500;});
    spawn(move||{data=1000;});
    sleep(D::from_secs(3));

    println!("{}", data);
}
