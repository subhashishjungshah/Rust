// using standard input/output library inside a rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // This line is a inclusive range where I get secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // created a mutable variable that is currently bound to a new, empty instance of a String
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Some error occured");

        // Type casting guess from string to u32

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Breaking the loop
            }
        }
    }
}
