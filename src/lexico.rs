mod file_reader;
pub mod token;

use file_reader::FileReader;
use token::{Token, TipoToken};

pub struct Lexico {
    reader: FileReader,
    line: u32
}

impl Lexico {

    /// retorna instancia de analisador lexico
    pub fn new(file: &str) -> Self {
        Lexico {
            reader: FileReader::new(file),
            line: 1
        }
    }

    /// retorna linha atual
    pub fn line(&mut self) -> u32 {
        self.line
    }

    /// debug: exibe conteúdo do buffer
    pub fn print_buffer(&mut self) {
        self.reader.print_buffer();
    }

    /// retorna proximo token ou token de erro no caso de erro lexico
    pub fn next_token(&mut self) -> Token {
        let mut next = self.whitespace_and_comments();
        self.reader.confirm();
        if next.is_some() {
            return next.unwrap();
        }
        
        next = self.end();
        if next.is_none() {
            self.reader.reset();
        } else {
            self.reader.confirm();
            return next.unwrap();
        }

        next = self.keywords();
        if next.is_none() {
            self.reader.reset();
        } else {
            self.reader.confirm();
            return next.unwrap();
        }

        next = self.identifier();
        if next.is_none() {
            self.reader.reset();
        } else {
            self.reader.confirm();
            return next.unwrap();
        }

        next = self.number();
        if next.is_none() {
            self.reader.reset();
        } else {
            self.reader.confirm();
            return next.unwrap();
        }

        next = self.op_arit();
        if next.is_none() {
            self.reader.reset();
        } else {
            self.reader.confirm();
            return next.unwrap();
        }

        next = self.op_rel();
        if next.is_none() {
            self.reader.reset();
        } else {
            self.reader.confirm();
            return next.unwrap();
        }

        next = self.special_char();
        if next.is_none() {
            self.reader.reset();
        } else {
            self.reader.confirm();
            return next.unwrap();
        }

        next = self.str_literal();
        if next.is_none() {
            self.reader.reset();
        } else {
            self.reader.confirm();
            return next.unwrap();
        }
        
        let message = "Linha ".to_string() + &self.line.to_string() + ": " + &self.reader.next_char().to_string() + " - simbolo nao identificado\n";
        return Token::new(TipoToken::Erro, message, self.line)
    }

    fn op_arit(&mut self) -> Option<Token> {
        let c = self.reader.next_char();
        match c {
            '*' => return Some(Token::new(TipoToken::OpAritMult, self.reader.get_lexeme(), self.line)),
            '/' => return Some(Token::new(TipoToken::OpAritDiv, self.reader.get_lexeme(), self.line)),
            '+' => return Some(Token::new(TipoToken::OpAritSoma, self.reader.get_lexeme(), self.line)),
            '-' => return Some(Token::new(TipoToken::OpAritSub, self.reader.get_lexeme(), self.line)),
            _ => return None
        }
    }

    fn op_rel(&mut self) -> Option<Token> {
        let mut c = self.reader.next_char();
        match c {
            '<' => {
                c = self.reader.next_char();
                match c {
                    '>' => return Some(Token::new(TipoToken::OpRelDif, self.reader.get_lexeme(), self.line)),
                    '=' => return Some(Token::new(TipoToken::OpRelMenorIgual, self.reader.get_lexeme(), self.line)),
                    '-' => {
                        self.reader.decrement();
                        self.reader.decrement();
                        return None;
                    }
                    _ => {
                        self.reader.decrement();
                        return Some(Token::new(TipoToken::OpRelMenor, self.reader.get_lexeme(), self.line));
                    }
                }
            }
            '=' => return Some(Token::new(TipoToken::OpRelIgual, self.reader.get_lexeme(), self.line)),
            '>' => {
                c = self.reader.next_char();
                match c {
                    '=' => return Some(Token::new(TipoToken::OpRelMaiorIgual, self.reader.get_lexeme(), self.line)),
                    _ => {
                        self.reader.decrement();
                        return Some(Token::new(TipoToken::OpRelMaior, self.reader.get_lexeme(), self.line));
                    }
                }
            }
            _ => return None
        }
    }

    fn number(&mut self) -> Option<Token> {
        let mut state = 1;
        loop {
            let mut c = self.reader.next_char();
            if state == 1 {
                if c.is_digit(10) {
                    state = 2;
                } else {
                    return None
                }
            } else if state == 2 {
                if c == '.' {
                    c = self.reader.next_char();
                    if c.is_digit(10) {
                        state = 3;
                    } else if c == '.' {
                        self.reader.decrement();
                        self.reader.decrement();
                        return Some(Token::new(TipoToken::NumInt, self.reader.get_lexeme(), self.line));
                    } else {
                        return None
                    }
                } else if !(c.is_digit(10)) {
                    self.reader.decrement();
                    return Some(Token::new(TipoToken::NumInt, self.reader.get_lexeme(), self.line));
                }
            } else if state == 3 {
                if !(c.is_digit(10)) {
                    self.reader.decrement();
                    return Some(Token::new(TipoToken::NumReal, self.reader.get_lexeme(), self.line));
                }
            }
        }
    }

