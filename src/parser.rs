use crate::token::Token;
use crate::ast::Node;

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

    fn parse_expression(&mut self) -> Option<Node> {
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