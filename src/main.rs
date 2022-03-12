use std::io; //std neabs "standard library" and its refering to the "input/output" library
use rand::Rng; // using rand crate , specifically random number generator
use std::cmp::Ordering;

fn main() { //main function (the first function that will always be called in any rust exectuion)
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); //defines unmutable variable "secret_number" as a random number using dependency found in cargo.toml between 1 and 101, inclusive on lower bound but exclusive on upper bound

    loop { //loops until correct guess
        println!("Please input your guess.");

        let mut guess = String::new(); // defining a mutable variable called "guess" as a new (empty) string

        io::stdin() // calling the "stdnin" function from the io library specified in line 1
            .read_line(&mut guess) //read_line method being called, passing "&mut guess" as the argument to read_line which takes whatever the user types into a string (without overwriting whatever exists in the string already). &mut necessary to make it mutable.
            .expect("Failed to read line"); 
        
        let guess: u32 = match guess.trim().parse() {// trim removes whitespace, u32 makes sure its numerical
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // "cmp" compares 2 methods , "match" expression decides what to do based on which variant of ordering is returned, "&secret_number" tells guess to reference the correct variable
            Ordering::Less => println!("Too small!"), //if guess is less than the secret number
            Ordering::Greater => println!("Too big!"), //if guess is greater than the secret number
            Ordering::Equal => {
                println!("bing bong u win lmao!"); //if guess is the secret number
                break; //end if correct guess
            }
        }
    }
}
