use crate::token::Token;
use crate::ast::{Node, Operator};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Node {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }

        Node::Program(statements)
    }

    fn parse_statement(&mut self) -> Option<Node> {
        match self.peek_token() {
            Some(Token::TouchGrass) => self.parse_var_declaration(),
            Some(Token::Print) => self.parse_print_statement(),
            Some(Token::GoOutside) => self.parse_go_outside(),
            _ => None,
        }
    }

    fn parse_var_declaration(&mut self) -> Option<Node> {
        self.advance(); // consume TouchGrass
        
        let var_type = if let Some(Token::NumberType) = self.peek_token() {
            self.advance();
            "number".to_string()
        } else {
            return None;
        };

        let name = match self.peek_token() {
            Some(Token::Identifier(ref name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => return None,
        };

        if let Some(Token::As) = self.peek_token() {
            self.advance();
        } else {
            return None;
        }

        let value = self.parse_expression()?;

        Some(Node::VarDeclaration {
            name,
            var_type,
            value: Box::new(value),
        })
    }

    fn parse_print_statement(&mut self) -> Option<Node> {
        self.advance(); // consume Print
        let expr = self.parse_expression()?;
        Some(Node::Print(Box::new(expr)))
    }

    fn parse_go_outside(&mut self) -> Option<Node> {
        self.advance(); // consume 'go outside'

        if let Some(Token::If) = self.peek_token() {
            self.advance();
        } else {
            return None;
        }

        let condition = self.parse_expression()?;

        if let Some(Token::Then) = self.peek_token() {
            self.advance();
        } else {
            return None;
        }

        let then_branch = self.parse_block()?;

        Some(Node::GoOutside {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
        })
    }

    fn parse_block(&mut self) -> Option<Node> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.peek_token()? {
                Token::FrFr => {
                    self.advance();
                    break;
                }
                _ => {
                    if let Some(stmt) = self.parse_statement() {
                        statements.push(stmt);
                    }
                }
            }
        }

        Some(Node::Block(statements))
    }

    fn parse_expression(&mut self) -> Option<Node> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Option<Node> {
        let left = self.parse_primary()?;

        match self.peek_token()? {
            Token::GreaterThan => {
                self.advance();
                let right = self.parse_primary()?;
                Some(Node::BinaryOp {
                    left: Box::new(left),
                    operator: Operator::GreaterThan,
                    right: Box::new(right),
                })
            }
            Token::LessThan => {
                self.advance();
                let right = self.parse_primary()?;
                Some(Node::BinaryOp {
                    left: Box::new(left),
                    operator: Operator::LessThan,
                    right: Box::new(right),
                })
            }
            Token::Equals => {
                self.advance();
                let right = self.parse_primary()?;
                Some(Node::BinaryOp {
                    left: Box::new(left),
                    operator: Operator::Equals,
                    right: Box::new(right),
                })
            }
            _ => Some(left),
        }
    }

    fn parse_primary(&mut self) -> Option<Node> {
        match self.peek_token()? {
            Token::Number(n) => {
                let val = *n;
                self.advance();
                Some(Node::NumberLiteral(val))
            }
            Token::String(ref s) => {
                let val = s.clone();
                self.advance();
                Some(Node::StringLiteral(val))
            }
            Token::Cap => {
                self.advance();
                Some(Node::Boolean(false))
            }
            Token::NoCap => {
                self.advance();
                Some(Node::Boolean(true))
            }
            Token::Bugatti => {
                self.advance();
                Some(Node::Bugatti)
            }
            Token::Identifier(ref name) => {
                let val = name.clone();
                self.advance();
                Some(Node::Identifier(val))
            }
            _ => None,
        }
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}