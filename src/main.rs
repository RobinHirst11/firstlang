use intepreter::eval;

pub mod ast;
pub mod parser;
pub mod intepreter;

fn main() {
    let ast = parser::parse("1+2+34+3-13+41").unwrap_or_else(|e| { panic!("{e}") });
    println!("{}", eval(&ast[0]));
}
