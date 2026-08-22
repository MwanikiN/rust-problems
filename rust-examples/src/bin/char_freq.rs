/*Character Frequency
Given a string, count how many times each character occurs. */

use std::collections::HashMap;

fn main() {
    let sentence = "hello world";
    let mut frequency = HashMap::new();

    for character in sentence.chars() {
        *frequency.entry(character).or_insert(0) += 1;
    }

    println!("{:?}", frequency);
}