pub fn unexpected_eof(pos: &u32) -> ! {
    panic!("Unexpected end of input at position {}", pos);
}

pub fn unexpected_token(token: &str, pos: &u32) -> ! {
    panic!("Unexpected token '{}' at position {}", token, pos);
}