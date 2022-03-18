use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_num: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input a guess between 1 and 100");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Greater => {
                println!("too big");
            }
        }
    }

    println!("the number was {}", secret_num);
}
