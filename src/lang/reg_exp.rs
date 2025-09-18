use regex::Regex;

macro_rules! check_regex {
    ($pattern: expr, $value: expr) => {{
        Regex::new($pattern).unwrap().is_match($value)
    }};
}

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
    LeftSqBrace,
    RightSqBrace,
    LeftCurlyBrace,
    RightCurlyBrace,
    Comma,
    DecimalPoint,
}

impl TokenRegEx {
    pub fn test(&self, value: &str) -> bool {
        match self {
            TokenRegEx::EmptySpace => check_regex!(r"[\s\t\n\r]", value),
            TokenRegEx::Char => check_regex!(r"[a-zA-Z]", value),
            TokenRegEx::Number => check_regex!(r"[0-9]", value),
            TokenRegEx::SimpleQuote => check_regex!(r"[']", value),
            TokenRegEx::DoubleQuote => check_regex!(r#"["]"#, value),
            TokenRegEx::SemiColon => check_regex!(r"[;]", value),
            TokenRegEx::LeftParen => check_regex!(r"[(]", value),
            TokenRegEx::RightParen => check_regex!(r"[)]", value),
            TokenRegEx::EqOp => check_regex!(r"[\=]", value),
            TokenRegEx::AddOp => check_regex!(r"[\+]", value),
            TokenRegEx::SubOp => check_regex!(r"[\-]", value),
            TokenRegEx::DivOp => check_regex!(r"[\/]", value),
            TokenRegEx::MulOp => check_regex!(r"[\*]", value),
            TokenRegEx::PowOp => check_regex!(r"[\^]", value),
            TokenRegEx::GtOp => check_regex!(r"[>]", value),
            TokenRegEx::LtOp => check_regex!(r"[<]", value),
            TokenRegEx::NegationOp => check_regex!(r"[\!]", value),
            TokenRegEx::LeftSqBrace => check_regex!(r"[\[]", value),
            TokenRegEx::RightSqBrace => check_regex!(r"[\]]", value),
            TokenRegEx::LeftCurlyBrace => check_regex!(r"[\{]", value),
            TokenRegEx::RightCurlyBrace => check_regex!(r"[\}]", value),
            TokenRegEx::Comma => check_regex!(r"[\,]", value),
            TokenRegEx::DecimalPoint => check_regex!(r"[\.]", value),
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
        let decimal_point = TokenRegEx::DecimalPoint.test(".");
        assert_eq!(space, true);
        assert_eq!(tab, true);
        assert_eq!(new_line, true);
        assert_eq!(character, false);
        assert_eq!(decimal_point, true);
    }
}