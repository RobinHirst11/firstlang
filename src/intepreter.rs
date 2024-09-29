use crate::ast::{Node, Operator};

pub fn eval(node: &Node) -> i32 {
    match node {
        Node::Int(n) => *n,
        Node::UnaryExpr { op, child } => {
            let child = eval(child);
            match op {
                Operator::Plus => child,
                Operator::Minus => -child,
            }
        },
        Node::BinaryExpr { op, lhs, rhs } => {
            let lhs = eval(lhs);
            let rhs = eval(rhs);
            match op {
                Operator::Plus => lhs + rhs,
                Operator::Minus => lhs - rhs,
            }
        }
    }
}
