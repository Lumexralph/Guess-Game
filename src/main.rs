// importing an external dependency/library
extern crate rand;

// we need to get input and output data
// import the io library from the standard library
use std::io;
use rand::Rng; // using trait to bring the generator of number in scope


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    println!("Please input your guess:");

    // variable are immutable by default, so to
    // make it mutable, you use mut before the variable
    let mut guess = String::new();

    // & allowing us to reference a data without copying it
    // into memory again also using mut to make it mutable
    io::stdin().read_line(&mut guess)
        .expect("Failed to readline");

    println!("You guessed: {}", guess);
}
