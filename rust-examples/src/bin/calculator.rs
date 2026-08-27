/*Custom Result Calculator
Division should return an error when dividing by zero.
 */
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(a / b)
    }


}

fn main() {
    let a = 10.0;
    let b = 2.0;

    match divide(a, b) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("{}", e),
    }
}