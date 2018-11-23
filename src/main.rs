// we need to get input and output data
// import the io library from the standard library
use std::io;

fn main() {
    println!("Guess the number!");
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
