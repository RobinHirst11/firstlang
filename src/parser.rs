use std::result::Result;

use pest::iterators::Pair;
use pest::Parser;

use crate::ast::{AstNode, BinaryOperator, UnaryOperator};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;

pub fn parse(source: &str) -> Result<AstNode, pest::error::Error<Rule>> {
    let pair = CalcParser::parse(Rule::Program, source)?.next().unwrap();
    let ast = build_ast_from_root(pair);

    Ok(ast)
}

fn build_ast_from_root(pair: Pair<Rule>) -> AstNode {
    let mut func_defs: Vec<Box<AstNode>> = vec![];
    let rule_pairs = pair.into_inner();
    for rule_pair in rule_pairs {
        match rule_pair.as_rule() {
            Rule::FuncDef => func_defs.push(Box::new(parse_func_def(rule_pair))),
            Rule::EOI => (),
            unknown => panic!("Unknown root: {:?}", unknown),
        };
    }

    AstNode::Program(func_defs)
}

fn build_ast_from_block(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::Block => {
            let mut statement_vec: Vec<Box<AstNode>> = vec![];
            let statements = pair.into_inner();
            for statement in statements {
                statement_vec.push(Box::new(build_ast_from_statement(statement)));
            }

            AstNode::Block(statement_vec)
        }
        unknown => panic!("Unknown block: {:?}", unknown),
    }
}

fn build_ast_from_statement(pair: Pair<Rule>) -> AstNode {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::VarDecl => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap().as_str();
            let value = pair.next();
            let value = match value {
                Some(pair) => Some(Box::new(build_ast_from_expression(pair))),
                None => None,
            };
            AstNode::VarDecl {
                name: name.to_string(),
                value,
            }
        }
        Rule::VarSet => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap().as_str();
            let value = pair.next();
            let value = match value {
                Some(pair) => Some(Box::new(build_ast_from_expression(pair))),
                None => None,
            };
            AstNode::VarSet {
                name: name.to_string(),
                value: value.unwrap(),
            }
        }
        Rule::FuncCall => build_ast_from_function_call(pair),
        Rule::FuncReturn => AstNode::FuncReturn(Box::new(build_ast_from_expression(
            pair.into_inner().next().unwrap(),
        ))),
        Rule::ForLoop => {
            let mut pair = pair.into_inner();
            let for_params = parse_for_params(pair.next().unwrap());
            let block = build_ast_from_block(pair.next().unwrap());
            AstNode::ForLoop {
                params: Box::new(for_params),
                body: Box::new(block),
            }
        }
        Rule::WhileLoop => {
            let mut pair = pair.into_inner();
            let condition = build_ast_from_expression(pair.next().unwrap());
            let block = build_ast_from_block(pair.next().unwrap());
            AstNode::WhileLoop {
                condition: Box::new(condition),
                body: Box::new(block),
            }
        }
        Rule::IfStatement => {
            let mut pair = pair.into_inner();
            let condition = build_ast_from_expression(pair.next().unwrap());
            let block = build_ast_from_block(pair.next().unwrap());
            AstNode::IfStatement {
                condition: Box::new(condition),
                body: Box::new(block),
            }
        }
        unknown => panic!("Unknown statement: {:?}", unknown),
    }
}

fn build_ast_from_expression(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::Expression => AstNode::Expression(Box::new(build_ast_from_expression(
            pair.into_inner().next().unwrap(),
        ))),
        Rule::Number => parse_number(pair),
        Rule::UnaryExpression => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let child = pair.next().unwrap();
            let child = build_ast_from_term(child);

            parse_unary_expression(op, child)
        }
        Rule::BinaryExpression => {
            let mut pair = pair.into_inner();
            let lhs = pair.next().unwrap();
            let mut lhs = build_ast_from_term(lhs);
            let op = pair.next().unwrap();
            let rhs = pair.next().unwrap();
            let mut rhs = build_ast_from_term(rhs);

            let mut retval = parse_binary_expression(lhs, op, rhs);
            loop {
                let pair_buf = pair.next();
                if let Some(op) = pair_buf {
                    lhs = retval;
                    rhs = build_ast_from_term(pair.next().unwrap());
                    retval = parse_binary_expression(lhs, op, rhs);
                } else {
                    return retval;
                }
            }
        }
        Rule::FuncCall => build_ast_from_function_call(pair),
        Rule::Identifier => AstNode::Identifier(pair.as_str().to_string()),
        Rule::String => {
            let string_lit = pair.as_str();
            AstNode::Str(string_lit[1..string_lit.len() - 1].to_string())
        }
        Rule::Boolean => match pair.as_str() {
            "True" => AstNode::Boolean(true),
            "False" => AstNode::Boolean(false),
            unknown => panic!("Unknown boolean: {:?}", unknown),
        },
        unknown => panic!("Unknown expression: {:?}", unknown),
    }
}

