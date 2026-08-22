// // /*Multiplication Table

// // Print the multiplication table for a number.
// //  */
// // fn main() {
// //     for i in 1..10 {
// //         for j in 1..10 {
// //             println!("{} * {} = {}", i, j, i*j);
// //         }
// //     }
// // }

// fn main() {
//     for i in 1..=10 {
//         for j in 1..=10 {
//             print!("{:>3} x {:>2} = {:>3}   ", i, j, i * j);
//         }
//         println!();
//     }
// }

fn main() {
    for i in 1..=10 {
        for j in 1..=10 {
            print!("{:4}", i * j);
        }
        println!();
    }
}