use std::io;
use std::str;
use std::u32;

fn main() {
    

    println!("Welcome, Please Enter the value of n:");
  
        let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let input: u32 = line
        .trim()
        .parse()
        .expect("Wanted a number");

       let nn = input+1;


        let y = (input*nn)/2;
        
        println!("the Triangular Number is : {}", y);    
    
    
}