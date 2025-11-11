//! A command-line tool for parsing and reformatting structured text.
//!
//! This is a Rust implementation inspired by the shuffle script from RISE-Maritime/porla.
//!
//! # Usage
//!
//! ```bash
//! echo "2024-01-15 INFO Hello" | shuffle "{date} {level} {message}" "{level}: {message}"
//! # Output: INFO: Hello
//! ```

use gullwing::{Formatter, Parser};
use std::env;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} INPUT_FORMAT OUTPUT_FORMAT", args[0]);
        eprintln!();
        eprintln!("Parse stdin using INPUT_FORMAT and output using OUTPUT_FORMAT");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  echo '2024-01-15 INFO Hello' | {} '{{date}} {{level}} {{message}}' '{{level}}: {{message}}'", args[0]);
        eprintln!("  Output: INFO: Hello");
        process::exit(1);
    }

    let input_format = &args[1];
    let output_format = &args[2];

    // Create parser and formatter
    let parser = match Parser::new(input_format) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing input format: {}", e);
            process::exit(1);
        }
    };

    let formatter = match Formatter::new(output_format) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error parsing output format: {}", e);
            process::exit(1);
        }
    };

    // Process stdin line by line
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        };

        // Try to parse the line
        match parser.parse(&line) {
            Ok(Some(result)) => {
                // Format the result
                match formatter.format_map(result.values()) {
                    Ok(output) => println!("{}", output),
                    Err(e) => {
                        eprintln!("Error formatting line '{}': {}", line, e);
                    }
                }
            }
            Ok(None) => {
                // No match - skip this line silently or print it unchanged
                // (Python version skips, we'll do the same)
            }
            Err(e) => {
                eprintln!("Error parsing line '{}': {}", line, e);
            }
        }
    }
}
