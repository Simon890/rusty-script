use std::{ops::{Add, Div, Mul, Sub}, rc::Rc};

use super::{env::Env, func::function_registry::{FunctionRegistry, RuntimeType}, parser::{ASTNode, Parser}};

pub struct Interpreter {
    env: Env,
    fr: FunctionRegistry,
}

impl Interpreter {

    pub fn new() -> Self {
        Interpreter {
            env: Env::new(None),
            fr: FunctionRegistry::new()
        }
    }

    pub fn run(&mut self, input: &str) -> RuntimeValue {
        let ast = Parser::new(input).parse();
        let mut last_value = RuntimeValue::Null;
        for node in ast {
            last_value = self.initial_expression(node);
        }
        last_value
    }

    fn initial_expression(&mut self, node: Rc<ASTNode>) -> RuntimeValue {
        match node.as_ref() {
            ASTNode::Number(value) => RuntimeValue::Number(*value),
            ASTNode::String(value) => RuntimeValue::String(value.clone()),
            ASTNode::Bool(value) => RuntimeValue::Bool(*value),
            ASTNode::BinaryExpression { .. } => self.binary_expression(node),
            ASTNode::UnaryExpression { .. } => self.unary_expression(node),
            ASTNode::FunctionCall { .. } => self.function_call(node),
            ASTNode::VarDeclaration { .. } => self.var_declaration(node),
            ASTNode::VarAssignment { .. } => self.var_assignment(node),
            ASTNode::Identifier { name } => self.env.get(name).clone()
        }
    }

    fn var_declaration(&mut self, node: Rc<ASTNode>) -> RuntimeValue {
        if let ASTNode::VarDeclaration { name, value } = node.as_ref() {
            let var_value = self.initial_expression(Rc::clone(value));
            self.env.add(name.to_owned(), var_value);
            return RuntimeValue::Null;
        }
        unreachable!("Expected VarDeclaration node!");
    }

    fn var_assignment(&mut self, node: Rc<ASTNode>) -> RuntimeValue {
        if let ASTNode::VarAssignment { name, value } = node.as_ref() {
            let var_value = self.initial_expression(Rc::clone(value));
            self.env.update(name.to_owned(), var_value);
            return RuntimeValue::Null;
        }
        unreachable!("Expected VarAssignment node!");
    }

    fn binary_expression(&mut self, node: Rc<ASTNode>) -> RuntimeValue {
        if let ASTNode::BinaryExpression { left, right, operator } = node.as_ref() {
            let left = self.initial_expression(Rc::clone(left));
            let right = self.initial_expression(Rc::clone(right));
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

    fn unary_expression(&mut self, node: Rc<ASTNode>) -> RuntimeValue {
        if let ASTNode::UnaryExpression { sign, expr } = node.as_ref() {
            let value = self.initial_expression(Rc::clone(expr));
            return value.mul(RuntimeValue::Number(if *sign == '-' {-1.0} else {1.0}));
        }
        unreachable!("Expected UnaryExpression node!");
    }

    fn function_call(&mut self, node: Rc<ASTNode>) -> RuntimeValue {
        if let ASTNode::FunctionCall { name, args } = node.as_ref() {
            let runtime_values = args.iter().map(|node| self.initial_expression(Rc::clone(node))).collect();
            return self.fr.call(name, runtime_values);
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

impl RuntimeValue {
    pub fn to_type(&self) -> RuntimeType {
        match self {
            Self::Bool(..) => RuntimeType::Bool,
            Self::Number(..) => RuntimeType::Number,
            Self::String(..) => RuntimeType::String,
            Self::Null => RuntimeType::Null,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn i_test_var_dec() {
        let mut i = Interpreter::new();
        let output = i.run(r#"
            let x = 10 + 5;
            let y = x * 2;
            y = 8;
            print("The result is " + y);
            "#);
        // dbg!(&i.env);
        dbg!(&output);
        dbg!(&i.env);
    }
}