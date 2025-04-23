mod ast;
mod parser;
mod codegen;

use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Read error");
    
    let ast = parser::parse(&input);
    let ir = codegen::generate(ast);
    fs::write("output.ll", ir).expect("Write error");
    
    Command::new("llc")
        .arg("output.ll")
        .status()
        .expect("LLVM failed");
    
    // Add error chain handling
    if let Err(e) = process_file(&args[1]) {
        eprintln!("Compilation failed: {}", e);
        std::process::exit(1);
    }
}

fn process_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Read error");
    
    let ast = parser::parse(&input);
    let ir = codegen::generate(ast);
    fs::write("output.ll", ir).expect("Write error");
    
    Command::new("llc")
        .arg("output.ll")
        .status()
        .expect("LLVM failed");
    Ok(())
}