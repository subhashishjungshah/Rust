// using standard input/output library inside a rust
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // created a mutable variable that is currently bound to a new, empty instance of a String
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Some error occured");

    println!("You guessed: {guess}");
}
