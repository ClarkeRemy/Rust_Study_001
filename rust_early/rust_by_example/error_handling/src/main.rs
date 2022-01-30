#![allow(dead_code)]

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

#[derive(Debug)]
enum Food_2 {
    CordonBleu,
    Steak,
    Sushi,
}
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

// We don't have the ingredients to make Sushi.
fn have_ingredients(food: Food_2) -> Option<Food_2> {
    match food {
        Food_2::Sushi => None,
        _ => Some(food),
    }
}

// We have the recipe for everything except Cordon Bleu.
fn have_recipe(food: Food_2) -> Option<Food_2> {
    match food {
        Food_2::CordonBleu => None,
        _ => Some(food),
    }
}

// To make a dish, we need the recipe and the ingredients.
// We can represent the logic with a chain of `match`es:
fn cookable_v1(food: Food_2) -> Option<Food_2> {
    match have_recipe(food) {
        None => None,
        Some(food) => match have_ingredients(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

// This can conveniently be rewritten more compactly with `and_then()`:
fn cookable_v2(food: Food_2) -> Option<Food_2> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat_2(food: Food_2, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

// Peeling food. If there isn't any, then return `None`.
// Otherwise, return the peeled food.
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

// Chopping food. If there isn't any, then return `None`.
// Otherwise, return the chopped food.
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// Cooking food. Here, we showcase `map()` insteadt of `match` for case handling.
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// A function to peel, chop, and cook food all in sequence.
// We chain multiple uses of `map()` to simplify the code.
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// Check whether there's food or not before trying to eat it!
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

struct Person {
    job: Option<Job>,
}
#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // Gets the area code of the phone_number of the person's job, if it exists.
    fn work_phone_area_code(&self) -> Option<u8> {
        // This would need many nested `match` statements without the `?` operator
        // It would take a lot more code - try writing it yourself and see which
        // is easier.
        self.job?.phone_number?.area_code
    }
}

fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }

    println!("Some refreshing {} is all I need.", beverage)
}
// The commoner has seen it all, and can handle any gift well.
// All gifts are handled wexplictly using `match`.
fn give_commoner(gift: Option<&str>) {
    // Specify a course of action for each case.
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner) => println!("No gift? Oh well."),
        None => println!("No gift? Oh well."),
    }
}
// Our sheltered royal will `panic` at the sight of snakes.
// All gifts are handled implictly using `unwrap`.
fn give_royal(gift: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s!!!!!", inside);
}
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // If `current_age`is `None`, this retruns `None`.
    // If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
    println!("Checking birthday");
    let next_age: u8 = current_age?;

    println!("birthday checked");
    Some(format!("Next year I will be {}", next_age))
}
fn main() {
    drink("water");
    // Intentional Panic
    // drink("lemonade");

    // 2

    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    // let nothing = None;

    give_royal(bird);
    // give_royal(nothing);

    // 3

    let current_age = None;
    next_birthday(current_age);

    // 4

    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));

    // 5

    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // Let's try the similar looking `process()` now.
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);

    // 6

    let (cordon_bleu, steak, sushi) = (Food_2::CordonBleu, Food_2::Steak, Food_2::Sushi);

    eat_2(cordon_bleu, Day::Monday);
    eat_2(steak, Day::Tuesday);
    eat_2(sushi, Day::Wednesday);

    // 7

    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}
