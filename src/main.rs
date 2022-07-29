// std --> standard library
// io --> library for user input/output
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("Secret number : {secret_number}");

    loop {
        println!("Please input your guess.");

        // Mutable variable
        // new() is a static function of String
        let mut guess = String::new();

        // read_line() takes user standard input and append to a string (w/o overriding). Returns a "Result" enum
        // &mut indicates that argument is a reference
        // *Note: references  are immutable by default
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadows the previous value of guess
        // trim() eliminiates whitespaces at beginning/end (\n , \r\n)
        // parse() turns into numbers. returns Result type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // ignore non-number guess
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // Each of these are called "arms"
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
