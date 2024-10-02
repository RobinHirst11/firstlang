#![feature(unboxed_closures)]

use std::collections::HashMap;
use std::rc::Rc;
use crate::interpreter::SymbolEntry;

pub mod ast;
pub mod parser;
mod interpreter;

fn main() {
    let source = "
fn main() {
    println(\"Hello\");
}
";
    let ast = parser::parse(source).unwrap_or_else(|e| panic!("{e}"));

    let mut symbol_table: HashMap<String, SymbolEntry> = HashMap::new();

    println!("{}", interpreter::eval(&ast, &mut symbol_table));
}
