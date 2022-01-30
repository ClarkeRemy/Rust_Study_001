#![allow(unused_variables)]
/*fn main() {
    let mut x=5;
    println!("The value of x is: {}", x);
    x=6;
    println!("The value of x is: {}", x);
}*/

fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    let x = 2.0; //f64
    let y: f32 = 3.0; //f32
    println!("The value of x is :{}", x);
    println!("The value of y is :{}", y);
    //let spaces ="    ";
    //let spaces = spaces.len();
    let guess: u32 = "42".parse().expect("Not a number!");

    let sum = 5 + 10;
    println!("The value of sum is :{}", sum);
    let difference = 95.5 - 4.3;
    println!("The value of difference is :{}", difference);
    let product = 4 * 30;
    println!("The value of product is :{}", product);
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is :{}", quotient);
    let remainder = 43 % 5;
    println!("The value of remainder is :{}", remainder);

    let t = true;
    let f: bool = false; //with explicit type annotation
    println!("The value of t is :{}", t);
    println!("The value of f is :{}", f);

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is :{}", heart_eyed_cat);
    println!("The value of c is :{}", c);
    println!("The value of z is :{}", z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is :{:?}", tup);
    let (x, y, z) = tup;
    println!("The value of y is :{}", y);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is :{}", five_hundred);
    println!("The value of six_point_four is :{}", six_point_four);
    println!("The value of one is :{}", one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is :{:?}", a);
    let months = [
        "January",
        "Febuary",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("The value of months is :{:?}", months);
    let w = [3; 5];
    println!("The value of w is :{:?}", w);
    let first = a[0];
    let second = a[1];
    println!("The value of first is :{}", first);
    println!("The value of second is :{}", second);
    /*let index =10;
    let element=a[index];
    println!("The value of element is :{}", element);*/
    //last part not possible
}
