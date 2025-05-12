use regex::Regex;

pub enum TokenRegEx {
    EmptySpace,
    Char,
    Number,
    SimpleQuote,
    DoubleQuote,
    SemiColon,
    LeftParen,
    RightParen,
    EqOp,
    AddOp,
    SubOp,
    DivOp,
    MulOp,
    PowOp,
    NegationOp,
    GtOp,
    LtOp,
}

impl TokenRegEx {
    pub fn test(&self, value: &str) -> bool {
        match self {
            TokenRegEx::EmptySpace => Regex::new(r"[\s\t\n\r]").unwrap().is_match(value),
            TokenRegEx::Char => Regex::new(r"[a-zA-Z]").unwrap().is_match(value),
            TokenRegEx::Number => Regex::new(r"[0-9]").unwrap().is_match(value),
            TokenRegEx::SimpleQuote => Regex::new(r"[']").unwrap().is_match(value),
            TokenRegEx::DoubleQuote => Regex::new(r#"["]"#).unwrap().is_match(value),
            TokenRegEx::SemiColon => Regex::new(r"[;]").unwrap().is_match(value),
            TokenRegEx::LeftParen => Regex::new(r"[(]").unwrap().is_match(value),
            TokenRegEx::RightParen => Regex::new(r"[)]").unwrap().is_match(value),
            TokenRegEx::EqOp => Regex::new(r"[\=]").unwrap().is_match(value),
            TokenRegEx::AddOp => Regex::new(r"[\+]").unwrap().is_match(value),
            TokenRegEx::SubOp => Regex::new(r"[\-]").unwrap().is_match(value),
            TokenRegEx::DivOp => Regex::new(r"[\/]").unwrap().is_match(value),
            TokenRegEx::MulOp => Regex::new(r"[\*]").unwrap().is_match(value),
            TokenRegEx::PowOp => Regex::new(r"[\^]").unwrap().is_match(value),
            TokenRegEx::GtOp => Regex::new(r"[>]").unwrap().is_match(value),
            TokenRegEx::LtOp => Regex::new(r"[<]").unwrap().is_match(value),
            TokenRegEx::NegationOp => Regex::new(r"[\!]").unwrap().is_match(value)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reg_ex_empty_space() {
        let space = TokenRegEx::EmptySpace.test(" ");
        let tab = TokenRegEx::EmptySpace.test("\t");
        let new_line = TokenRegEx::EmptySpace.test("\n");
        let character = TokenRegEx::EmptySpace.test("hello");
        assert_eq!(space, true);
        assert_eq!(tab, true);
        assert_eq!(new_line, true);
        assert_eq!(character, false);
    }
}