    fn identifier(&mut self) -> Option<Token> {
        let mut state = 1;
        loop {
            let c = self.reader.next_char();
            if state == 1 {
                if c.is_alphabetic() {
                    state = 2;
                } else {
                    return None
                }
            } else if state == 2 {
                if !(c.is_alphanumeric() || c == '_') {
                    self.reader.decrement();
                    return Some(Token::new(TipoToken::Ident, self.reader.get_lexeme(), self.line));
                }
            }
        }
    }

    fn str_literal(&mut self) -> Option<Token> {
        let mut state = 1;
        loop {
            let c = self.reader.next_char();
            if state == 1 {
                if c == '\"' {
                    state = 2;
                } else {
                    return None
                }
            } else if state == 2 {
                if c == '\n' {
                    let message = "Linha ".to_string() + &self.line.to_string() + ": cadeia literal nao fechada\n";
                    return Some(Token::new(TipoToken::Erro, message, self.line))
                }
                if c == '\"' {
                    return Some(Token::new(TipoToken::Cadeia, self.reader.get_lexeme(), self.line));
                } else if c == '\\' {
                    state = 3;
                }
            } else if state == 3 {
                if c == '\n' {
                    let message = "Linha ".to_string() + &self.line.to_string() + ": cadeia literal nao fechada\n";
                    return Some(Token::new(TipoToken::Erro, message, self.line))
                } else {
                    state = 2;
                }
            }
        }
    }

    fn whitespace_and_comments(&mut self) -> Option<Token> {
        let mut state = 1;
        loop {
            let c = self.reader.next_char();
            if state == 1 {
                if c.is_whitespace() || c == ' ' {
                    state = 2;
                } else if c == '{' {
                    state = 3;
                } else {
                    self.reader.decrement();
                    return None
                }
            } else if state == 2 {
                if c == '{' {
                    state = 3;
                } else if !(c.is_whitespace() || c == ' ') {
                    self.reader.decrement();
                    return None
                }
            } else if state == 3 {

                if c == '\n' {
                    let message = "Linha ".to_string() + &self.line.to_string() + ": comentario nao fechado\n";
                    return Some(Token::new(TipoToken::Erro, message, self.line))
                } else if c == '}' {
                    state = 1
                }
            }
            if c == '\n' {
                self.line += 1;
            }
        }
    }

    fn special_char(&mut self) -> Option<Token> {
        let mut c = self.reader.next_char();
        match c {
            '(' => return Some(Token::new(TipoToken::AbrePar, self.reader.get_lexeme(), self.line)),
            ')' => return Some(Token::new(TipoToken::FechaPar, self.reader.get_lexeme(), self.line)),
            '[' => return Some(Token::new(TipoToken::AbreCol, self.reader.get_lexeme(), self.line)),
            ']' => return Some(Token::new(TipoToken::FechaCol, self.reader.get_lexeme(), self.line)),
            ',' => return Some(Token::new(TipoToken::Virgula, self.reader.get_lexeme(), self.line)),
            '%' => return Some(Token::new(TipoToken::Porcento, self.reader.get_lexeme(), self.line)),
            ':' => return Some(Token::new(TipoToken::Delim, self.reader.get_lexeme(), self.line)),
            '^' => return Some(Token::new(TipoToken::Circunflexo, self.reader.get_lexeme(), self.line)),
            '&' => return Some(Token::new(TipoToken::EComercial, self.reader.get_lexeme(), self.line)),
            '.' => {
                c = self.reader.next_char();
                if c == '.' {
                    return Some(Token::new(TipoToken::PontoPonto, self.reader.get_lexeme(), self.line));
                } else {
                    self.reader.decrement();
                    return Some(Token::new(TipoToken::Ponto, self.reader.get_lexeme(), self.line));
                }
            }
            '<' => {
                c = self.reader.next_char();
                if c == '-' {
                    return Some(Token::new(TipoToken::BackArrow, self.reader.get_lexeme(), self.line));
                } else {
                    self.reader.decrement();
                    return None
                }
            }
            _ => return None
        }
    }

