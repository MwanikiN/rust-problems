/*Sum of 1 to N

Calculate 1 + 2 + ... + N. */
fn main() {
    let mut total = 0;
    for i in 1..=100 {
        total += i;
    }

    println!("{}", total);

    let n = 100;
    let sum = (1..=n).sum::<i32>();
    println!("{}", sum);

    let n = 100;
    let total = (1..=n).fold(0, |acc, x| acc + x);
    println!("{}", total);
}