use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
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
                Token::String(self.read_string())
            },
            _ => {
                if self.ch.is_alphabetic() {
                    let word = self.read_identifier();
                    match word.as_str() {
                        "print" => Token::Print,
                        _ => Token::Identifier(word),
                    }
                } else if self.ch.is_numeric() {
                    Token::Number(self.read_number())
                } else {
                    Token::EOF
                }
            }
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn read_string(&mut self) -> String {
        let position = self.position;
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
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