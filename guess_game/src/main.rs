use std::io;
use rand:: RngExt;
use std::cmp::Ordering;
fn main() {
    println!("Guess the Number");
let secret= rand::rng().random_range(1..=100);
println!("the secret number is {secret}");


    println!("Please input a number");

    let mut num= String::new();

    io::stdin().read_line(&mut num).expect("input needed");
    let num:u32 = num.trim().parse().expect("need to be a number");

    println!("You Guessed: {num}");

    match num.cmp(&secret)  {
        Ordering::Less =>println!("number too small"),
        Ordering::Greater=> println!("number too big"),
    Ordering::Equal => println!("You Win!")
    }
}
