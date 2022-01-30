fn main() {
    println!("HELLO WOLRD FROM RUST");
    for i in 1..=1000 {
        println!("CURRENT STATE I{i}");
    }
    println!("DONE ....");
    println!("BEGINNING FACTORIAL",);
    let mut a:i128=1;
    for i in 1..=33{ 
        a*=i; 
    println!("FACT: {i} {a}");
    }
}
