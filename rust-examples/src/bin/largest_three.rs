/*Largest of Three

Find the largest of three numbers without using a built-in maximum function. */

fn main() {
    let mut numbers = vec![9, 6, 2, 3, 7];
    numbers.sort();
    numbers.reverse();
    println!("{:?}", &numbers[..3]);

    //Alternative approach
    let mut numbers = vec![9, 6, 2, 3, 7];
    numbers.sort_by(|a, b| b.cmp(a));
    println!("{:?}", &numbers[..3]);

    // third approach 
    let numbers = [9, 6, 2, 3, 7];
    println!("{:?}", numbers.iter().max().unwrap());

}