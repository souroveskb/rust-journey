use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_num}");
    loop {
            
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide an integer numeber");
                continue;
            },
        };

        // let guess : u32 = guess.trim().parse().expect("Please provide an integer number");


        println!("You have guessed: {guess}");

        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("EQUAL!!");
                break;

            }
            Ordering::Greater => println!("Too big"),
        };
    }
}
