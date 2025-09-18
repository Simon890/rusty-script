use super::{panics::{casting_error, unexpected_eof, unexpected_token}, reg_exp::TokenRegEx};

pub struct Tokenizer<'a> {
    pos: u32,
    text: &'a str,
    tokens: Vec<Token>
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Tokenizer { pos: 0, text: text.trim(), tokens: vec![] }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.is_eof() {
            self.skip_empty_space();
            let current = self.current();
            
            if self.is_char(&current) {
                let identifier = self.identifier();
                if identifier == "true" || identifier == "false" {
                    self.tokens.push(Token::BoolLiteral { value: identifier == "true" });
                } else {
                    self.tokens.push(Token::Identifier { value: identifier });
                }
                continue;
            }

            if self.is_number(&current) || (self.is_decimal_point(&current) && self.is_number(&self.next())) {
                let value = self.number();
                self.tokens.push(Token::NumberLiteral { value });
                continue;
            }

            if self.is_simple_quote(&current) || self.is_double_quote(&current) {
                let value = self.string(&current);
                self.tokens.push(Token::StringLiteral { value });
                continue;
            }

            if self.is_semicolon(&current) {
                self.advance();
                self.tokens.push(Token::SemiColon);
                continue;
            }

            if self.is_left_paren(&current) {
                self.advance();
                self.tokens.push(Token::LeftParen);
                continue;
            }

            if self.is_right_paren(&current) {
                self.advance();
                self.tokens.push(Token::RightParen);
                continue;
            }

            if self.is_left_sq_brace(&current) {
                self.advance();
                self.tokens.push(Token::LeftSqBrace);
                continue;
            }

            if self.is_right_sq_brace(&current) {
                self.advance();
                self.tokens.push(Token::RightSqBrace);
                continue;
            }

            if self.is_left_curly_brace(&current) {
                self.advance();
                self.tokens.push(Token::LeftCurlyBrace);
                continue;
            }

            if self.is_right_curly_brace(&current) {
                self.advance();
                self.tokens.push(Token::RightCurlyBrace);
                continue;
            }

            //Operators

            if self.is_eq_op(&current) {
                self.advance();
                self.tokens.push(Token::EqOp);
                continue;
            }

            if self.is_negation_op(&current) {
                self.advance();
                if self.is_eq_op(&self.current()) {
                    self.advance();
                    self.tokens.push(Token::NotEqOp);
                    continue;
                }
                self.tokens.push(Token::NegationOp);
                continue;
            }

            if self.is_add_op(&current) {
                self.advance();
                self.tokens.push(Token::AddOp);
                continue;
            }

            if self.is_sub_op(&current) {
                self.advance();
                self.tokens.push(Token::SubOp);
                continue;
            }

            if self.is_mul_op(&current) {
                self.advance();
                self.tokens.push(Token::MulOp);
                continue;
            }

            if self.is_div_op(&current) {
                self.advance();
                self.tokens.push(Token::DivOp);
                continue;
            }

            if self.is_pow_op(&current) {
                self.advance();
                self.tokens.push(Token::PowOp);
                continue;
            }

            if self.is_gt_op(&current) {
                self.advance();
                self.tokens.push(Token::GtOp);
                continue;
            }

            if self.is_lt_op(&current) {
                self.advance();
                self.tokens.push(Token::LtOp);
                continue;
            }

            if self.is_comma(&current) {
                self.advance();
                self.tokens.push(Token::Comma);
                continue;
            }

            unexpected_token(&current, &self.pos);
        }
        self.tokens.push(Token::EOF);
        self.tokens.clone()
    }
    
    fn skip_empty_space(&mut self) {
        while self.is_empty_space(&self.current()) {
            self.advance();
        }
    }
    
    fn advance(&mut self) {
        self.pos += 1;
    }
    
    fn current(&self) -> String {
        match self.text.chars().nth(self.pos as usize) {
            Some(value) => value.to_string(),
            None => unexpected_eof(&self.pos),
        }
    }

    fn next(&self) -> String {
        match self.text.chars().nth((self.pos + 1) as usize) {
            Some(value) => value.to_string(),
            None => unexpected_eof(&self.pos)
        }
    }
    
    fn is_empty_space(&self, value: &str) -> bool {
        TokenRegEx::EmptySpace.test(value)
    }

    fn identifier(&mut self) -> String {
        let mut value = String::new();
        while !self.is_eof() && self.is_char(&self.current()) {
            value.push_str(&self.current());
            self.advance();
        }
        value
    }

    fn number(&mut self) -> f32 {
        let mut value = String::new();
        let mut is_there_decimal_point = false;
        while !self.is_eof() && (self.is_number(&self.current()) || self.is_decimal_point(&self.current())) {
            if is_there_decimal_point && self.is_decimal_point(&self.current()) {
                panic!("Invalid format number at position {}", self.pos);
            }
            if !is_there_decimal_point {
                is_there_decimal_point = self.is_decimal_point(&self.current());
            }
            value.push_str(&self.current());
            self.advance();
        }
        value.parse::<f32>().unwrap()
    }

    fn string(&mut self, quote_type: &str) -> String {
        self.advance();
        let mut value = String::new();
        while self.current() != quote_type {
            if self.is_eof() {
                unexpected_eof(&self.pos);
            }
            value.push_str(&self.current());
            self.advance();
        }
        self.advance();
        value
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.text.len().try_into().unwrap()
    }

    fn is_char(&self, value: &str) -> bool {
        TokenRegEx::Char.test(value)
    }

    fn is_number(&self, value: &str) -> bool {
        TokenRegEx::Number.test(value)
    }

    fn is_simple_quote(&self, value: &str) -> bool {
        TokenRegEx::SimpleQuote.test(value)
    }

    fn is_double_quote(&self, value: &str) -> bool {
        TokenRegEx::DoubleQuote.test(value)
    }

    fn is_semicolon(&self, value: &str) -> bool {
        TokenRegEx::SemiColon.test(value)
    }

    fn is_left_paren(&self, value: &str) -> bool {
        TokenRegEx::LeftParen.test(value)
    }
    
    fn is_right_paren(&self, value: &str) -> bool {
        TokenRegEx::RightParen.test(value)
    }

    fn is_eq_op(&self, value: &str) -> bool {
        TokenRegEx::EqOp.test(value)
    }

    fn is_add_op(&self, value: &str) -> bool {
        TokenRegEx::AddOp.test(value)
    }

    fn is_sub_op(&self, value: &str) -> bool {
        TokenRegEx::SubOp.test(value)
    }
    
    fn is_mul_op(&self, value: &str) -> bool {
        TokenRegEx::MulOp.test(value)
    }
    
    fn is_div_op(&self, value: &str) -> bool {
        TokenRegEx::DivOp.test(value)
    }
    
    fn is_pow_op(&self, value: &str) -> bool {
        TokenRegEx::PowOp.test(value)
    }

    fn is_gt_op(&self, value: &str) -> bool {
        TokenRegEx::GtOp.test(value)
    }

    fn is_lt_op(&self, value: &str) -> bool {
        TokenRegEx::LtOp.test(value)
    }

    fn is_negation_op(&self, value: &str) -> bool {
        TokenRegEx::NegationOp.test(value)
    }

    fn is_left_sq_brace(&self, value: &str) -> bool {
        TokenRegEx::LeftSqBrace.test(value)
    }

    fn is_right_sq_brace(&self, value: &str) -> bool {
        TokenRegEx::RightSqBrace.test(value)
    }

    fn is_left_curly_brace(&self, value: &str) -> bool {
        TokenRegEx::LeftCurlyBrace.test(value)
    }

    fn is_right_curly_brace(&self, value: &str) -> bool {
        TokenRegEx::RightCurlyBrace.test(value)
    }

    fn is_comma(&self, value: &str) -> bool {
        TokenRegEx::Comma.test(value)
    }

    fn is_decimal_point(&self, value: &str) -> bool {
        TokenRegEx::DecimalPoint.test(value)
    }

}

