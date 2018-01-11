use std::io;
use std::str;
use std::u32;

fn main() {
    

    println!("Welcome, Please Enter the Limit:");
  
        let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let input: u32 = line
        .trim()
        .parse()
        .expect("Wanted a number");

       let limit = input;
        let mut x = 1;
        let mut y = 2;
        let mut z = 0; 
      //  let  count = 0;
        
        println!("the Lucas Series for the limit {} is: ", y);    
        print!("{}\t{}\t", x, y);    
for mut count in  0..limit
      {
          count = count + 1;
          z = x + y;        
          print!("{}\t", z);
          x = y;
          y = z;
      }
    
    
}