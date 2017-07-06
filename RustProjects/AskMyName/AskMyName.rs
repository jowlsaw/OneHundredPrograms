
//used for standard IO
use std::io;

// official documentation(https://doc.rust-lang.org/std/io/prelude/index.html) is just too nerdy for me, but it seems to allow both the lock() and lines() functions to be used in the same line of code :) 
use std::io::prelude::*;


// This is the main function
fn main() {
    

    // Print text to the console
    println!("Welcome, Please Enter your Name:");
    
    //looks like an object reference for io:stdin();
    let stdin = io::stdin();


    // lock() is used to wait for user input
    //lines() is used to read the input buffer as a string
    for line in stdin.lock().lines() {
        println!("Have a good day {}", line.unwrap());
        println!("Press Ctrl + Z to Exit");
    }
}

