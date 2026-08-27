/*Command Parser
Parse commands such as:
add 10 20
subtract 30 5
multiply 4 5 */
fn parse_command(command: &str) -> Result<i32, String> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.len() != 3 {
        return Err(String::from(
            "Command must have an operation and two numbers",
        ));
    }

    let operation = parts[0];

    let num1: i32 = parts[1]
        .parse()
        .map_err(|_| String::from("Invalid first number"))?;

    let num2: i32 = parts[2]
        .parse()
        .map_err(|_| String::from("Invalid second number"))?;

    match operation {
        "add" => Ok(num1 + num2),
        "subtract" => Ok(num1 - num2),
        "multiply" => Ok(num1 * num2),
        _ => Err(String::from("Unknown command")),
    }
}

fn main() {
    let commands = vec![
        "add 10 20",
        "subtract 30 5",
        "multiply 4 5",
    ];

    for command in commands {
        match parse_command(command) {
            Ok(result) => println!("{} = {}", command, result),
            Err(error) => println!("{} → Error: {}", command, error),
        }
    }
}