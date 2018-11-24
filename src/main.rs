// importing an external dependency/library
extern crate rand;

// we need to get input and output data
// import the io library from the standard library
use std::io;
use std::cmp::Ordering;
use rand::Rng; // using trait to bring the generator of number in scope


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess:");

    loop {
        // variable are immutable by default, so to
        // make it mutable, you use mut before the variable
        let mut guess = String::new();

        // & allowing us to reference a data without copying it
        // into memory again also using mut to make it mutable
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You need to input an integer");
                continue;
            },
        };


        println!("You guessed: {}", guess);

        // do the comparison
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed right!!");
                break;
            },
        }
    }
}
