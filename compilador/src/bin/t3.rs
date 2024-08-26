use std::env;
use std::fs::File;
use std::io::Write;

use compilador::{
    lexico::Lexico,
    sintatico::parser::Parser,
    semantico::{visitor::Visitor, Semantico},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_out = File::create(&args[2]).unwrap();
    let lex = Lexico::new(&args[1]);
    let mut parser = Parser::new(lex);
    let ast = parser.programa();
    let mut semantico = Semantico::new();
    semantico.traverse(&ast);
    for erro in semantico.get_erros() {
        file_out.write_all(erro.as_bytes()).unwrap();
    }
    file_out.write_all("Fim da compilacao\n".as_bytes()).unwrap();
}