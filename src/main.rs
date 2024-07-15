use std::io;
use std::io::Write;

#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}

impl Operation {
    fn calculate(&self) -> Result<f64, String>{
        match self {
            Operation::Add(s1, s2) => {
                return Ok(s1 + s2);
            }
            Operation::Subtract(s1, s2) => {
                return Ok(s1 - s2);
            }
            Operation::Multiply(s1, s2) => {
                return Ok(s1 * s2);
            }
            Operation::Divide(s1, s2) => {
                if *s2 == 0.0 {
                    return Err("Dividing by zero causes inconsistent results".to_string());
                }
                return Ok(s1/s2);
            }
        }
    }
}


fn main() {
    let mut input1 = String::new();
    let mut first_number: f64 = 0.0;
    let mut second_number: f64 = 0.0;
    println!("Please enter the first number:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    first_number = match input1.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer");
            return;
        }
    };


    let mut input2 = String::new();
    println!("Please enter the second number:");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    second_number = match input2.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer");
            return;
        }
    };

    let arbitrary_operation_instance = Operation::Divide(first_number, second_number);
    println!("{:?}", arbitrary_operation_instance.calculate().expect("error"));
}
