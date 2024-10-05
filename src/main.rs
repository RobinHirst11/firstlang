#![feature(unboxed_closures)]

pub mod ast;
pub mod parser;
mod interpreter;

use interpreter::Evaluator;

fn main() {
    let source = "
fn main(){
    let x = 0;
    for(let i = 0; (i)<20; i = (i) + 1;){
        x = (x) + 1;
    };
    return x;
}
";
    let ast = parser::parse(source).unwrap_or_else(|e| panic!("{e}"));
    println!("{:?}", ast);
    let mut evaluator = Evaluator::new();
    println!("{:?}", evaluator.run(&ast).unwrap());
}
