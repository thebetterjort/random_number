use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
     let secret_number = rand::thread_rng().gen_range(1..101);
    //let foo = 5; // immutable
    //let mut bar = 5; // mutable
    println!("The secret number is {}", secret_number);
    //the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("You Guessed: {}", guess)
}
