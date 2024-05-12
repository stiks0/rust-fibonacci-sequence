use std::io;
use std::io::Write;
use std::time::Instant;
use num_bigint::BigUint;
use num_traits::{Zero, One};

fn fibonacci(n: usize) -> Vec<BigUint> {
    let mut fib_seq: Vec<BigUint> = vec![Zero::zero(), One::one()];
    for i in 2..=n {
        let next_val = &fib_seq[i-1] + &fib_seq[i-2];
        fib_seq.push(next_val);
    }
    fib_seq
}

fn main() {
    println!("Enter the desired length of the sequence: ");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");

    match input.trim().parse::<usize>() {
        Ok(n) => {
            if n > 500 {
                println!("\x1b[0;31m⚠️ The size of the sequence you entered is very large, so there may be some problem in displaying/processing. Are you sure you want to continue? (y/n) (default 'n') \n -> If the program takes too long, you can always use CTRL + C to aggressively terminate it. \x1b[0m");
                input.clear(); 
                io::stdout().flush().unwrap();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => { 
                        match input.trim() {
                            "n" => { println!("Exiting..."); return; }
                            "y" => {
                                println!("Continuing with the calculation...");
                            }
                            _ => { println!("Exiting..."); return; }
                        }
                    }
                    Err(error) => eprintln!("Error reading input: {}", error),
                }
            }

            let start = Instant::now();
            let fib_seq: Vec<BigUint> = fibonacci(n);
            let duration = start.elapsed();

            println!("\x1b[0;32mCalculation completed in: {:.2} seconds\x1b[0m", duration.as_secs_f64());

            for (i, num) in fib_seq.iter().enumerate() {
                println!("\x1b[0;32m Fibonacci {}: \x1b[0m {}", i, num);
            }

            let total_duration = start.elapsed();
            println!("\x1b[0;32mTotal program duration: {:.2} seconds\x1b[0m", total_duration.as_secs_f64());
        }
        Err(_) => println!("Invalid input. Please enter a valid positive integer."),
    }
}