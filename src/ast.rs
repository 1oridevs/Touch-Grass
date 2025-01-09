#[derive(Debug)]
pub enum Node {
    // Program root
    Program(Vec<Node>),

    // Statements
    VarDeclaration {
        name: String,
        var_type: String,
        value: Box<Node>,
    },

    // Expressions
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(i64),
    Boolean(bool),  // cap/no_cap
    Bugatti,        // our null/undefined

    // Print statement
    Print(Box<Node>),

    // Conditional statements
    GoOutside {
        condition: Box<Node>,
        then_branch: Box<Node>,
    },
    Block(Vec<Node>),
    
    // Binary operations (for comparisons)
    BinaryOp {
        left: Box<Node>,
        operator: Operator,
        right: Box<Node>,
    },
}

#[derive(Debug)]
pub enum Operator {
    GreaterThan,
    LessThan,
    Equals,
}