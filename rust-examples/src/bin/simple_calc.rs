/*Simple Calculator

Accept two numbers and an operator (+, -, *, /).
 */
fn main(){
    let operators = ['+', '-', '*', '/' ];
    let num1: f64 = 5.0;
    let num2: f64 = 6.0;
    for operator in operators {
        if operator == '+' {
            println!("{} + {} = {}", num1, num2, (num1 + num2));
        }
        else if operator == '-' {
            println!("{} - {} = {}", num1, num2, (num1 - num2));
        }
        else if operator == '*' {
            println!("{} * {} = {}", num1, num2, (num1 * num2));
        }
        else if operator == '/' {
            println!("{} / {} = {}", num1, num2, (num1 / num2));
        }
    }
}