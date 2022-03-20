use std::io::{self, Write}; // Import the io module
use std::cmp::Ordering;
use rand::Rng; // Import the Rng trait from the rand crate. cargo doc --open will also show documentation from dependencies.
// We are creating an executable crate. Dependencies are usually library crates (which contains code rather than executables).

fn main() { // Main function
    let min_num = 1; // Minimum number
    let max_num = 100; // Maximum number
    let max_attempts = 5; // Maximum number of attempts
    let mut guess_count = 0; // Number of guesses
    let mut won = false; // Did the user win?

    println!("Guess the number between {} and {}! You have {} attempts.", min_num, max_num, max_attempts); // Prints the string (println! is a macro, hence the bang [exclamation point])

    let secret_number = rand::thread_rng().gen_range(min_num..=max_num); // Generate a random number between 1 and 100 (could also use 1..=100)

    while guess_count < max_attempts { // While the guess count is less than the maximum number of attempts
        print!("Please input your guess: ");
        io::stdout().flush().unwrap(); // Flush the output buffer
        
        let mut guess = String::new(); // Variables are immutable by default. mut makes the variable mutable.
        
        io::stdin()
            .read_line(&mut guess) // & makes this a reference (i.e. don't copy the value, reuse the address). References are immutable by default, hence adding mut.
            .expect("Failed to read line"); // Read_line returns an io::Result, an enum that can be Ok or Err. It has an expect method that will panic if the result is Err.
            // If the result is Ok, expect will take value stored in Ok and return it (in this case it's the num of bytes of input).
    
        let guess: u32 = match guess.trim().parse() {  // Rust let's us shadow (overwrite) variables.
            Ok(num) => num, // If the parse method returns Ok, it will take the value stored in Ok and return it.
            Err(_) => continue, // If the parse method returns Err (underscore means all errors), it will go to the next iteration of the loop.
        };
        // Trim removes whitespace, new lines (\n), and return carriages (\r\n) from the beginning and end of the string. Parse converts the string to a number.
    
        // println!("You guessed: {}", guess);
        
        guess_count += 1;
        /*
        Match is basically like switch in other languages. 
        It takes a value and compares it to a list of patterns (it breaks automatically on a match).
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low! You have {} tries left.", max_attempts - guess_count),
            Ordering::Greater => println!("Too high! You have {} tries left.", max_attempts - guess_count),
            Ordering::Equal => {
                won = true;
                println!("You win! You guessed the secret number in {}/{} tries.", guess_count, max_attempts);
                break;
            },
        }
    }

    if (guess_count >= max_attempts) && !won {
        println!("You lose! The secret number was {}.", secret_number);
    }
}