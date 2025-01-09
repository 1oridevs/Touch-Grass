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

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '\0' => Token::EOF,
            '"' => {
                self.read_char();
                Token::String(self.read_string())
            },
            '>' => Token::GreaterThan,
            '<' => Token::LessThan,
            '=' => Token::Equals,
            _ => {
                if self.ch.is_alphabetic() {
                    let word = self.read_word();
                    match word.as_str() {
                        "touch" => {
                            self.skip_whitespace();
                            if self.read_word() == "grass" {
                                Token::TouchGrass
                            } else {
                                Token::Identifier("touch".to_string())
                            }
                        },
                        "go" => {
                            self.skip_whitespace();
                            if self.read_word() == "outside" {
                                Token::GoOutside
                            } else {
                                Token::Identifier("go".to_string())
                            }
                        },
                        "fr" => {
                            self.skip_whitespace();
                            if self.read_word() == "fr" {
                                Token::FrFr
                            } else {
                                Token::Identifier("fr".to_string())
                            }
                        },
                        "cap" => Token::Cap,
                        "no_cap" => Token::NoCap,
                        "bugatti" => Token::Bugatti,
                        "if" => Token::If,
                        "print" => Token::Print,
                        "as" => Token::As,
                        "then" => Token::Then,
                        "number" => Token::NumberType,
                        _ => Token::Identifier(word),
                    }
                } else if self.ch.is_numeric() {
                    Token::Number(self.read_number())
                } else {
                    Token::EOF
                }
            }
        };

        if tok != Token::EOF {
            self.read_char();
        }
        tok
    }

    fn read_word(&mut self) -> String {
        let mut word = String::new();
        while self.ch.is_alphabetic() {
            word.push(self.ch);
            self.read_char();
        }
        word
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        while self.ch != '"' && self.ch != '\0' {
            string.push(self.ch);
            self.read_char();
        }
        string
    }

    fn read_number(&mut self) -> i64 {
        let mut num = String::new();
        while self.ch.is_numeric() {
            num.push(self.ch);
            self.read_char();
        }
        num.parse().unwrap_or(0)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}