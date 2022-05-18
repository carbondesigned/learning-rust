use rand::Rng;

// ordering type is another enum
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // range(start..end)
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        // :: looks cool, it just means that it is an associated function of the String type
        io::stdin()
            // even though the referenced var is mutable, references are immutable by default as well.
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadow of variable "guess"
        // removes the need for separate vars: i.e. "guess_str" and "guess"
        // parse turns a string into some kind of number.
        // u32 is a good default choice for a small positive number.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // cmp == compare
        // A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("TOO BIG!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
