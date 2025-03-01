use std::collections::HashMap;
use crate::ast::{Node, Operator};

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

pub struct Interpreter {
    pub env: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, node: Node) {
        match node {
            Node::Program(statements) => {
                for stmt in statements {
                    self.execute(stmt);
                }
            }
            _ => { self.execute(node); }
        }
    }

    fn execute(&mut self, node: Node) -> Value {
        match node {
            Node::VarDeclaration { name, var_type: _, value } => {
                let val = self.evaluate(*value);
                self.env.insert(name.clone(), val.clone());
                val
            }
            Node::Assignment { name, value } => {
                let val = self.evaluate(*value);
                if self.env.contains_key(&name) {
                    self.env.insert(name, val.clone());
                } else {
                    eprintln!("Undefined variable: {}", name);
                }
                val
            }
            Node::Print(expr) => {
                let val = self.evaluate(*expr);
                println!("{}", val);
                val
            }
            Node::WhileLoop { condition, body } => {
                loop {
                    // Evaluate the condition and store the result
                    let cond_value = self.evaluate((*condition).clone());
                    // If the condition is false, break out of the loop
                    if !self.is_truthy(cond_value) {
                        break;
                    }
                    // Execute the body of the loop
                    self.execute((*body).clone());
                }
                Value::Null
            }
            Node::Block(statements) => {
                let mut last = Value::Null;
                for stmt in statements {
                    last = self.execute(stmt);
                }
                last
            }
            Node::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                self.evaluate_binary_op(left_val, operator, right_val)
            }
            Node::Identifier(name) => {
                self.env.get(&name).cloned().unwrap_or(Value::Null)
            }
            Node::NumberLiteral(n) => Value::Number(n),
            Node::StringLiteral(s) => Value::String(s),
            Node::Boolean(b) => Value::Boolean(b),
            Node::Bugatti => Value::String("Bugatti!".to_string()),
            _ => Value::Null,
        }
    }

    fn evaluate(&mut self, node: Node) -> Value {
        self.execute(node)
    }

    fn evaluate_binary_op(&self, left: Value, operator: Operator, right: Value) -> Value {
        match operator {
            Operator::Plus => {
                if let (Value::Number(a), Value::Number(b)) = (left, right) {
                    Value::Number(a + b)
                } else {
                    Value::Null
                }
            }
            Operator::Minus => {
                if let (Value::Number(a), Value::Number(b)) = (left, right) {
                    Value::Number(a - b)
                } else {
                    Value::Null
                }
            }
            Operator::GreaterThan => {
                if let (Value::Number(a), Value::Number(b)) = (left, right) {
                    Value::Boolean(a > b)
                } else {
                    Value::Null
                }
            }
            Operator::LessThan => {
                if let (Value::Number(a), Value::Number(b)) = (left, right) {
                    Value::Boolean(a < b)
                } else {
                    Value::Null
                }
            }
            Operator::Equals => {
                if let (Value::Number(a), Value::Number(b)) = (left, right) {
                    Value::Boolean(a == b)
                } else {
                    Value::Null
                }
            }
        }
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Boolean(b) => b,
            Value::Null => false,
            Value::Number(n) => n != 0,
            Value::String(s) => !s.is_empty(),
        }
    }
}
