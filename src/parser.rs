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
            } else {
                // Skip token on error to avoid infinite loops
                self.advance();
            }
        }

        Node::Program(statements)
    }

    fn parse_statement(&mut self) -> Option<Node> {
        match self.peek_token() {
            Some(Token::TouchGrass) => self.parse_var_declaration(),
            Some(Token::Print) => self.parse_print_statement(),
            Some(Token::Go) => self.parse_go_outside(),
            _ => None,
        }
    }

    fn parse_var_declaration(&mut self) -> Option<Node> {
        self.advance(); // consume TouchGrass

        let var_type = if let Some(Token::NumberType) = self.peek_token() {
            self.advance();
            "number".to_string()
        } else {
            eprintln!("Error: Expected number type after touch grass");
            return None;
        };

        let name = match self.peek_token() {
            Some(Token::Identifier(ref name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => {
                eprintln!("Error: Expected identifier for variable name");
                return None;
            }
        };

        if let Some(Token::As) = self.peek_token() {
            self.advance();
        } else {
            eprintln!("Error: Expected 'as' after variable name");
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
        self.advance(); // consume 'go'

        if let Some(Token::Outside) = self.peek_token() {
            self.advance();
        } else {
            eprintln!("Error: Expected 'outside' after 'go'");
            return None;
        }

        if let Some(Token::If) = self.peek_token() {
            self.advance();
        } else {
            eprintln!("Error: Expected 'if' after 'go outside'");
            return None;
        }

        let condition = self.parse_expression()?;

        if let Some(Token::Then) = self.peek_token() {
            self.advance();
        } else {
            eprintln!("Error: Expected 'then' after condition");
            return None;
        }

        let then_branch = self.parse_block()?;

        let else_branch = if let Some(Token::Instead) = self.peek_token() {
            self.advance();
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };

        Some(Node::GoOutside {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn parse_block(&mut self) -> Option<Node> {
        let mut statements = Vec::new();
    
        while !self.is_at_end() {
            if let Some(Token::FrFr) = self.peek_token() {
                self.advance(); // consume FrFr and end the block
                break;
            }
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                // In case of error, advance to avoid infinite loop
                self.advance();
            }
        }
    
        Some(Node::Block(statements))
    }

    // Expression parser supporting comparisons and addition/subtraction
    fn parse_expression(&mut self) -> Option<Node> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Option<Node> {
        let mut left = self.parse_term()?;

        while let Some(tok) = self.peek_token() {
            let op = match tok {
                Token::GreaterThan => Operator::GreaterThan,
                Token::LessThan => Operator::LessThan,
                Token::Equals => Operator::Equals,
                _ => break,
            };
            self.advance(); // consume operator
            let right = self.parse_term()?;
            left = Node::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        Some(left)
    }

    // Added support for addition and subtraction
    fn parse_term(&mut self) -> Option<Node> {
        let mut left = self.parse_factor()?;

        while let Some(tok) = self.peek_token() {
            let op = match tok {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                _ => break,
            };
            self.advance(); // consume operator
            let right = self.parse_factor()?;
            left = Node::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        Some(left)
    }

    // Parse primary values: numbers, strings, booleans, identifiers, bugatti literal.
    fn parse_factor(&mut self) -> Option<Node> {
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
            _ => {
                eprintln!("Error: Unexpected token {:?}", self.peek_token());
                self.advance();
                None
            }
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
