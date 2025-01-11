use std::collections::HashMap;
use crate::ast::{Node, Operator};

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
    Bugatti,
}

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, node: Node) -> Result<Option<Value>, String> {
        match node {
            Node::Program(statements) => {
                let mut result = None;
                for stmt in statements {
                    result = self.interpret(stmt)?;
                }
                Ok(result)
            },
            Node::Print(expr) => {
                let value = self.interpret(*expr)?;
                match value {
                    Some(Value::String(s)) => println!("{}", s),
                    Some(Value::Number(n)) => println!("{}", n),
                    Some(Value::Boolean(true)) => println!("no cap"),
                    Some(Value::Boolean(false)) => println!("cap"),
                    Some(Value::Bugatti) => println!("bugatti"),
                    None => println!("bugatti moment"),
                }
                Ok(None)
            },
            Node::VarDeclaration { name, var_type: _, value } => {
                let evaluated = self.interpret(*value)?;
                if let Some(val) = evaluated {
                    self.environment.set(name, val);
                }
                Ok(None)
            },
            Node::GoOutside { condition, then_branch, else_branch } => {
                let cond_value = self.interpret(*condition)?;
                match cond_value {
                    Some(Value::Boolean(true)) => self.interpret(*then_branch),
                    Some(Value::Boolean(false)) => {
                        if let Some(else_branch) = else_branch {
                            self.interpret(*else_branch)
                        } else {
                            Ok(None)
                        }
                    },
                    _ => Err("bro that's not a condition fr fr".to_string()),
                }
            },
            Node::BinaryOp { left, operator, right } => {
                let left_val = self.interpret(*left)?;
                let right_val = self.interpret(*right)?;
                self.evaluate_binary_op(left_val, operator, right_val)
            },
            Node::NumberLiteral(n) => Ok(Some(Value::Number(n))),
            Node::StringLiteral(s) => Ok(Some(Value::String(s))),
            Node::Boolean(b) => Ok(Some(Value::Boolean(b))),
            Node::Bugatti => Ok(Some(Value::Bugatti)),
            Node::Identifier(name) => {
                if let Some(value) = self.environment.get(&name) {
                    Ok(Some(value.clone()))
                } else {
                    Err(format!("skill issue: {} is not defined", name))
                }
            },
            Node::Block(statements) => {
                let mut result = None;
                for stmt in statements {
                    result = self.interpret(stmt)?;
                }
                Ok(result)
            },
        }
    }

    fn evaluate_binary_op(&self, left: Option<Value>, operator: Operator, right: Option<Value>) 
        -> Result<Option<Value>, String> {
        match (left, right) {
            (Some(Value::Number(l)), Some(Value::Number(r))) => {
                match operator {
                    Operator::GreaterThan => Ok(Some(Value::Boolean(l > r))),
                    Operator::LessThan => Ok(Some(Value::Boolean(l < r))),
                    Operator::Equals => Ok(Some(Value::Boolean(l == r))),
                }
            },
            _ => Err("skill issue: can't compare these values fr fr".to_string()),
        }
    }
}