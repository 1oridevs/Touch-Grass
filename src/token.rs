#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords for the lang
    Print,

    // Literals for the lang
    String(String),
    Number(i64),

    // Identifiers and everything else

    Identifier(String),
    EOF,
}