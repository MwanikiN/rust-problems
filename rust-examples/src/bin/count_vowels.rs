/*Count Vowels
Count a, e, i, o, and u. */
fn main() {
    let sentence = "This is a sentence";
    let mut vowels = 0;

    for character in sentence.chars() {
        if "aeiou".contains(character) {
            vowels += 1;
        }
    }

    println!("Number of vowels: {}", vowels);
}