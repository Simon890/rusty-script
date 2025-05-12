use regex::Regex;

use super::{panics::unexpected_eof, reg_exp::TokenRegEx};

pub struct Tokenizer<'a> {
    pos: u32,
    text: &'a str,
    tokens: Vec<Token>
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Tokenizer { pos: 0, text, tokens: vec![] }
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
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

            if self.is_number(&current) {
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

            panic!("Unexpected token {} at position {}", current, self.pos);
        }
        &self.tokens
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

    fn number(&mut self) -> i32 {
        let mut value = String::new();
        while !self.is_eof() && self.is_number(&self.current()) {
            value.push_str(&self.current());
            self.advance();
        }
        value.parse::<i32>().unwrap()
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

}

#[derive(Debug)]
pub enum Token {
    Identifier {
        value: String
    },
    NumberLiteral {
        value: i32
    },
    BoolLiteral {
        value: bool
    },
    StringLiteral {
        value: String
    },
    LeftParen,
    RightParen,
    SimpleQuote,
    DoubleQuote,
    SemiColon,
}