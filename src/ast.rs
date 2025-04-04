#[derive(Debug, Clone)]
pub enum Node {
    Program(Vec<Node>),

    VarDeclaration {
        name: String,
        var_type: String,
        value: Box<Node>,
    },

    // NEW: Variable Assignment
    Assignment {
        name: String,
        value: Box<Node>,
    },

    Identifier(String),
    StringLiteral(String),
    NumberLiteral(i64),
    Boolean(bool),
    Bugatti,

    Print(Box<Node>),

    GoOutside {
        condition: Box<Node>,
        then_branch: Box<Node>,
        else_branch: Option<Box<Node>>,
    },
    Block(Vec<Node>),

    // NEW: While Loop
    WhileLoop {
        condition: Box<Node>,
        body: Box<Node>,
    },
    
    BinaryOp {
        left: Box<Node>,
        operator: Operator,
        right: Box<Node>,
    },
}

#[derive(Debug, Clone)]
pub enum Operator {
    GreaterThan,
    LessThan,
    Equals,
    Plus,
    Minus,
}
