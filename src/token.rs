#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords for the lang
    Print,
    TouchGrass, // var for nerds
    As,
    NumberType,
    GoOutside, // If state
    If,
    Then, // Then Keyword
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

    // Identifiers and everything else

    Identifier(String),
    Comment,
    EOF,
}