    fn keywords(&mut self) -> Option<Token> {
        loop {
            let c = self.reader.next_char();
            if !(c.is_alphabetic() || c == '_') {
                self.reader.decrement();
                let lexeme: &str = &self.reader.get_lexeme();
                match lexeme {
                    "algoritmo" => return Some(Token::new(TipoToken::PCalgoritmo, self.reader.get_lexeme(), self.line)),
                    "declare" => return Some(Token::new(TipoToken::PCdeclare, self.reader.get_lexeme(), self.line)),
                    "literal" => return Some(Token::new(TipoToken::PCliteral, self.reader.get_lexeme(), self.line)), 
                    "inteiro" => return Some(Token::new(TipoToken::PCinteiro, self.reader.get_lexeme(), self.line)), 
                    "leia" => return Some(Token::new(TipoToken::PCleia, self.reader.get_lexeme(), self.line)), 
                    "escreva" => return Some(Token::new(TipoToken::PCescreva, self.reader.get_lexeme(), self.line)), 
                    "fim_algoritmo" => return Some(Token::new(TipoToken::PCfimAlgoritmo, self.reader.get_lexeme(), self.line)), 
                    "real" => return Some(Token::new(TipoToken::PCreal, self.reader.get_lexeme(), self.line)), 
                    "logico" => return Some(Token::new(TipoToken::PClogico, self.reader.get_lexeme(), self.line)), 
                    "fim_se" => return Some(Token::new(TipoToken::PCfimSe, self.reader.get_lexeme(), self.line)), 
                    "senao" => return Some(Token::new(TipoToken::PCsenao, self.reader.get_lexeme(), self.line)), 
                    "entao" => return Some(Token::new(TipoToken::PCentao, self.reader.get_lexeme(), self.line)), 
                    "se" => return Some(Token::new(TipoToken::PCse, self.reader.get_lexeme(), self.line)), 
                    "fim_caso" => return Some(Token::new(TipoToken::PCfimCaso, self.reader.get_lexeme(), self.line)), 
                    "para" => return Some(Token::new(TipoToken::PCpara, self.reader.get_lexeme(), self.line)), 
                    "ate" => return Some(Token::new(TipoToken::PCate, self.reader.get_lexeme(), self.line)), 
                    "faca" => return Some(Token::new(TipoToken::PCfaca, self.reader.get_lexeme(), self.line)), 
                    "fim_para" => return Some(Token::new(TipoToken::PCfimPara, self.reader.get_lexeme(), self.line)), 
                    "fim_enquanto" => return Some(Token::new(TipoToken::PCfimEnquanto, self.reader.get_lexeme(), self.line)),
                    "seja" => return Some(Token::new(TipoToken::PCseja, self.reader.get_lexeme(), self.line)), 
                    "caso" => return Some(Token::new(TipoToken::PCcaso, self.reader.get_lexeme(), self.line)), 
                    "enquanto" => return Some(Token::new(TipoToken::PCenquanto, self.reader.get_lexeme(), self.line)), 
                    "registro" => return Some(Token::new(TipoToken::PCregistro, self.reader.get_lexeme(), self.line)), 
                    "fim_registro" => return Some(Token::new(TipoToken::PCfimRegistro, self.reader.get_lexeme(), self.line)), 
                    "tipo" => return Some(Token::new(TipoToken::PCtipo, self.reader.get_lexeme(), self.line)), 
                    "fim_procedimento" => return Some(Token::new(TipoToken::PCfimProcedimento, self.reader.get_lexeme(), self.line)), 
                    "procedimento" => return Some(Token::new(TipoToken::PCprocedimento, self.reader.get_lexeme(), self.line)), 
                    "var" => return Some(Token::new(TipoToken::PCvar, self.reader.get_lexeme(), self.line)), 
                    "funcao" => return Some(Token::new(TipoToken::PCfuncao, self.reader.get_lexeme(), self.line)), 
                    "fim_funcao" => return Some(Token::new(TipoToken::PCfimFuncao, self.reader.get_lexeme(), self.line)), 
                    "retorne" => return Some(Token::new(TipoToken::PCretorne, self.reader.get_lexeme(), self.line)), 
                    "constante" => return Some(Token::new(TipoToken::PCconstante, self.reader.get_lexeme(), self.line)), 
                    "falso" => return Some(Token::new(TipoToken::PCfalso, self.reader.get_lexeme(), self.line)), 
                    "verdadeiro" => return Some(Token::new(TipoToken::PCverdadeiro, self.reader.get_lexeme(), self.line)), 
                    "nao" => return Some(Token::new(TipoToken::PCnao, self.reader.get_lexeme(), self.line)), 
                    "ou" => return Some(Token::new(TipoToken::PCou, self.reader.get_lexeme(), self.line)), 
                    "e" => return Some(Token::new(TipoToken::PCe, self.reader.get_lexeme(), self.line)),
                    &_ => return None
                }
            }
        }
    }

    fn end(&mut self) -> Option<Token> {
        let c = self.reader.next_char();
        if c == '\0' {
            return Some(Token::new(TipoToken::Fim, "EOF".to_string(), self.line))
        }
        return None
    }
}