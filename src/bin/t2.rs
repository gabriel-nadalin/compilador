use std::env;
use compilador::sintatico::parser::Parser;
use compilador::lexico::Lexico;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lex = Lexico::new(&args[1]);
    let mut parser = Parser::new(lex, &args[2]);
    parser.programa();
}