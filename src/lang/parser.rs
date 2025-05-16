use super::tokenizer::{Token, TokenKind, Tokenizer};

pub struct Parser {
    pos: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(text: &str) -> Self {
        let tokens = Tokenizer::new(text).tokenize();
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut tokens: Vec<ASTNode> = vec![];
        while !self.is_eof() {
            let token = self.parse_sum_expression();
            tokens.push(token);
        }
        tokens
    }

    fn current(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    fn is_eof(&self) -> bool {
        matches!(self.tokens.get(self.pos), Some(Token::EOF) | None)
    }

    fn advance(&mut self, expected: Option<TokenKind>) -> Token {
        match expected {
            Some(kind) => {
                if self.current().kind() == kind {
                    let t = self.current();
                    self.pos += 1;
                    return t;
                }
                panic!("Unexpected token {:?}. Expected: {:?}", self.current().kind(), kind);
            },
            None => {
                let t = self.current();
                self.pos += 1;
                t
            }
        }
    }

    fn expect(&self, token: TokenKind) -> bool {
        if self.is_eof() {
            return false;
        }
        let next_token = self.tokens.get(self.pos + 1).unwrap();
        token == next_token.kind()
    }

    fn parse_expression(&mut self) -> ASTNode {
        match self.current() {
            Token::NumberLiteral { value } => {
                self.advance(None);
                ASTNode::Number(value)
            },
            Token::BoolLiteral { value } => {
                self.advance(None);
                ASTNode::Bool(value)
            },
            Token::StringLiteral { value } => {
                self.advance(None);
                ASTNode::String(value)
            },
            Token::Identifier { .. } => {
                self.parse_function()
            },
            Token::SubOp | Token::AddOp => {
                self.parse_unary_expression()
            },
            token => panic!("TODO! {:#?}", token)
        }
    }

    fn parse_function(&mut self) -> ASTNode {
        let ident = self.advance(Some(TokenKind::Identifier));
        self.advance(Some(TokenKind::LeftParen));
        let args = self.parse_args();
        self.advance(Some(TokenKind::RightParen));
        return ASTNode::FunctionCall { name: ident.as_string(), args: Box::new(args) };
    }

    fn parse_args(&mut self) -> Vec<ASTNode> {
        let mut args: Vec<ASTNode> = Vec::new();
        while !self.is_eof() && self.current().kind() != TokenKind::RightParen {
            let arg = self.parse_expression();
            args.push(arg);
            if self.current().kind() == TokenKind::RightParen {
                break;
            }
            self.advance(Some(TokenKind::Comma));
        }
        args
    }

    fn parse_pow_expression(&mut self) -> ASTNode {
        let mut left = self.parse_expression();
        while !self.is_eof() && self.current().kind() == TokenKind::PowOp {
            self.advance(Some(TokenKind::PowOp));
            let right = self.parse_expression();
            left = ASTNode::BinaryExpression { left: Box::new(left), right: Box::new(right), operator: '^' }
        }
        left
    }

    fn parse_sum_expression(&mut self) -> ASTNode {
        let mut left = self.parse_mul_expression();
        while !self.is_eof() && (self.current().kind() == TokenKind::AddOp || self.current().kind() == TokenKind::SubOp) {
            let expect = match self.current().kind() {
                TokenKind::AddOp => Some(TokenKind::AddOp),
                _ => Some(TokenKind::SubOp)
            };
            let math_op = self.advance(expect);
            let operator = match math_op {
                Token::AddOp => '+',
                Token::SubOp => '-',
                _ => unreachable!("Unexpected operator")
            };
            let right = self.parse_mul_expression();
            left = ASTNode::BinaryExpression { left: Box::new(left), right: Box::new(right) , operator }
        }
        left
    }

    fn parse_mul_expression(&mut self) -> ASTNode {
        let mut left = self.parse_pow_expression();
        while !self.is_eof() && self.current().kind() == TokenKind::MulOp {
            self.advance(Some(TokenKind::MulOp));
            let right = self.parse_pow_expression();
            left = ASTNode::BinaryExpression { left: Box::new(left), right: Box::new(right), operator: '*' }
        }
        left
    }

    fn parse_unary_expression(&mut self) -> ASTNode {
        let token_sign = self.advance(None);
        let sign = match token_sign {
            Token::SubOp => '-',
            Token::AddOp => '+',
            _ => unreachable!("Unexpected sign")
        };
        let expression = self.parse_expression();
        ASTNode::UnaryExpression { sign, expr: Box::new(expression) }
    }
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f32),
    Bool(bool),
    String(String),
    FunctionCall {
        name: String,
        args: Box<Vec<ASTNode>>
    },
    BinaryExpression {
        left: Box<ASTNode>,
        right: Box<ASTNode>,
        operator: char,
    },
    UnaryExpression {
        sign: char,
        expr: Box<ASTNode>
    }
}