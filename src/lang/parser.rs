use std::rc::Rc;

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

    pub fn parse(&mut self) -> Vec<Rc<ASTNode>> {
        let mut tokens: Vec<Rc<ASTNode>> = vec![];
        while !self.is_eof() {
            let token = match self.current() {
                Token::Identifier { value } if value == "let" => self.parse_var_declaration(),
                Token::Identifier { value } => {
                    if self.expect(TokenKind::LeftParen) {
                        self.parse_function()
                    } else {
                        self.advance(None);
                        Rc::new(ASTNode::Identifier { name: value })
                    }
                },
                value => {
                    if self.is_expr(&value) {
                        self.parse_sum_expression();
                    }
                    panic!("Not recognized token!")
                }
            };
            self.advance(Some(TokenKind::SemiColon));
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

    fn is_expr(&self, token: &Token) -> bool {
        matches!(
            token.kind(), 
            TokenKind::NumberLiteral | 
            TokenKind::StringLiteral | 
            TokenKind::LeftParen | 
            TokenKind::Identifier |
            TokenKind::SubOp |
            TokenKind::AddOp
        )
    }

    fn parse_expr(&mut self) -> Rc<ASTNode> {
        match self.current() {
            Token::LeftParen => {
                self.advance(None);
                let node = self.parse_sum_expression();
                self.advance(Some(TokenKind::RightParen));
                node
            },
            Token::NumberLiteral { value } => {
                self.advance(None);
                Rc::new(ASTNode::Number(value))
            },
            Token::BoolLiteral { value } => {
                self.advance(None);
                Rc::new(ASTNode::Bool(value))
            },
            Token::StringLiteral { value } => {
                self.advance(None);
                Rc::new(ASTNode::String(value))
            },
            Token::SubOp | Token::AddOp => {
                self.parse_unary_expression()
            },
            Token::Identifier { value } if value == "let" => {
                self.parse_var_declaration()
            },
            Token::Identifier { value } => {
                if self.expect(TokenKind::LeftParen) {
                    self.parse_function()
                } else {
                    self.advance(None);
                    Rc::new(ASTNode::Identifier { name: value })
                }
            },
            token => panic!("TODO! {:#?}", token)
        }
    }

    fn parse_var_declaration(&mut self) -> Rc<ASTNode> {
        self.advance(Some(TokenKind::Identifier));
        let var_name = self.advance(Some(TokenKind::Identifier));
        self.advance(Some(TokenKind::EqOp));
        let value = self.parse_sum_expression();
        Rc::new(
            ASTNode::VarDeclaration { name: var_name.as_string(), value }
        )
    }

    fn parse_function(&mut self) -> Rc<ASTNode> {
        let ident = self.advance(Some(TokenKind::Identifier));
        self.advance(Some(TokenKind::LeftParen));
        let args = self.parse_args();
        self.advance(Some(TokenKind::RightParen));
        return Rc::new(
            ASTNode::FunctionCall { name: ident.as_string(), args }
        );
    }

    fn parse_args(&mut self) -> Vec<Rc<ASTNode>> {
        let mut args: Vec<Rc<ASTNode>> = Vec::new();
        while !self.is_eof() && self.current().kind() != TokenKind::RightParen {
            let arg = self.parse_sum_expression();
            args.push(arg);
            if self.current().kind() == TokenKind::RightParen {
                break;
            }
            self.advance(Some(TokenKind::Comma));
        }
        args
    }

    fn parse_pow_expression(&mut self) -> Rc<ASTNode> {
        let mut left = self.parse_expr();
        while !self.is_eof() && self.current().kind() == TokenKind::PowOp {
            self.advance(Some(TokenKind::PowOp));
            let right = self.parse_expr();
            left = Rc::new(ASTNode::BinaryExpression { left, right, operator: '^' })
        }
        left
    }

    fn parse_sum_expression(&mut self) -> Rc<ASTNode> {
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
            left = Rc::new(ASTNode::BinaryExpression { left, right , operator })
        }
        left
    }

    fn parse_mul_expression(&mut self) -> Rc<ASTNode> {
        let mut left = self.parse_pow_expression();
        while !self.is_eof() && self.current().kind() == TokenKind::MulOp {
            self.advance(Some(TokenKind::MulOp));
            let right = self.parse_pow_expression();
            left = Rc::new(ASTNode::BinaryExpression { left, right, operator: '*' })
        }
        left
    }

    fn parse_unary_expression(&mut self) -> Rc<ASTNode> {
        let token_sign = self.advance(None);
        let sign = match token_sign {
            Token::SubOp => '-',
            Token::AddOp => '+',
            _ => unreachable!("Unexpected sign")
        };
        let expression = self.parse_expr();
        Rc::new(
            ASTNode::UnaryExpression { sign, expr: expression }
        )
    }
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f32),
    Bool(bool),
    String(String),
    Identifier {
        name: String
    },
    FunctionCall {
        name: String,
        args: Vec<Rc<ASTNode>>
    },
    BinaryExpression {
        left: Rc<ASTNode>,
        right: Rc<ASTNode>,
        operator: char,
    },
    UnaryExpression {
        sign: char,
        expr: Rc<ASTNode>
    },
    VarDeclaration {
        name: String,
        value: Rc<ASTNode>
    },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_var_declaration() {
        let mut p = Parser::new("let x = 4;");
        dbg!(&p.parse());
    }
}