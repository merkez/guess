extern crate rand;
use rand::random;
use std::io::stdin; 
mod module;
use module::foo;

fn get_guess() -> u8 {
    loop {
        println!("Input guess: ");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Could not read from stdin");
        match guess.trim().parse::<u8>() {
            Ok(v) => return v, 
            Err(e) => println!("Could not understand input: {}", e),
            
        }
    }

    
}

fn handle_guess(guess: u8, correct: u8) -> bool {
    if guess < correct  {
        println!("Too low.");
        false 
    }else if guess > correct {
        println!("Too high.");
        false 
    } else {
        println!("You got it! ");
        true
    }

}


/// This is a doc comment!! It gets documented. 
/// 
/// # Examples 
/// ```rust
/// println!("Hello!")
/// ````
pub fn tea() {}

fn main() {
    foo();
    println!("Welcome to the guessing game !");
  
    let correct = random::<u8>();
    // println!("Correct value is {}", correct);
   
   loop {
        let guess = get_guess();
        if handle_guess(guess, correct) {
            break; 
        } 
   }

}