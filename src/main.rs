mod c_parser;

use clap::Parser;
use std::fs;
use std::path::Path;
use crate::c_parser::lexer::Lexer;
use crate::c_parser::parser::Parser as CParser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the C file to parse
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.file);

    // Check if the file format is C (.c extension)
    if path.extension().and_then(|s| s.to_str()) != Some("c") {
        eprintln!("Error: File '{}' is not a C file (.c extension required).", args.file);
        std::process::exit(1);
    }

    // Read the file content
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", args.file, e);
            std::process::exit(1);
        }
    };

    // Run the parser
    println!("Parsing C file: {}...", args.file);
    let lexer = Lexer::new(&content);
    let mut parser = CParser::new(lexer);

    // Basic validation (catch panics for now as the parser is simple)
    let result = std::panic::catch_unwind(move || {
        parser.parse_program()
    });

    match result {
        Ok(ast) => {
            println!("Successfully parsed C file!");
            println!("AST: {:?}", ast);
        }
        Err(_) => {
            eprintln!("Error: Failed to parse C file. It might contain invalid syntax for this simple parser.");
            std::process::exit(1);
        }
    }
}
