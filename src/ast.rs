#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,       // +
    Subtract,  // -
    Multiply,  // *
    Divide,    // /
    Greater,   // >
    Less,      // <
    Equal,     // ==
    NotEqual,  // !=
    GreaterEq, // >=
    LessEq,    // <=
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Minus, // -
    Not,   // !
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Program(Vec<Box<AstNode>>),
    FuncDef {
        name: String,
        args: Box<AstNode>,
        body: Box<AstNode>,
    },
    Block(Vec<Box<AstNode>>),
    VarDecl {
        name: String,
        value: Option<Box<AstNode>>,
    },
    VarSet {
        name: String,
        value: Box<AstNode>,
    },
    FuncCall {
        name: String,
        args: Box<AstNode>,
    },
    FuncReturn(Box<AstNode>),
    ForLoop {
        params: Box<AstNode>,
        body: Box<AstNode>,
    },
    ForLoopParams {
        initialization: Box<AstNode>,
        condition: Box<AstNode>,
        updater: Box<AstNode>,
    },
    WhileLoop {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    IfStatement {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    DefArgList(Vec<String>),
    ArgList(Vec<Box<AstNode>>),
    Expression(Box<AstNode>),
    BinaryExpression {
        lhs: Box<AstNode>,
        op: BinaryOperator,
        rhs: Box<AstNode>,
    },
    UnaryExpression {
        op: UnaryOperator,
        child: Box<AstNode>,
    },
    Term(Box<AstNode>),
    Int(i32),
    Str(String),
    Identifier(String),
    Boolean(bool),
}
