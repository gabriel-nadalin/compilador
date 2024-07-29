use std::env;
use std::fs::File;
use std::io::Write;

use compilador::{
    lexico::Lexico,
    semantico::{visitor::Visitor, Semantico},
    sintatico::parser::Parser
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_out = File::create(&args[2]).unwrap();
    let lex = Lexico::new(&args[1]);
    let mut parser = Parser::new(lex);
    let ast = parser.programa();
    if ast.is_erro() {
        let mensagem = ast.get_erro().unwrap();
        file_out.write_all(mensagem.as_bytes()).unwrap();
    }
    let mut semantico = Semantico::new();
    semantico.traverse(&ast);
    for erro in semantico.get_erros() {
        file_out.write_all(erro.as_bytes()).unwrap();
    }
    file_out.write_all("Fim da compilacao\n".as_bytes()).unwrap();
}