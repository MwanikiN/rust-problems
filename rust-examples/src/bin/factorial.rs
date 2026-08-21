fn main(){
    let mut n = 3;
    let mut factorial = 1;
    loop {
        factorial *= n;
        n -= 1;
        if n <= 1 {
            break;
        }
    }

    println!("The factorial is {}", factorial);

    let n = 5;
    let factorial = (1..=n).fold(1, |acc, x| acc * x );
    println!("The factorial of {} is {}", n, factorial);


    let n = 5;

    let factorial: i32 = (1..=n).product();

    println!("{}! = {}", n, factorial);
    }