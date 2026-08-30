/* Binary Search
Implement binary search from scratch.*/


fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = (low + high) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }

    None
}

fn main() {
    let arr = [3, 1, 0, 9, 7, 4, 8, 2, 5, 6];
    let target = 5;
    match binary_search(&arr, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found", target),
    }
}