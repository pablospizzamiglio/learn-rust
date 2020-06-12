// Declares imports
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generates a number between 1 and 101 (the second parameter is
    // non-inclusive)
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Creates an infinite loop
    loop {
        println!("Please input your guess.");

        // Creates a new mutable variable of type `String`
        let mut guess = String::new();

        // Reads user input and tries to store it in the previously defined
        // variable `guess`, if the action fails it panics and shows an error
        // message
        // Returns a `Result`, which is an enumeration that can contain a value
        // or an error
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Removes any trailing white space character
        // Tries to parse the `String` to get a `u32` integer.
        // The parse method returns an enum that can contain a value or an error
        // The match expression is composed by branches, each branch represents
        // a pattern that will only execute if the result fits one of the
        // patterns
        // Changes the variable type from `String` to `u32` and makes it
        // immutable, this is called `shadowing`. This technique is usually
        // used when converting values
        let guess: u32 = match guess.trim().parse() {
            // Extracts the value from `Result` if the parsing was successful
            Ok(num) => num,
            // Skips the current iteration if we got an error
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // The `cmp` method compares two values and can be invoked on anything
        // that can be compared
        // Again we use a match expression to evaluate `cmp` results and
        // determine the current result of the game
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
