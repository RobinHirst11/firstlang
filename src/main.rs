#![feature(unboxed_closures)]

pub mod ast;
pub mod parser;
pub mod interpreter;
pub mod builtins;


use std::fs;

use interpreter::Evaluator;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file was provided");
        std::process::exit(-1);
    }

    let source = fs::read_to_string(args[1].clone()).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(-1);
    });


    let ast = parser::parse(&source).unwrap_or_else(|e| panic!("{e}"));
    let mut evaluator = Evaluator::new();
    evaluator.run(&ast).unwrap();
}
