#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}  {:?}",six, five);

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    match some_u8_value {
        Some(1) => println!("one"),
        Some(3) => println!("three"),
        Some(5) => println!("five"),
        Some(7) => println!("seven"),
        _ => (),
    };

    let coin = Coin::Penny;
    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}", state),
    //     _ => count+=1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count+=1;
    }


}