fn build_ast_from_term(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::Term => AstNode::Term(Box::new(build_ast_from_term(
            pair.into_inner().next().unwrap(),
        ))),
        Rule::Number => parse_number(pair),
        Rule::Expression => build_ast_from_expression(pair),
	Rule::FuncCall => build_ast_from_function_call(pair),
        Rule::Identifier => AstNode::Str(pair.as_str().to_string()),
        unknown => panic!("Unknown term: {:?}", unknown),
    }
}

fn build_ast_from_function_call(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::FuncCall => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap().as_str();
            let arg_list = build_ast_from_arg_list(pair.next().unwrap());
            AstNode::FuncCall {
                name: name.to_string(),
                args: Box::new(arg_list),
            }
        }
        unknown => panic!("Unknown function call: {:?}", unknown),
    }
}

fn build_ast_from_arg_list(pair: Pair<Rule>) -> AstNode {
    let mut args: Vec<Box<AstNode>> = vec![];
    let arg_pairs = pair.into_inner();

    for arg in arg_pairs {
        args.push(Box::new(build_ast_from_expression(arg)));
    }
    AstNode::ArgList(args)
}

fn parse_func_def(pair: Pair<Rule>) -> AstNode {
    let mut pair = pair.into_inner();
    let ident = pair.next().unwrap().as_str();
    let args = pair.next().unwrap();
    let body = pair.next().unwrap();
    AstNode::FuncDef {
        name: ident.to_string(),
        args: Box::new(parse_def_arg_list(args)),
        body: Box::new(build_ast_from_block(body)),
    }
}

fn parse_def_arg_list(pair: Pair<Rule>) -> AstNode {
    let mut args = vec![];
    let arg_pairs = pair.into_inner();

    for arg in arg_pairs {
        args.push(arg.as_str().to_string());
    }
    AstNode::DefArgList(args)
}

fn parse_number(pair: Pair<Rule>) -> AstNode {
    let value: i32 = pair.as_str().parse().unwrap();
    AstNode::Int(value)
}

fn parse_for_params(pair: Pair<Rule>) -> AstNode {
    let mut pair = pair.into_inner();
    AstNode::ForLoopParams {
        initialization: Box::new(build_ast_from_statement(pair.next().unwrap())),
        condition: Box::new(build_ast_from_expression(pair.next().unwrap())),
        updater: Box::new(build_ast_from_statement(pair.next().unwrap())),
    }
}

fn parse_unary_expression(pair: Pair<Rule>, child: AstNode) -> AstNode {
    AstNode::UnaryExpression {
        op: parse_unary_operator(pair),
        child: Box::new(child),
    }
}

fn parse_unary_operator(pair: Pair<Rule>) -> UnaryOperator {
    match pair.as_str() {
        "!" => UnaryOperator::Not,
        "-" => UnaryOperator::Minus,
        unknown => panic!("Unknown rule: {:?}", unknown),
    }
}

fn parse_binary_expression(lhs: AstNode, op: Pair<Rule>, rhs: AstNode) -> AstNode {
    AstNode::BinaryExpression {
        lhs: Box::new(lhs),
        op: parse_binary_operator(op),
        rhs: Box::new(rhs),
    }
}

fn parse_binary_operator(pair: Pair<Rule>) -> BinaryOperator {
    match pair.as_str() {
        "+" => BinaryOperator::Add,
        "-" => BinaryOperator::Subtract,
        "*" => BinaryOperator::Multiply,
        "/" => BinaryOperator::Divide,
        ">" => BinaryOperator::Greater,
        "<" => BinaryOperator::Less,
        "==" => BinaryOperator::Equal,
        "!=" => BinaryOperator::NotEqual,
        ">=" => BinaryOperator::GreaterEq,
        "<=" => BinaryOperator::LessEq,
        unknown => panic!("Unknown rule: {:?}", unknown),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn e2e_functions() {}
}
