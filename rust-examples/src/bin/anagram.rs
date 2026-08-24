/*Anagram Checker
Determine whether two strings are anagrams. */

use std::collections::HashMap;

fn main() 
{
    let s1 = "listen";
    let s2 = "silent";

    let mut map: HashMap<char, i32> = HashMap::new();

    for c in s1.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        *map.entry(c).or_insert(0) -= 1;
    }

    let anagram = map.iter().all(|(_, v)| *v == 0);
    println!("{}", anagram);

    // Alternatively
    let word1 = "listen";
    let word2 = "silent";

    let mut frequency1 = HashMap::new();
    let mut frequency2 = HashMap::new();

    for character in word1.chars() {
        *frequency1.entry(character).or_insert(0) += 1;
    }

    for character in word2.chars() {
        *frequency2.entry(character).or_insert(0) += 1;
    }

    if frequency1 == frequency2 {
        println!("The strings are anagrams");
    } else {
        println!("The strings are not anagrams");
    }
}