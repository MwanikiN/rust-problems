/*Read an integer and determine whether it is even or odd. */
use std::io;

fn main () 
{
    println!("Enter an integer:");
    let mut input = String::new();
    let _= io::stdin().read_line(&mut input);
    let number: i32 = input.trim().parse().unwrap();
    if number % 2 == 0 {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }
}