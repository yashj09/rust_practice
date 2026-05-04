use rand::RngExt;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number");
    let secret = rand::rng().random_range(1..=100);
    // println!("the secret number is {secret}");

    loop {
        println!("Please input a number");

        let mut num = String::new();

        io::stdin().read_line(&mut num).expect("input needed");

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {num}");

        match num.cmp(&secret) {
            Ordering::Less => println!("number too small"),
            Ordering::Greater => println!("number too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
