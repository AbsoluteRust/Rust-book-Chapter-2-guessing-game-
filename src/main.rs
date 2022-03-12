//std neabs "standard library" and its refering to the "input/output" library
use std::io; 
// using rand crate , specifically random number generator
use rand::Rng; 
use std::cmp::Ordering;

///main function (the first function that will always be called in any rust exectuion)
fn main() { 
    println!("Guess the number!");

    //defines unmutable variable "secret_number" as a random number using dependency found in cargo.toml between 1 and 101, inclusive on lower bound but exclusive on upper bound
    let secret_number = rand::thread_rng().gen_range(1..101); 

     ///loops until correct guess
    loop {
        println!("Please input your guess.");

        // defining a mutable variable called "guess" as a new (empty) string
        let mut guess = String::new(); 

        // calling the "stdnin" function from the io library specified in line 2
        io::stdin() 

            //read_line method being called, passing "&mut guess" as the argument to read_line which takes whatever the user types into a string (without overwriting whatever exists in the string already). &mut necessary to make it mutable.
            .read_line(&mut guess) 
            .expect("Failed to read line"); 

        /// trim removes whitespace, u32 makes sure its numerical, the rest of this ensures its correct type of input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        /// "cmp" compares 2 methods , "match" expression decides what to do based on which variant of ordering is returned, "&secret_number" tells guess to reference the correct variable
        match guess.cmp(&secret_number) { 

            //if guess is less than the secret number
            Ordering::Less => println!("Too small!"), 

            //if guess is greater than the secret number
            Ordering::Greater => println!("Too big!"),

            Ordering::Equal => {
                //if guess is the secret number
                println!("bing bong u win lmao!"); 
                //End if correct guess
                break; 
            }
        }
    }
}
