use std::vec;

/*Remove Duplicates
Given a vector, return a vector containing only unique values. */
use itertools::Itertools;
fn main() {
    let vector = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
    let mut unique_vec = Vec::<i32>::new();
    
    for number in vector {
        if unique_vec.contains(&number) {
            continue;
        }
        else {
            unique_vec.push(number);

        }}
    print!("{:?} ", unique_vec);


    let vector = vec![9,5,6,9,26,0, 5,6];
    let  mut unique_vec = Vec::new();
    for number in  vector {
        if unique_vec.contains(&number) {
            continue;
        }
        else {
            unique_vec.push(number)
        }
    }
    println!("{:?}", unique_vec);



    let vector = vec![1, 2, 2, 3, 4, 4, 5];

    let unique: Vec<_> = vector.iter().unique().collect();

    println!("{:?}", unique);

    // using hashset
    use std::collections::HashSet;

    let vec= vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
    let set: HashSet<_> = vec.into_iter().collect();
    let unique_vec: Vec<_> = set.into_iter().collect();
    println!("{:?}", unique_vec);

}