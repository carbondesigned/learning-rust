use std::{fs::File, io::ErrorKind};

fn main() {

    // If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
    // If the Result is the Err variant, unwrap will call the panic! macro for us.
    let f = File::open("hello.txt").unwrap();

    // works similarly to unwrap() but expect uses a manual error message
    // let f = File::open("hello.txt").expect("a good error message");

    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         // looking for a file that doesn't exist
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem Creating file: {:?}", e),
    //         },
    //         other_err => {
    //             panic!("Problem opening file: {:?}", other_err)
    //         }
    //     },
    // };
}
