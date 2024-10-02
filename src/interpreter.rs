use std::collections::HashMap;
use std::fmt::{Formatter, write};
use crate::ast::AstNode;

pub enum ProgramTypes {
    Str(String),
    Int(i32),
    Bool(bool),
}

impl std::fmt::Display for ProgramTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramTypes::Str(s) => write!(f, "{s}"),
            ProgramTypes::Int(i) => write!(f, "{i}"),
            ProgramTypes::Bool(b) => write!(f, "{b}")
        }
    }
}

pub enum SymbolEntry {
    Value(ProgramTypes),
    StringBuiltin(Box<dyn Fn<String, Output = Option<String>>>),
    Function{
        body: Box<AstNode>,
        scope: HashMap<String, SymbolEntry>,
    },
}

pub fn eval(node: &AstNode, symbol_table: &mut HashMap<String, SymbolEntry>) -> ProgramTypes {
    match node {
        AstNode::Program(p) => {
            for funcdef in p {
                match funcdef.as_ref() {
                    AstNode::FuncDef { name, .. } => {
                        symbol_table.insert(name.clone(), SymbolEntry::Function {
                            body: Box::from(funcdef.clone()),
                            scope: HashMap::new(),
                        });
                    },
                    unknown => panic!("Unknown function def {:?}", unknown)
                }
            }

            ProgramTypes::Int(44)
        },
        AstNode::Boolean(b) => ProgramTypes::Bool(*b),
        unknown => panic!("Cannot evaluate {:?}", unknown)
    }
}
