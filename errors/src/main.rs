
use std::error::Error;
use std::fs::File;
use std::{fs, io};
use std::io::ErrorKind;

fn main() -> Result<(), Box<dyn Error>> {
    // basics();

    let txt = return match read_username_from_file() {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    };

    // generally, expect and unwrap should not be prod grade
    // always handle errors exhaustively
    basics();

    // Ok(());
}

fn basics() {
    // fucks shit up - critical error
    // panic!("crash and burn");

    // get backtrace
    // RUST_BACKTRACE=1 cargo run

    // enum Result<T, E> { Ok(T), Err(E), }

    // EXAMPLES
    let f = File::open("hello.txt");

    // HANDLE CRITICAL
    // let f =  match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening file {:?}", error),
    // };

    // HANDLE SPECIFIC + create if missing
    let f = f.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file {:?}", e),
        }
        other_error => {
            panic!("Problem opening the file: {:?}", other_error)
        }
    });

    // this:
    // let f1 = File::open("hello.txt").unwrap();

    // same as this:
    // let f1 = match f1 {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    let f1 = File::open(("hello.txt")).expect("Couldn't find the file");
}

// error propagation best practice - pass error higher
// cleanest version, otherwise exhaustive syntax
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}