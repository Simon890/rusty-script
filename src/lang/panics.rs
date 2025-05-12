pub fn unexpected_eof(pos: &u32) -> ! {
    panic!("Unexpected end of input at position {}", pos);
}