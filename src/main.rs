use compare::{Compare, natural};
use rand::Rng;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;

fn main() {
    print!("{esc}c", esc = 27 as char);
    println!("Guess what the number I'm thinking of");

    let mut rng = rand::rng();
    let random = rng.random_range(1..100);

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let input_number: i32 = input.trim().parse().expect("Not a whole number buddy");

        println!("You guessed {}", input_number);
        // println!("I guessed {}", random);

        match input_number.cmp(&random) {
            Greater => println!("Too high!"),
            Less => println!("Too Low :("),
            Equal => {
                println!("YOU WON!");
                break;
            }
        }
    }
}
