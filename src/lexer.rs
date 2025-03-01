use crate::token::Token;
use std::time::SystemTime;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
    start_time: SystemTime,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            start_time: SystemTime::now(),
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '\0' => Token::EOF,
            '"' => {
                self.read_char();
                let s = self.read_string();
                Token::String(s)
            },
            '>' => { self.read_char(); Token::GreaterThan },
            '<' => { self.read_char(); Token::LessThan },
            '=' => { self.read_char(); Token::Equals },
            '+' => { self.read_char(); Token::Plus },
            '-' => { self.read_char(); Token::Minus },
            _ => {
                if self.ch.is_alphabetic() {
                    let word = self.read_word();
                    return match word.as_str() {
                        "touch" => {
                            self.skip_whitespace();
                            if self.read_word() == "grass" {
                                Token::TouchGrass
                            } else {
                                Token::Identifier("touch".to_string())
                            }
                        },
                        "go" => Token::Go,
                        "outside" => Token::Outside,
                        "if" => Token::If,
                        "then" => Token::Then,
                        "instead" => Token::Instead,
                        "fr" => {
                            self.skip_whitespace();
                            if self.read_word() == "fr" {
                                Token::FrFr
                            } else {
                                Token::Identifier("fr".to_string())
                            }
                        },
                        "print" => Token::Print,
                        "as" => Token::As,
                        "number" => Token::NumberType,
                        "cap" => Token::Cap,
                        "no_cap" => Token::NoCap,
                        "bugatti" => Token::Bugatti,
                        // NEW keywords:
                        "set" => Token::Set,
                        "to" => Token::To,
                        "while" => Token::While,
                        _ => Token::Identifier(word),
                    };
                } else if self.ch.is_numeric() {
                    let num = self.read_number();
                    return Token::Number(num);
                } else {
                    // Return an illegal token for unexpected characters
                    let illegal = Token::Illegal(self.ch);
                    self.read_char();
                    illegal
                }
            }
        };

        tok
    }

    fn read_word(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn read_string(&mut self) -> String {
        let position = self.position;
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        let str_val = self.input[position..self.position].iter().collect();
        if self.ch == '"' {
            self.read_char(); // consume closing quote
        } else {
            // Handle unterminated string literal if needed
            eprintln!("Error: Unterminated string literal");
        }
        str_val
    }

    fn read_number(&mut self) -> i64 {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        let num_str: String = self.input[position..self.position].iter().collect();
        num_str.parse().unwrap_or(0)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}
