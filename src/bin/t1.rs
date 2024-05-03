use std::env;
use std::fs::File;
use std::io::Write;
use compilador::lexico::{token, Lexico};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lex = Lexico::new(&args[1]);
    let mut file_out = File::create(&args[2]).unwrap();
    let mut next = lex.next_token();

    while next.tipo() != token::TipoToken::Fim {
        if next.tipo() == token::TipoToken::Erro {
            file_out.write_all(next.lexema().as_bytes()).unwrap();
            break;
        }
        file_out.write_all((next.to_string() + "\n").as_bytes()).unwrap();
        next = lex.next_token();
    }
}