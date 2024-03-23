use rand::Rng;//random library is imported, also changed cargo.toml
use std::cmp::Ordering; //used to comapre two numbers
use std::io; //To take inputs from user

fn main() {
    println!("------------------ GUESS THE NUMBER GAME --------------");

    let secret_number = rand::thread_rng().gen_range(1, 101); //generate a number from 1-100
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 1)) let guess: u32 = guess.trim().parse().expect("Please type a number!");
        //if any other input is given other than an unsigned32 then the code is exited with the msg

        //2))
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //if any other input is given other than an unsigned32 then the code is not exited and asks again for the required input type
        

        println!("You guessed: {}", guess);


        // to check if the secret number and guessed number are greater, smaller or equal.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
