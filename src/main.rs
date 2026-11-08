use std::io::{self, Write};

use dbm::convert_expression;

fn main() {
    println!("Denerbone's Math v1.0");
    println!("Type an expression or 'exit' to quit.\n");

    loop {
        print!("dbm> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nGoodbye!");
                break;
            }

            Ok(_) => {
                let input = input.trim();

                if input.eq_ignore_ascii_case("exit") {
                    println!("Goodbye!");
                    break;
                }

                if input.is_empty() {
                    continue;
                }

                match convert_expression(input) {
                    Ok(result) => println!("{result}"),
                    Err(error) => println!("Error: {error}"),
                }
            }

            Err(error) => {
                eprintln!("Input error: {error}");
                break;
            }
        }
    }
}