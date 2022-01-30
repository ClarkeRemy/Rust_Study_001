#![allow(unreachable_code)]

fn main() {
    // if & else
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`.
        10 * n
    } else {
        println!(", and is a big number, halve tthe number");

        // This expression must return an `i32` as well.
        n / 2
        // TODO ^ Try expression this expression with a semicolon. (it won't return)
    };
    // ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);

    // Loop

    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            // break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    // Return from loops

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // while

    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

    // for loops
    // for and range

    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // inclusive range version
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // for and iterators

    // iter will borrow
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"), // notice the borrowing (&)
            _ => println!("Hello {}", name),
        }
    }
    // into_iter will consume the collection
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // iter mut can be modified in place
    let mut mut_names = vec!["Bob", "Frank", "Ferris"];

    for name in mut_names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!", // notice the mut borrowing (&mut)
            _ => "Hello",
        }
    }

    println!("names: {:?}", mut_names);

    //match

    let num = 13;
    // TODO ^ try different values for `num`

    println!("Tell me about {}", num);
    match num {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match in an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    for integer in 1..20 {
        match integer {
            // Match a single value
            1 => println!("One!"),
            // Match several values
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            // Match in an inclusive range
            13..=19 => println!("A teen"),
            // Handle the rest of cases
            _ => println!("Ain't special"),
        }
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
        // the compiler will complain that you avent done every case
    };

    println!("{} -> {}", boolean, binary);
}
