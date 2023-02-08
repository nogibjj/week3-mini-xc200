use std::io;

fn main() {
    println!("Welcome to the calculator!");
    loop {
        println!("Please input an expression:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            println!("Goodbye!");
            break;
        }
        let result = match input.parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                let parts: Vec<&str> = input.split(" ").collect();
                if parts.len() != 3 {
                    println!("Invalid expression: {}", input);
                    continue;
                }
                let a = parts[0].parse::<f64>().unwrap();
                let b = parts[2].parse::<f64>().unwrap();
                match parts[1] {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => {
                        println!("Invalid operator: {}", parts[1]);
                        continue;
                    }
                }
            }
        };
        println!("Result: {}", result);
    }
}
