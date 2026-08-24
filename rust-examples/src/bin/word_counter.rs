/*Word Counter
Given a sentence, count the number of words.
 */
fn main() {
    let sentence = "This is a sentence with seven words";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("Number of words: {}", words.len());

}