use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

// enum Result<T,E> {
//     Ok(T),
//     Err(E),
// }

//fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem opening the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file {:?}", error);
    //     }
    // });

    // let t = File::open("hello.txt").unwrap();
    // let t = File::open("hello.txt").expect("Failed to open hello.txt");





    // fn read_username_from_file() -> Result<String, io::Error> {
        // let f = File::open("hello.txt");

        // let mut f = match f {
        //     Ok(file) => file,
        //     Err(e) => return Err(e),
        // };

        // let mut s = String::new();

        // match f.read_to_string(&mut s) {
        //     Ok(_) => Ok(s),
        //     Err(e) => Err(e),
        // }





        // let mut f = File::open("hello.txt")?;

        // let mut s = String::new();
        // f.read_to_string(&mut s)?;
        // Ok(s)



    //     let mut s = String::new();
    //     File::open("hello.txt")?.read_to_string(&mut s)?;

    //     Ok(s)
    // };


//}



fn main() -> Result<(), Box<dyn Error>> {

    let f = File::open("hello.txt")?;

    Ok(())
}
