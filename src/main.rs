extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    print!("\n");
    println!("The secret number is: {}", secret_number);
    print!("\n");
    println!("Please input your guess.");
    print!("\n");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    print!("\n");
    println!("You memory adress of a guess: {:?}", &guess as *const _);
    print!("\n");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    print!("\n");
    println!(
        "You guessed: **** {} ** {:p} ****",
        guess, &guess as *const _
    );
}
