use crate::interpreter::Value;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
pub type BuiltinFunction = fn(Vec<Value>) -> Result<Value, String>;
pub struct Builtins {
    pub functions: HashMap<String, BuiltinFunction>
}
impl Builtins {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        functions.insert("print".to_string(), Builtins::print as BuiltinFunction);
        functions.insert("input".to_string(), Builtins::input as BuiltinFunction);
        functions.insert("str".to_string(), Builtins::str as BuiltinFunction);
        functions.insert("int".to_string(), Builtins::int as BuiltinFunction);
        Builtins { functions }
    }
    fn print(args: Vec<Value>) -> Result<Value, String> {
        for arg in args {
            match arg {
                Value::Integer(i) => print!("{}", i),
                Value::String(s) => print!("{}", s),
                Value::Boolean(b) => print!("{}", b),
                _ => return Err("Unsupported type for print".to_string()),
            }
        }
        println!();
        Ok(Value::Integer(0))
    }
    fn input(args: Vec<Value>) -> Result<Value, String> {
        for arg in args {
            match arg {
                Value::Integer(i) => print!("{}", i),
                Value::String(s) => print!("{}", s),
                Value::Boolean(b) => print!("{}", b),
                _ => return Err("Unsupported type for print".to_string()),
            }
        }
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        Ok(Value::String(s.trim().to_string()))
    }
    fn str(args: Vec<Value>)  -> Result<Value, String> {
        let mut retval = String::new();
        for arg in args {
            match arg {
                Value::Integer(i) => retval.push_str(&format!("{}", i)),
                Value::String(s) => retval.push_str(&format!("{}", s)),
                Value::Boolean(b) => retval.push_str(&format!("{}", b)),
                _ => return Err("Unsupported type for print".to_string()),
            }
        }
        Ok(Value::String(retval))
    }
    fn int(args: Vec<Value>) -> Result<Value, String> {
        let input = Builtins::str(args).expect("Failed to parse int() arguments"); // Use builtin formatter to convert args to str
        if let Value::String(s) = input {
            let retval: i32 = s.parse().expect("Failed to convert int() inputs to signed integer value");
            Ok(Value::Integer(retval))
        } else {
            Err("int() input is not a string.".to_string())
        }
    }
}
