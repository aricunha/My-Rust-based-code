use std::io;

fn main() {
    println!("Yo, welcome to the Rust basic calculator, let's get this math done!");

    // Seek the first number
    let num1 = read_input("Ayo, drop the first number, fam!: ")
        .trim()
        .parse::<f64>()
        .expect("C'mon, drop a valid number!");

    // Request the user to select the operator
    let operator = read_input("Alright, choose your operation (+ for add, - for subtract, * for multiply, / for divide): ").trim().to_string();

    // Get the second number
    let num2 = read_input("Go ahead, enter the second number: ")
        .trim()
        .parse::<f64>()
        .expect("Please, provide a valid number!");

    // Carry out the math
    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Oops! You can't divide by zero, math don't work like that!");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Whoops, that's not a valid operation! Pick +, -, , or /, alright?");
            return;
        }
    };

    // Give the results
    println!("The result of {} {} {} is: {}", num1, operator, num2, result);
}

// Function to get user input
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada!");
    input
}
