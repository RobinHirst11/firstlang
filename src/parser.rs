e core::panic;
use std::result::Result;

use pest::iterators::Pair;
use pest::Parser;

use crate::ast::Node;
use crate::ast::Operator;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;

pub fn parse(source: &str) -> Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = CalcParser::parse(Rule::Program, source)?;

    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }

    Ok(ast)
}

fn build_ast_from_expr(pair: Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::UnaryExpr => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let child = build_ast_from_term(pair.next().unwrap());

            parse_unary_expr(op, child)
        }
        Rule::BinaryExpr => {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let op = pair.next().unwrap();
            let rhspair = pair.next().unwrap();

            let mut lhs = build_ast_from_term(lhspair);
            let mut rhs = build_ast_from_term(rhspair);

            let mut retval = parse_binary_expr(op, lhs, rhs);

            loop {
                match pair.next() {
                    Some(op) => {
                        lhs = retval;
                        rhs = build_ast_from_term(pair.next().unwrap());
                        retval = parse_binary_expr(op, lhs, rhs)
                    }
                    None => return retval
                }
            }
        }
        unknown => panic!("Unknown term: {unknown:?}"),
    }
}

fn parse_binary_expr(pair: Pair<Rule>, lhs: Node, rhs: Node) -> Node {
    Node::BinaryExpr {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => panic!(),
        },
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

fn parse_unary_expr(pair: Pair<Rule>, child: Node) -> Node {
    Node::UnaryExpr {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => panic!(),
        },
        child: Box::new(child),
    }
}

fn build_ast_from_term(pair: Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Int => {
            // Match on first char
            // If -, multiply -1, and return Node::Int(-istr)
            // Else Node::Int(istr)
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1] {
                "-" => (-1, &istr[..1]),
                _ => (1, istr),
            };

            let int: i32 = istr.parse().unwrap();
            Node::Int(sign * int)
        }
        Rule::Expr => build_ast_from_expr(pair),
        unknown => panic!("Unknown term: {unknown:?}"),
    }
}
