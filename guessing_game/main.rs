use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("The secret num is {}", secret_num);

    println!("Guess the num");
    
    loop {
        println!("Please input your guess:");
        
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read the line");

        let guess: u32 = match guess.trim()
                        .parse() {
                            Ok(num) => num,
                            Err(_) => continue,
                        };

        println!("Your guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
