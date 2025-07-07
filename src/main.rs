// src/main.rs

use dryad::cli::DryadCli;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut cli = DryadCli::new();
    
    if let Err(e) = cli.run(args) {
        eprintln!("Erro: {}", e);
        std::process::exit(1);
    }
}