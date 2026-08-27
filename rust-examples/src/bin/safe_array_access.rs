/*Safe Array Access
Implement a function that returns Option<&T>. */
fn safe_access<T>(arr: &[T], index: usize) -> Option<&T> {
    if index < arr.len() {
        Some(&arr[index])  //can use items.get(index) instead of if statement for safe access
    } else {
        None
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];

    match safe_access(&arr, 2) {
        Some(value) => println!("Value at index 2: {}", value),
        None => println!("Index out of bounds"),
    }

    match safe_access(&arr, 10) {
        Some(value) => println!("Value at index 10: {}", value),
        None => println!("Index out of bounds"),
    }
}