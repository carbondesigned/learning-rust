use std::{
    fs::{File, self},
    io::{self, ErrorKind, Read},
};

fn main() {}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//
//     // no need to call `return` because it is the last expression in the function.
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    //
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    //
    // Ok(s)

    // even shorter
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
