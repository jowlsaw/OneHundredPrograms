

use std::io;
use std::io::prelude::*;
use std::str;

fn main() {
    

    println!("Welcome, Please Enter your Name:");
  
    let stdin = io::stdin();


    for line in stdin.lock().lines() {

let readline = line.unwrap();
        let lowerline = readline.to_lowercase();

        if (lowerline == "alice") || (lowerline == "bob") {
        
            println!("Have a good day {}", readline);
            
        }

        else {
                println!("I'm feeling grumpy today, No greetings for you! {}", readline);
                
            }
        println!("Try again or Press Ctrl + Z to Exit");
    }
}