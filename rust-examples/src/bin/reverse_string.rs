/*Reverse a String
"hello" → "olleh" */

fn main() {
    let s = "hello";
    let reversed: String = s.chars().rev().collect();
    println!("{}", reversed);

    let text = "hello";
    let mut reversed = String::new();

    for character in text.chars().rev() {
        reversed.push(character);
    }

    println!("{}", reversed);
}