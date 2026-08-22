/*FizzBuzz
Print numbers from 1–100:
multiples of 3 → Fizz
multiples of 5 → Buzz
multiples of both → FizzBuzz */
fn main() {
    let n: i32 = 100;
    for i in 1..=n {
        if i%3 == 0 && i%5 == 0{
            println!("FizzBuzz");
        }
        else if i%3 == 0 {
            println!("Fizz");
        }
        else if i%5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", i);
        }
    }

    }