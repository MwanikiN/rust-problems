/*Find the Largest Element
Find the largest value in a Vec<i32>. */

fn main() {
    let numbers = vec![1, 5, 3, 9, 2, 8, 4, 7, 6];
    let mut largest = numbers[0];
    for i in 1..numbers.len() {
        if numbers[i] > largest {
            largest = numbers[i];
        }
    }
    println!("The largest number is {}", largest);

    // alternatively using fold
    let largest = numbers.iter().fold(numbers[0], |acc, &x| if x > acc { x } else { acc });
    println!("The largest number is {}", largest);

    // alternatively using max
    let largest = numbers.iter().max().unwrap();
    println!("The largest number is {}", largest);

    let vector = vec![9, 5, 6, 10, 26, 0];

    let largest = vector.iter().fold(vector[0], |largest, &i| largest.max(i));

    println!("{}", largest);
}
