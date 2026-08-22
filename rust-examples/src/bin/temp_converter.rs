/*Temperature Converter

Convert Celsius ↔ Fahrenheit. */
 use std::io::stdin;

 fn main() {
     println!("Enter temperature in Celsius: ");
     let mut input = String::new();
     let _= stdin().read_line(&mut input).expect("failed to read input");
     let celsius: f64 = input.trim().parse().unwrap();
     let fahrenheit = (celsius * 9.0/5.0) + 32.0;
     println!("{}°C is equal to {}°F", celsius, fahrenheit);
 }
// use std::io;

// fn main() {
//     let mut input = String::new();

//     println!("Enter temperature in Celsius:");

//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read input");

//     let celsius: f64 = input
//         .trim()
//         .parse()
//         .expect("Please enter a valid number");

//     let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;

//     println!("{}°C = {}°F", celsius, fahrenheit);
// }