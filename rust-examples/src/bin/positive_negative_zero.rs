/*Positive, Negative, or Zero

Given an integer, classify it. */

use std::io;

fn main() {
    println!("Input an integer:");
    let mut input = String::new();
    let _= io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();

    if number > 0 {
        println!("positive");
    } else if number < 0 {
        println!("negative");
    } else {
        println!("zero");
    }

}