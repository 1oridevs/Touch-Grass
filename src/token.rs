#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords for the lang
    Print,
    TouchGrass, // variable declaration keyword
    As,
    NumberType,
    Go,
    Outside,
    If,
    Then,
    Instead,
    FrFr, // Block Ending

    // REAL-LIFE OPERATORS
    NoCap,
    Cap,
    Bugatti,

    // Literals for the lang
    String(String),
    Number(i64),

    // Operators for the lang
    GreaterThan,
    LessThan,
    Equals,
    Plus,
    Minus,

    // Identifiers and others
    Identifier(String),
    Comment,
    Illegal(char),
    EOF,
}
