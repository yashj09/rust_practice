use std::io;

fn main() {
    println!("Guess the Number");

    println!("Please input a number");

    let mut num= String::new();

    io::stdin().read_line(&mut num).expect("input needed");

    println!("You Guessed: {num}")
}
