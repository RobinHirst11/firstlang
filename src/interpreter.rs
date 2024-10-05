
// Temporary to remove all warnings
#[allow(dead_code)]

use core::panic;
use std::collections::HashMap;

use crate::ast::AstNode;
use crate::ast::BinaryOperator; 
use crate::ast::UnaryOperator;

#[derive(Debug, Clone)] pub enum Value { Integer(i32),
    String(String),
    Boolean(bool),
    Function(String, Vec<String>, Box<AstNode>),
}

#[derive(Clone)]
struct SymbolTable {
    symbols: HashMap<String, Value>,
    parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    fn with_parent(parent: Box<SymbolTable>) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: Some(parent),
        }
    }

    fn get(&self, name: &str) -> Option<Value> {
        match self.symbols.get(name) {
            Some(value) => Some(value.clone()),
            None => match &self.parent {
                Some(parent) => parent.get(name),
                None => None,
            },
        }
    }

    fn set(&mut self, name: String, value: Value) {
        self.symbols.insert(name, value);
    }
}

pub struct Evaluator {
    symbol_table: SymbolTable, 
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn eval(&mut self, node: &AstNode) -> Result<Value, String> {
        match node {
            AstNode::Program(funcdefs) => {
                let mut result = Value::Integer(0);

                for funcdef in funcdefs {
                    result = self.eval(funcdef)?;
                }
                Ok(result)

            },
            AstNode::FuncDef { name, args, body } => {
                if let AstNode::DefArgList(arg_names) = &**args {
                    let func = Value::Function(name.clone(), arg_names.clone(), body.clone());
                    self.symbol_table.set(name.clone(), func);

                    Ok(Value::Integer(0))
                } else {
                    Err("Invalid function definition".to_string())
                }
            }
            AstNode::Expression(expr) => self.eval(expr),
            AstNode::BinaryExpression { lhs, op, rhs } => {
                let left = self.eval(lhs)?;
                let right = self.eval(rhs)?;
                match (left, right) {
                    (Value::Integer(l), Value::Integer(r)) => match op {
                        BinaryOperator::Add => Ok(Value::Integer(l + r)),
                        BinaryOperator::Subtract => Ok(Value::Integer(l - r)),
                        BinaryOperator::Multiply => Ok(Value::Integer(l * r)),
                        BinaryOperator::Divide => Ok(Value::Integer(l / r)),
                        BinaryOperator::Greater => Ok(Value::Boolean(l > r)),
                        BinaryOperator::Less => Ok(Value::Boolean(l < r)),
                        BinaryOperator::Equal => Ok(Value::Boolean(l == r)),
                        BinaryOperator::NotEqual => Ok(Value::Boolean(l != r)),
                        BinaryOperator::GreaterEq => Ok(Value::Boolean(l >= r)),
                        BinaryOperator::LessEq => Ok(Value::Boolean(l <= r)),
                    },
                    _ => Err("Invalid operands for binary expression".to_string()),

                }
            },
            AstNode::UnaryExpression { op, child } => {
                let value = self.eval(child)?;
                match (op, value) {
                    (UnaryOperator::Minus, Value::Integer(i)) => Ok(Value::Integer(-i)),
                    (UnaryOperator::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
                    _ => Err("Invalid operand for unary expression".to_string()),
                }
            }
            AstNode::Term(term) => self.eval(term),
            AstNode::Int(i) => Ok(Value::Integer(*i)),
            AstNode::Boolean(b) => Ok(Value::Boolean(*b)),
            AstNode::Str(s) => Ok(Value::String(s.clone())),
            AstNode::Identifier(name) => self.symbol_table.get(name).ok_or_else(|| format!("Undefined variable {}", name)),
            AstNode::Block(statements) => {
                let mut result = Value::Integer(0);

                for stmt in statements {
                    // TODO: Add return value value, and break out with that incase it's that
                    // Or make result into the statement itself?
                    // Not sure, gotta figure this stuff out
                    result = self.eval(stmt)?;
                }

                Ok(result)
            },
            AstNode::FuncReturn(expr) => self.eval(expr),
            AstNode::VarDecl { name, value } => {
                let val = match value {
                    Some(expr) => self.eval(expr)?,
                    None => Value::Integer(0),
                };

                self.symbol_table.set(name.clone(), val);
                Ok(Value::Integer(0))
            },
            AstNode::VarSet { name, value } => {
                let value = self.eval(value)?;
                self.symbol_table.set(name.clone(), value);
                Ok(Value::Integer(0))
            },
            AstNode::FuncCall { name, args } => {
                if let Some(Value::Function(_, params, body)) = self.symbol_table.get(name) {
                    match &**args {
                        AstNode::ArgList(arg_values) => {
                            self.symbol_table = SymbolTable::with_parent(Box::new(self.symbol_table.clone()));

                            for (param, arg) in params.iter().zip(arg_values.iter()) {
                                let arg_value = self.eval(arg)?;
                                self.symbol_table.set(param.clone(), arg_value);
                            }

                            let result = self.eval(&body)?;
                            self.symbol_table = *self.symbol_table.parent.as_mut().unwrap().clone();

                            Ok(result)
                        },
                        unknown => panic!("Can only have ArgList as params, you had {:?}", unknown)
                    }
                } else {
                    Err(format!("Function '{}' not found", name))
                }
            },
            AstNode::IfStatement { condition, body } => {
                if let Value::Boolean(true) = self.eval(condition)? {
                    self.eval(body)
                } else {
                    Ok(Value::Integer(0))
                }
            },
            AstNode::WhileLoop { condition, body } => {
                while let Value::Boolean(true) = self.eval(condition)? {
                    self.eval(body)?;
                } 
                Ok(Value::Integer(0))
            },
            AstNode::ForLoop { params, body } => {
                if let AstNode::ForLoopParams { initialization, condition, updater } = &**params {
                    self.eval(initialization)?;
                    while let Value::Boolean(true) = self.eval(condition)? {
                        self.eval(body)?;
                        self.eval(updater)?;
                    }

                    Ok(Value::Integer(0))
                } else {
                    Err("Invalid for loop parameters".to_string())
                }
            },
            unknown => panic!("Unimplemented Node {unknown:?}")
        }
    }

    pub fn run(&mut self, ast: &AstNode) -> Result<Value, String> {
        let _ = self.eval(ast);

        match self.symbol_table.get("main") {
            Some(Value::Function(_, params, body)) => {
                if !params.is_empty() {
                    return Err("main() function should not have parameters".to_string());
                }

                self.symbol_table = SymbolTable::with_parent(Box::new(self.symbol_table.clone()));
                let result = self.eval(&body);
                self.symbol_table = *self.symbol_table.parent.as_mut().unwrap().clone();

                result
            },
            Some(_) => {
                panic!("main() is not a function.. how the fuck did you mess that up?")
            },
            None => Err("No main() function defined".to_string()),
        }

    }
}
