use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess = String::new();

    // :: looks cool, it just means that it is an associated function of the String type
    io::stdin()
        // even though the referenced var is mutable, references are immutable by default as well.
        .read_line(&mut guess)
        .expect("Failed to read line");
    print!("You guessed: {}", guess);
}
