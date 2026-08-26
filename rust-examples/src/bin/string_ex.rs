/*27. **Custom String Functions** 

-  Write functions that accept `&str` and return useful information without unnecessarily cloning the string. */

fn count_chararcters (input: &str) -> usize {
    input.chars().count()
}

fn count_words(input: &str) -> usize {
    input.split_whitespace().count()
}

fn is_palindrome(input: &str) -> bool {
    input.chars().eq(input.chars().rev())
}

fn first_character(input: &str) -> char{
    input.chars().next().unwrap()
}

fn count_vowels(input: &str) -> usize {
    input.chars().filter(|&c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')).count()
}

fn count_digits(input: &str) -> usize {
    input.chars().filter(|c| c.is_ascii_digit()).count()
}

fn longest_word(input: &str) -> &str {
    input.split_whitespace().max_by_key(|word| word.len()).unwrap_or("")
}

fn main() {
    let sentence = "hello! world";

    println!("Number of characters: {}", count_chararcters(sentence));
    println!("Number of words: {}", count_words(sentence));
    println!("Is palindrome: {}", is_palindrome(sentence));
    println!("First character: {:?}", first_character(sentence));
    println!("The number of vowels is: {}", count_vowels(sentence));
    println!("The number of digits is: {}", count_digits(sentence));
    println!("Longest word: {}", longest_word(sentence));
}