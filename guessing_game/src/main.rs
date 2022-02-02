//io library is used to read user input from concole and print it there
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Type in your answer:");
        //let is used to introduce a new variable
        //variables in rust are if not otherwise declared immutable
        //if you want to make a variable mutable use tue 'mut' keyword
        //String::new() creates a new instance of an empty string
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");

        //shadowing of variable guess, shadowing is often used to parse one type into another
        //let guess: u32 = guess.trim().parse().expect("Please only type in numbers"); -> program stops after detecting illegal input
        //better handling which let program continue after wrong input:
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Oh no! Too low!"),
            Ordering::Greater => println!("Bye bye! Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
