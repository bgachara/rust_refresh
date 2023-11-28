//recoverable and unrecoverable errors.
//Result<T,E>
//print the error, unwind and clean up the stack, quit
//env var to display call stack to easily trace error.
//aborting dies immediately without cleanup.
//panic! used for unrecoverable errors, can be explicit of otherwise.
//expect used more than unwrap for debugging purposes.
//propagating errrors using the question mark operator and one can chain the calls to it, used for values where return values are compatible with value used on
use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_file = File::open("open.txt").expect("Herein lies the error!!");

    // let man_file = match greeting_file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("open.txt") {
    //             Ok(f) => f,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    println!("This is from the Errors package.");

    let v = vec![1, 2, 3];

    println!("This is {}", v[1]);
}
