#[derive(Debug)]
pub enum Node {
    Program(Vec<Node),

    VarDeclaration {
        name: String,
        var_type: String,
        value: Box<Node>,
    },

    Identifier(String),
    StringLiteral(String),
    NumberLiteral(i64),
    Boolean(bool),
    Bugatti,

    Print(Box<Node>),
}