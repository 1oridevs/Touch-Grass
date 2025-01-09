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

    // Literals for the lang
    String(String),
    Number(i64),

    // Operators for the lang
    GreaterThan,
    LessThan,
    Equals,

    // Identifiers and everything else

    Identifier(String),
    EOF,
}