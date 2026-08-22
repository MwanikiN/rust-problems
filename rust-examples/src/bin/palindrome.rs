/* Palindrome Checker
Determine whether a word is a palindrome. */
fn main() {
    let word: &str = "madam";

    let reversed: String = word.chars().rev().collect();

    if reversed == word {
        println!("The word is a palindrome");
    } else {
        println!("The word is not a palindrome");
    }
}