#[derive(Debug, Clone)]
pub enum Token {
    Identifier {
        value: String
    },
    NumberLiteral {
        value: f32
    },
    BoolLiteral {
        value: bool
    },
    StringLiteral {
        value: String
    },
    LeftParen,
    RightParen,
    SemiColon,
    EqOp,
    NotEqOp,
    SubOp,
    AddOp,
    MulOp,
    DivOp,
    PowOp,
    GtOp,
    LtOp,
    NegationOp,
    LeftSqBrace,
    RightSqBrace,
    LeftCurlyBrace,
    RightCurlyBrace,
    Comma,
    EOF
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        match self {
            Self::Identifier {..} => TokenKind::Identifier,
            Self::NumberLiteral { .. } => TokenKind::NumberLiteral,
            Self::BoolLiteral { .. } => TokenKind::BoolLiteral,
            Self::StringLiteral { .. } => TokenKind::StringLiteral,
            Self::LeftParen => TokenKind::LeftParen,
            Self::RightParen => TokenKind::RightParen,
            Self::SemiColon => TokenKind::SemiColon,
            Self::EqOp => TokenKind::EqOp,
            Self::NotEqOp => TokenKind::NotEqOp,
            Self::SubOp => TokenKind::SubOp,
            Self::AddOp => TokenKind::AddOp,
            Self::MulOp => TokenKind::MulOp,
            Self::DivOp => TokenKind::DivOp,
            Self::PowOp => TokenKind::PowOp,
            Self::GtOp => TokenKind::GtOp,
            Self::LtOp => TokenKind::LtOp,
            Self::NegationOp => TokenKind::NegationOp,
            Self::LeftSqBrace => TokenKind::LeftSqBrace,
            Self::RightSqBrace => TokenKind::RightSqBrace,
            Self::LeftCurlyBrace => TokenKind::LeftCurlyBrace,
            Self::RightCurlyBrace => TokenKind::RightCurlyBrace,
            Self::Comma => TokenKind::Comma,
            Self::EOF => TokenKind::EOF,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::Identifier { value } | Self::StringLiteral { value } => value.to_owned(),
            _ => casting_error("String")
        }
    }

    pub fn as_f32(&self) -> f32 {
        match self {
            Self::NumberLiteral { value } => *value,
            _ => casting_error("f32")
        }
    }

    pub fn as_bool(&self) -> bool {
        match &self {
            Self::BoolLiteral { value } => *value,
            _ => casting_error("bool")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Identifier,
    NumberLiteral,
    BoolLiteral,
    StringLiteral,
    LeftParen,
    RightParen,
    SemiColon,
    EqOp,
    NotEqOp,
    SubOp,
    AddOp,
    MulOp,
    DivOp,
    PowOp,
    GtOp,
    LtOp,
    NegationOp,
    LeftSqBrace,
    RightSqBrace,
    LeftCurlyBrace,
    RightCurlyBrace,
    Comma,
    EOF
}