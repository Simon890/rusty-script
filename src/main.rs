mod lang;
use lang::tokenizer;

fn main() {
    let mut tokenizer = tokenizer::Tokenizer::new("hola como estas true false 16 5 \"un string literal\"");
    println!("{:#?}", tokenizer.tokenize())
}
