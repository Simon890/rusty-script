use std::ops::{Add, Div, Mul, Sub};

use super::parser::{ASTNode, Parser};

pub struct Interpreter {
    ast: Vec<ASTNode>,
}

impl Interpreter {

    pub fn new() -> Self {
        Interpreter {
            ast: vec![]
        }
    }

    pub fn run(&mut self, input: &str) -> RuntimeValue {
        self.ast = Parser::new(input).parse();

        let mut last_value = RuntimeValue::Null;
        for node in &self.ast {
            last_value = self.initial_expression(node.clone());
        }
        last_value
    }

    fn initial_expression(&self, node: ASTNode) -> RuntimeValue {
        match node {
            ASTNode::Number(value) => RuntimeValue::Number(value),
            ASTNode::String(value) => RuntimeValue::String(value),
            ASTNode::Bool(value) => RuntimeValue::Bool(value),
            ASTNode::BinaryExpression { .. } => self.binary_expression(node),
            ASTNode::UnaryExpression { .. } => self.unary_expression(node),
            ASTNode::FunctionCall { .. } => self.function_call(node),
        }
    }

    fn binary_expression(&self, node: ASTNode) -> RuntimeValue {
        if let ASTNode::BinaryExpression { left, right, operator } = node {
            let left = self.initial_expression(*left);
            let right = self.initial_expression(*right);
            return match operator {
                '-' => left - right,
                '+' => left + right,
                '*' => left * right,
                '/' => left / right,
                '^' => left.pow(&right),
                _ => unreachable!("Unexpected operator")
            };
        }
        unreachable!("Expected BinaryExpression node!");
    }

    fn unary_expression(&self, node: ASTNode) -> RuntimeValue {
        if let ASTNode::UnaryExpression { sign, expr } = node {
            let value = self.initial_expression(*expr);
            return value.mul(RuntimeValue::Number(if sign == '-' {-1.0} else {1.0}));
        }
        unreachable!("Expected UnaryExpression node!");
    }

    fn function_call(&self, node: ASTNode) -> RuntimeValue {
        if let ASTNode::FunctionCall { name, args } = node {
            return RuntimeValue::Bool(false);
        }
        unreachable!("Expected FunctionCall node!")
    }
    
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Number(f32),
    String(String),
    Bool(bool),
    Null
}

impl Sub for RuntimeValue {
    type Output = RuntimeValue;
    fn sub(self, rhs: Self) -> Self::Output {
        if let (Self::Number(left_value), Self::Number(right_value)) = (&self, &rhs) {
            return Self::Number(left_value - right_value);
        }
        panic!("Cannot subtract {:?} and {:?}", self, rhs)
    }
}

impl Add for RuntimeValue {
    type Output = RuntimeValue;
    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Number(left_value), Self::Number(right_value)) => Self::Number(left_value + right_value),
            (Self::String(left_value), Self::String(right_value)) => Self::String(format!("{}{}", left_value, right_value)),
            (Self::Number(left_value), Self::String(right_value)) => Self::String(format!("{}{}", left_value, right_value)),
            (Self::String(left_value), Self::Number(right_value)) => Self::String(format!("{}{}", left_value, right_value)),
            _ => panic!("Cannot add {:?} and {:?}", self, rhs)
        }
    }
}

impl Mul for RuntimeValue {
    type Output = RuntimeValue;
    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Number(left_value), Self::Number(right_value)) => Self::Number(left_value * right_value),
            (Self::Number(left_value), Self::String(right_value)) => Self::String(right_value.repeat(*left_value as usize)),
            (Self::String(left_value), Self::Number(right_value)) => Self::String(left_value.repeat(*right_value as usize)),
            _ => panic!("Cannot multiply {:?} and {:?}", self, rhs)
        }
    }
}

impl Div for RuntimeValue {
    type Output = RuntimeValue;
    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Number(left_value), Self::Number(right_value)) => {
                if *right_value == 0.0 {
                    panic!("Cannot divide by zero: {} / {}", left_value, right_value);
                }
                Self::Number(left_value / right_value)
            }
            _ => panic!("Cannot divide {:?} and {:?}", self, rhs)
        }
    }
}

impl RuntimeValue {
    fn pow(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Self::Number(left_value), Self::Number(right_value)) => {
                Self::Number(left_value.powf(*right_value))
            }
            _ => panic!("Cannot rise {:?} to the power of {:?}", self, rhs)
        }
    }
}