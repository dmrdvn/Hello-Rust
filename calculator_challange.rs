use std::io;

enum Operation { //Defined Operations with enum
    Add(f64,f64),
    Subtract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64),
}

fn calculate(operation:Operation) -> f64 {
    
    match operation { //Using Pattern Matching for the enum
        Operation::Add(num1, num2) => num1 + num2,
        Operation::Subtract(num1, num2) => num1 - num2,
        Operation::Multiply(num1, num2) => num1 * num2,
        Operation::Divide(num1, num2) => {
            if num2 != 0.0 {
                num1/num2
            }else {
                println!("The second number cannot be 0!");
                std::f64::NAN // Return NaN for division by zero
            }
        }
    }
    
}

fn main() {

    // Operator Selection
    println!("Hello! Welcome to My Simple Calculator.");

    // Getting first number
    println!("Please enter the first number.");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Error Reading Input!");


    // Selecting Operator
    println!("Choose Operator: + , - , * , /");
    let mut op_input = String::new();
    io::stdin().read_line(&mut op_input).expect("Error Reading Input!");

    
    // Getting second number
    println!("Please enter the second number.");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Error Reading Input!");
    
    
    // String to Int Parsing
    let num1: f64 = num1.trim().parse().expect("Invalid input");
    let num2: f64 = num2.trim().parse().expect("Invalid input");
    

    
    
    //Adding input data to op variable
    let op = match op_input.trim() { //clean the space with trim in input data 
        "+" => Operation::Add(num1,num2),
        "-" => Operation::Subtract(num1,num2),
        "*" => Operation::Multiply(num1,num2),
        "/" => Operation::Divide(num1,num2),
        _ => {
            println!("Invalid Operator Selection!");
            return;
        }
        
    };
 
    
    let result = calculate(op); //Call function with op parameter
    println!("Result: {}", result); //Printing the result variable
    
}
