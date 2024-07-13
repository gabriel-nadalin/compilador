use crate::{
    lexico::{
        Lexico,
        token::{Token, TipoToken}
    },
    sintatico::arvore_sintatica::NoAST
};

use std::fs::File;
use std::io::Write;

const BUFFER_SIZE: usize = 10;

pub struct Parser {
    buffer_tokens: Vec<Token>,
    lex: Lexico,
    end: bool,
    file_out: File,
}

impl Parser {

    /// retorna instancia de parser
    pub fn new(lex: Lexico, file_out: &str) -> Self {
        let buffer_tokens = vec![];
        let file_out = File::create(file_out).unwrap();
        let mut parser = Self {
            buffer_tokens,
            lex,
            file_out,
            end: false,
        };
        parser.read_token();
        parser
    }

    /// recarrega buffer, le proximo token e retorna token anterior
    fn read_token(&mut self) -> Token {

        // remove token anterior
        let anterior = if self.buffer_tokens.len() > 0 {
            self.buffer_tokens.remove(0)
        } else {
            Token::new(TipoToken::Vazio, "".to_string(), self.lex.line())
        };

        // recarrega buffer
        while self.buffer_tokens.len() < BUFFER_SIZE && !self.end {
            let next = self.lex.next_token();
            if next.tipo() == TipoToken::Fim {
                self.end = true;
            }
            self.buffer_tokens.push(next);
        }

        println!("Lido: {}", &self.lookahead(1).tipo_string());
        anterior
    }

    /// avalia se `tipo` corresponde ao tipo do proximo token\
    /// retorna proximo token ou erro sintatico caso contrario
    fn match_(&mut self, tipo: TipoToken) -> Result<Token, NoAST> {
        if self.lookahead(1).tipo() == tipo {
            println!("Match: {}", &self.lookahead(1).tipo_string());
            Ok(self.read_token())
        } else {
            Err(self.erro_sintatico())
        }
    }

    /// retorna uma copia do `k`esimo token do buffer, sem altera-lo
    fn lookahead(&mut self, k: usize) -> Token {
        let len = self.buffer_tokens.len();
        if len == 0 {
            return Token::new(TipoToken::Vazio, "".to_string(), self.lex.line());
        }
        if k - 1 >= len {
            return self.buffer_tokens[len - 1].copy()
        }
        self.buffer_tokens[k - 1].copy()
    }

    /// interrompe execucao e imprime mensagem de erro constando linha e lexema atuais
    fn erro_sintatico(&mut self) -> NoAST {
        let linha = self.buffer_tokens[0].linha();
        let lexema = self.buffer_tokens[0].lexema();
        let mensagem = if self.buffer_tokens[0].tipo() == TipoToken::Erro {
            lexema + "Fim da compilacao\n"
        } else {
            "Linha ".to_string() + &linha.to_string() + ": erro sintatico proximo a " + &lexema + "\nFim da compilacao\n"
        };

        // self.file_out.write_all(mensagem.as_bytes()).unwrap();
        // panic!()

        NoAST::Erro { mensagem }
    }

    // as funcoes a seguir representam as leis de formacao da gramatica da linguagem LA

    /// inicia a analise sintatica\
    /// programa : declaracoes 'algoritmo' corpo 'fim_algoritmo'
    pub fn programa(&mut self) -> NoAST {
        let declaracoes = Box::new(self.declaracoes());
        match self.match_(TipoToken::PCalgoritmo) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let corpo = Box::new(self.corpo());
        match self.match_(TipoToken::PCfimAlgoritmo) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        match self.match_(TipoToken::Fim) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        NoAST::Programa { declaracoes, corpo }
    }

    /// declaracoes : declaracao declaracoes | <<vazio>>
    fn declaracoes(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCdeclare
            | TipoToken::PCconstante
            | TipoToken::PCtipo
            | TipoToken::PCprocedimento
            | TipoToken::PCfuncao  =>  {
                let declaracao = Box::new(self.declaracao());
                let declaracoes = Box::new(self.declaracoes());
                NoAST::Declaracoes { declaracao, declaracoes }
            }
            _ => NoAST::Vazio
        }
    }

    /// declaracao : declaracao_local | declaracao_global
    fn declaracao(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCdeclare
            | TipoToken::PCconstante
            | TipoToken::PCtipo => self.declaracao_local(),
            TipoToken::PCprocedimento
            | TipoToken::PCfuncao => self.declaracao_global(),
            _ => self.erro_sintatico()
        }
    }

    /// declaracao_local : 'declare' variavel
    ///     | 'tipo' IDENT ':' tipo
    ///     | 'constante' IDENT ':' tipo_basico '=' valor_constante
    fn declaracao_local(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCdeclare => {
                match self.match_(TipoToken::PCdeclare) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let variavel = Box::new(self.variavel());
                NoAST::DeclaracaoVariavel { variavel }
            }
            TipoToken::PCtipo => {
                match self.match_(TipoToken::PCtipo) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => Box::new(NoAST::Ident(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                match self.match_(TipoToken::Delim) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let tipo = Box::new(self.tipo());
                NoAST::DeclaracaoTipo { ident, tipo }
            }
            TipoToken::PCconstante => {
                match self.match_(TipoToken::PCconstante) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => Box::new(NoAST::Ident(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                match self.match_(TipoToken::Delim) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let tipo_basico = Box::new(self.tipo_basico());
                match self.match_(TipoToken::OpRelIgual) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let valor_constante = Box::new(self.valor_constante());
                NoAST::DeclaracaoConstante { ident, tipo_basico, valor_constante }
            }
            _ => self.erro_sintatico()
        }
    }

    /// variavel : identificador identificadores ':' tipo
    fn variavel(&mut self) -> NoAST {
        let identificador = Box::new(self.identificador());
        let identificadores = Box::new(self.identificadores());
        match self.match_(TipoToken::Delim) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let tipo = Box::new(self.tipo());
        NoAST::Variavel { identificador, identificadores, tipo }
    }

    /// identificador : IDENT identificador2 dimensao
    fn identificador(&mut self) -> NoAST {
        let ident = match self.match_(TipoToken::Ident) {
            Ok(token) => Box::new(NoAST::Ident(token)),
            Err(erro_sintatico) => return erro_sintatico
        };
        let identificador2 = Box::new(self.identificador2());
        let dimensao = Box::new(self.dimensao());
        NoAST::Identificador { ident, identificador2, dimensao }
    }

    /// identificador2 : '.' IDENT identificador2 | <<vazio>>
    fn identificador2(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::Ponto => {
                match self.match_(TipoToken::Ponto) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => Box::new(NoAST::Ident(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                let identificador2 = Box::new(self.identificador2());
                NoAST::Identificador2 { ident, identificador2 }
            }
            _ => NoAST::Vazio
        }
    }

    /// identificadores: ',' identificador identificadores | <<vazio>>
    fn identificadores(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                match self.match_(TipoToken::Virgula) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let identificador = Box::new(self.identificador());
                let identificadores = Box::new(self.identificadores());
                NoAST::Identificadores { identificador, identificadores }
            }
            _ => NoAST::Vazio
        }
    }

    /// dimensao : '[' exp_aritmetica ']' dimensao | <<vazio>>
    fn dimensao(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::AbreCol => {
                match self.match_(TipoToken::AbreCol) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let exp_aritmetica = Box::new(self.exp_aritmetica());
                match self.match_(TipoToken::FechaCol) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let dimensao = Box::new(self.dimensao());
                NoAST::Dimensao { exp_aritmetica, dimensao }
            }
            _ => NoAST::Vazio
        }
    }

    /// tipo : registro | tipo_estendido
    fn tipo(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCregistro => self.registro(),
            TipoToken::Circunflexo
            | TipoToken::PCliteral
            | TipoToken::PCinteiro
            | TipoToken::PCreal
            | TipoToken::PClogico => self.tipo_estendido(),
            _ => self.erro_sintatico()
        }
    }

    /// tipo_basico : 'literal' | 'inteiro' | 'real' | 'logico'
    fn tipo_basico(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::PCliteral => match self.match_(TipoToken::PCliteral) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PCinteiro => match self.match_(TipoToken::PCinteiro) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PCreal => match self.match_(TipoToken::PCreal) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PClogico => match self.match_(TipoToken::PClogico) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };
        NoAST::TipoBasico(token)
    }

    /// tipo_basico_ident : tipo_basico | IDENT
    fn tipo_basico_ident(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCliteral
            | TipoToken::PCinteiro
            | TipoToken::PCreal
            | TipoToken::PClogico => self.tipo_basico(),
            TipoToken::Ident => {
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => token,
                    Err(erro_sintatico) => return erro_sintatico
                };
                NoAST::Ident(ident)
            }
            _ => self.erro_sintatico()
        }
    }

    /// tipo_estendido : circunflexo tipo_basico_ident
    fn tipo_estendido(&mut self) -> NoAST {
        let circunflexo = Box::new(self.circunflexo());
        let tipo_basico_ident = Box::new(self.tipo_basico_ident());
        NoAST::TipoExtendido { circunflexo, tipo_basico_ident }
    }

    /// circunflexo: '^' | <<vazio>>
    fn circunflexo(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::Circunflexo => match self.match_(TipoToken::Circunflexo) {
                Ok(_) => NoAST::Circunflexo,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => NoAST::Vazio
        }
    }

    /// valor_constante : CADEIA | NUM_INT | NUM_REAL | 'verdadeiro' | 'falso'
    fn valor_constante(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::Cadeia => match self.match_(TipoToken::Cadeia) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::NumInt => match self.match_(TipoToken::NumInt) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::NumReal => match self.match_(TipoToken::NumReal) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PCverdadeiro => match self.match_(TipoToken::PCverdadeiro) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PCfalso => match self.match_(TipoToken::PCfalso) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };
        NoAST::ValorConstante(token)
    }

    /// registro : 'registro' variaveis 'fim_Registro'
    fn registro(&mut self) -> NoAST {
        match self.match_(TipoToken::PCregistro) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let variaveis = Box::new(self.variaveis());
        match self.match_(TipoToken::PCfimRegistro) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        NoAST::Registro { variaveis }
    }
    
    /// variaveis : variavel variaveis | <<vazio>>
    fn variaveis(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::Ident => {
                let variavel = Box::new(self.variavel());
                let variaveis = Box::new(self.variaveis());
                NoAST::Variaveis { variavel, variaveis }
            },
            _ => NoAST::Vazio
        }
    }
    
    /// declaracao_global : 'procedimento' IDENT '(' parametros ')' declaracoes_locais cmds 'fim_procedimento'
    ///     | 'funcao' IDENT '(' parametros ')' ':' tipo_estendido declaracoes_locais cmds 'fim_funcao'
    fn declaracao_global(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCprocedimento => {
                match self.match_(TipoToken::PCprocedimento) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => Box::new(NoAST::Ident(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                match self.match_(TipoToken::AbrePar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let parametros = Box::new(self.parametros());
                match self.match_(TipoToken::FechaPar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let declaracoes_locais = Box::new(self.declaracoes_locais());
                let cmds = Box::new(self.cmds());
                match self.match_(TipoToken::PCfimProcedimento) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                NoAST::DeclaracaoProcedimento { ident, parametros, declaracoes_locais, cmds }
            }
            TipoToken::PCfuncao => {
                match self.match_(TipoToken::PCfuncao) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => Box::new(NoAST::Ident(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                match self.match_(TipoToken::AbrePar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let parametros = Box::new(self.parametros());
                match self.match_(TipoToken::FechaPar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                match self.match_(TipoToken::Delim) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let tipo_estendido = Box::new(self.tipo_estendido());
                let declaracoes_locais = Box::new(self.declaracoes_locais());
                let cmds = Box::new(self.cmds());
                match self.match_(TipoToken::PCfimFuncao) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                NoAST::DeclaracaoFuncao { ident, parametros, tipo_estendido, declaracoes_locais, cmds }
            }
            _ => self.erro_sintatico()
        }
    }

    /// declaracoes_locais : declaracao_local declaracoes_locais | <<vazio>>
    fn declaracoes_locais(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCdeclare
            | TipoToken::PCconstante
            | TipoToken::PCtipo => {
                let declaracao_local = Box::new(self.declaracao_local());
                let declaracoes_locais = Box::new(self.declaracoes_locais());
                NoAST::DeclaracoesLocais { declaracao_local, declaracoes_locais }
            }
            _ => NoAST::Vazio
        }
    }

    /// parametro : var identificador identificadores ':' tipo_estendido
    fn parametro(&mut self) -> NoAST {
        let var = Box::new(self.var());
        let identificador = Box::new(self.identificador());
        let identificadores = Box::new(self.identificadores());
        match self.match_(TipoToken::Delim) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let tipo_estendido = Box::new(self.tipo_estendido());
        NoAST::Parametro { var, identificador, identificadores, tipo_estendido }
    }

    /// parametros : parametro parametros2 | <<vazio>>
    fn parametros(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCvar
            | TipoToken::Ident => {
                let parametro = Box::new(self.parametro());
                let parametros2 = Box::new(self.parametros2());
                NoAST::Parametros { parametro, parametros2 }
            }
            _ => NoAST::Vazio
        }
    }

    /// parametros2 : ',' parametro parametros2 | <<vazio>>
    fn parametros2(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                match self.match_(TipoToken::Virgula) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let parametro = Box::new(self.parametro());
                let parametros2 = Box::new(self.parametros2());
                NoAST::Parametros2 { parametro, parametros2 }
            }
            _ => NoAST::Vazio
        }
    }

    /// var : 'var' | <<vazio>>
    fn var(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCvar => match self.match_(TipoToken::PCvar) {
                Ok(_) => NoAST::Var,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => NoAST::Vazio
        }
    }

    /// corpo : declaracoes_locais cmds
    fn corpo(&mut self) -> NoAST {
        let declaracoes_locais = Box::new(self.declaracoes_locais());
        let cmds = Box::new(self.cmds());
        NoAST::Corpo { declaracoes_locais, cmds }
    }

    /// cmd : cmdLeia | cmdEscreva | cmdSe | cmdCaso | cmdPara| cmdEnquanto
    ///     | cmdFaca | cmdAtribuicao | cmdChamada | cmdRetorne
    fn cmd(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCleia => self.cmd_leia(),
            TipoToken::PCescreva => self.cmd_escreva(),
            TipoToken::PCse => self.cmd_se(),
            TipoToken::PCcaso => self.cmd_caso(),
            TipoToken::PCpara => self.cmd_para(),
            TipoToken::PCenquanto => self.cmd_enquanto(),
            TipoToken::PCfaca => self.cmd_faca(),
            TipoToken::Circunflexo => self.cmd_atribuicao(),
            TipoToken::Ident => {
                match self.lookahead(2).tipo() {
                    TipoToken::AbrePar => self.cmd_chamada(),
                    _ => self.cmd_atribuicao()
                }
            }
            TipoToken::PCretorne => self.cmd_retorne(),
            _ => self.erro_sintatico()
        }
    }

    /// cmds : cmd cmds | <<vazio>>
    fn cmds(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCleia | TipoToken::PCescreva | TipoToken::PCse | TipoToken::PCcaso
            | TipoToken::PCpara | TipoToken::PCenquanto | TipoToken::PCfaca | TipoToken::Circunflexo
            | TipoToken::Ident | TipoToken::PCretorne => {
                let cmd = Box::new(self.cmd());
                let cmds = Box::new(self.cmds());
                NoAST::CMDs { cmd, cmds }
            }
            _ => NoAST::Vazio
        }
    }

    /// cmdLeia : 'leia' '(' circunflexo identificador cmdLeia2 ')'
    fn cmd_leia(&mut self) -> NoAST {
        match self.match_(TipoToken::PCleia) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        match self.match_(TipoToken::AbrePar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let circunflexo = Box::new(self.circunflexo());
        let identificador = Box::new(self.identificador());
        let cmd_leia2 = Box::new(self.cmd_leia2());
        match self.match_(TipoToken::FechaPar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        NoAST::CMDLeia { circunflexo, identificador, cmd_leia2 }
    }

    /// cmdLeia2 : ',' circunflexo identificador cmdLeia2 | <<vazio>>
    fn cmd_leia2(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                match self.match_(TipoToken::Virgula) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let circunflexo = Box::new(self.circunflexo());
                let identificador = Box::new(self.identificador());
                let cmd_leia2 = Box::new(self.cmd_leia2());
                NoAST::CMDLeia2 { circunflexo, identificador, cmd_leia2 }
            }
            _ => NoAST::Vazio
        }
    }

    /// cmdEscreva : 'escreva' '(' expressao expressoes ')'
    fn cmd_escreva(&mut self) -> NoAST {
        match self.match_(TipoToken::PCescreva) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        match self.match_(TipoToken::AbrePar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let expressao = Box::new(self.expressao());
        let expressoes = Box::new(self.expressoes());
        match self.match_(TipoToken::FechaPar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        NoAST::CMDEscreva { expressao, expressoes }
    }

    /// cmdSe : 'se' expressao 'entao' cmds senao 'fim_se'
    fn cmd_se(&mut self) -> NoAST {
        match self.match_(TipoToken::PCse) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let expressao = Box::new(self.expressao());
        match self.match_(TipoToken::PCentao) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let cmds = Box::new(self.cmds());
        let senao = Box::new(self.senao());
        match self.match_(TipoToken::PCfimSe) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        NoAST::CMDSe { expressao, cmds, senao }
    }

    /// senao : 'senao' cmds | <<vazio>>
    fn senao(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCsenao => {
                match self.match_(TipoToken::PCsenao) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let cmds = Box::new(self.cmds());
                NoAST::Senao { cmds }
            }
            _ => NoAST::Vazio
        }
    }

    /// cmdCaso : 'caso' exp_aritmetica 'seja' selecao senao 'fim_caso'
    fn cmd_caso(&mut self) -> NoAST {
        match self.match_(TipoToken::PCcaso) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let exp_aritmetica = Box::new(self.exp_aritmetica());
        match self.match_(TipoToken::PCseja) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let selecao = Box::new(self.selecao());
        let senao = Box::new(self.senao());
        match self.match_(TipoToken::PCfimCaso) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        NoAST::CMDCaso { exp_aritmetica, selecao, senao }
    }

    /// cmdPara : 'para' IDENT '<-' exp_aritmetica 'ate' exp_aritmetica 'faca' cmds 'fim_para'
    fn cmd_para(&mut self) -> NoAST {
        match self.match_(TipoToken::PCpara) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let ident = match self.match_(TipoToken::Ident) {
            Ok(token) => Box::new(NoAST::Ident(token)),
            Err(erro_sintatico) => return erro_sintatico
        };
        match self.match_(TipoToken::BackArrow) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let exp_aritmetica_1 = Box::new(self.exp_aritmetica());
        match self.match_(TipoToken::PCate) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let exp_aritmetica_2 = Box::new(self.exp_aritmetica());
        match self.match_(TipoToken::PCfaca) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let cmds = Box::new(self.cmds());
        match self.match_(TipoToken::PCfimPara) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        NoAST::CMDPara { ident, exp_aritmetica_1, exp_aritmetica_2, cmds }
    }

    /// cmdEnquanto : 'enquanto' expressao 'faca' cmds 'fim_enquanto'
    fn cmd_enquanto(&mut self) -> NoAST {
        match self.match_(TipoToken::PCenquanto) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let expressao = Box::new(self.expressao());
        match self.match_(TipoToken::PCfaca) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let cmds = Box::new(self.cmds());
        match self.match_(TipoToken::PCfimEnquanto) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        NoAST::CMDEnquanto { expressao, cmds }
    }

    /// cmdFaca : 'faca' cmds 'ate' expressao
    fn cmd_faca(&mut self) -> NoAST {
        match self.match_(TipoToken::PCfaca) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let cmds = Box::new(self.cmds());
        match self.match_(TipoToken::PCate) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let expressao = Box::new(self.expressao());
        NoAST::CMDFaca { cmds, expressao }
    }

    /// cmdAtribuicao : circunflexo identificador '<-' expressao
    fn cmd_atribuicao(&mut self) -> NoAST {
        let circunflexo = Box::new(self.circunflexo());
        let identificador = Box::new(self.identificador());
        match self.match_(TipoToken::BackArrow) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let expressao = Box::new(self.expressao());
        NoAST::CMDAtribuicao { circunflexo, identificador, expressao }
    }

    /// cmdChamada : IDENT '(' expressao expressoes ')'
    fn cmd_chamada(&mut self) -> NoAST {
        let ident = match self.match_(TipoToken::Ident) {
            Ok(token) => Box::new(NoAST::Ident(token)),
            Err(erro_sintatico) => return erro_sintatico
        };
        match self.match_(TipoToken::AbrePar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let expressao = Box::new(self.expressao());
        let expressoes = Box::new(self.expressoes());
        match self.match_(TipoToken::FechaPar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        NoAST::CMDChamada { ident, expressao, expressoes }
    }

    /// cmdRetorne : 'retorne' expressao
    fn cmd_retorne(&mut self) -> NoAST {
        match self.match_(TipoToken::PCretorne) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let expressao = Box::new(self.expressao());
        NoAST::CMDRetorne { expressao }
    }

    /// selecao : item_selecao selecao | <<vazio>>
    fn selecao(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritSub | TipoToken::NumInt => {
                let item_selecao = Box::new(self.item_selecao());
                let selecao = Box::new(self.selecao());
                NoAST::Selecao { item_selecao, selecao }
            }
            _ => NoAST::Vazio
        }
    }

    /// item_selecao : constantes ':' cmds
    fn item_selecao(&mut self) -> NoAST {
        let constantes = Box::new(self.constantes());
        match self.match_(TipoToken::Delim) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        let cmds = Box::new(self.cmds());
        NoAST::ItemSelecao { constantes, cmds }
    }
    
    /// constantes : numero_intervalo numero_intervalos
    fn constantes(&mut self) -> NoAST {
        let numero_intervalo = Box::new(self.numero_intervalo());
        let numero_intervalos = Box::new(self.numero_intervalos());
        NoAST::Constantes { numero_intervalo, numero_intervalos }
    }
    
    /// numero_intervalo : op_unario NUM_INT numero_intervalo2
    fn numero_intervalo(&mut self) -> NoAST {
        let op_unario = Box::new(self.op_unario());
        let num_int = match self.match_(TipoToken::NumInt) {
            Ok(token) => Box::new(NoAST::NumInt(token)),
            Err(erro_sintatico) => return erro_sintatico
        };
        let numero_intervalo2 = Box::new(self.numero_intervalo2());
        NoAST::NumeroIntervalo { op_unario, num_int, numero_intervalo2 }
    }
    
    /// numero_intervalos : ',' numero_intervalo numero_intervalos | <<vazio>>
    fn numero_intervalos(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                match self.match_(TipoToken::Virgula) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let numero_intervalo = Box::new(self.numero_intervalo());
                let numero_intervalos = Box::new(self.numero_intervalos());
                NoAST::NumeroIntervalos { numero_intervalo, numero_intervalos }
            }
            _ => NoAST::Vazio
        }
    }
    
    /// numero_intervalo2 : '..' op_unario NUM_INT | <<vazio>>
    fn numero_intervalo2(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PontoPonto => {
                match self.match_(TipoToken::PontoPonto) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let op_unario = Box::new(self.op_unario());
                let num_int = match self.match_(TipoToken::NumInt) {
                    Ok(token) => Box::new(NoAST::NumInt(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                NoAST::NumeroIntervalo2 { op_unario, num_int }
            }
            _ => NoAST::Vazio
        }
    }
    
    /// op_unario : '-' | <<vazio>>
    fn op_unario(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritSub => {
                match self.match_(TipoToken::OpAritSub) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                NoAST::OpUnario
            }
            _ => NoAST::Vazio
        }
    }
    
    /// exp_aritmetica : termo termos
    fn exp_aritmetica(&mut self) -> NoAST {
        let termo = Box::new(self.termo());
        let termos = Box::new(self.termos());
        NoAST::ExpAritmetica { termo, termos }
    }
    
    /// termo : fator fatores
    fn termo(&mut self) -> NoAST {
        let fator = Box::new(self.fator());
        let fatores = Box::new(self.fatores());
        NoAST::Termo { fator, fatores }
    }
    
    /// termos : op1 termo termos | <<vazio>>
    fn termos(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritSoma | TipoToken::OpAritSub => {
                let op1 = Box::new(self.op1());
                let termo = Box::new(self.termo());
                let termos = Box::new(self.termos());
                NoAST::Termos { op1, termo, termos }
            }
            _ => NoAST::Vazio
        }
    }
    
    /// fator : parcela parcelas
    fn fator(&mut self) -> NoAST {
        let parcela = Box::new(self.parcela());
        let parcelas = Box::new(self.parcelas());
        NoAST::Fator { parcela, parcelas }
    }
    
    /// fatores : op2 fator fatores | <<vazio>>
    fn fatores(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritMult | TipoToken::OpAritDiv => {
                let op2 = Box::new(self.op2());
                let fator = Box::new(self.fator());
                let fatores = Box::new(self.fatores());
                NoAST::Fatores { op2, fator, fatores }
            }
            _ => NoAST::Vazio
        }
    }
    
    /// op1 : '+' | '-'
    fn op1(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::OpAritSoma => match self.match_(TipoToken::OpAritSoma) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpAritSub => match self.match_(TipoToken::OpAritSub) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };
        NoAST::Op1(token)
    }
    
    /// op2 : '*' | '/'
    fn op2(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::OpAritMult => match self.match_(TipoToken::OpAritMult) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpAritDiv => match self.match_(TipoToken::OpAritDiv) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };
        NoAST::Op2(token)
    }
    
    /// op3 : '%'
    fn op3(&mut self) -> NoAST {
        match self.match_(TipoToken::Porcento) {
            Ok(_) => NoAST::Op3,
            Err(erro_sintatico) => erro_sintatico
        }
    }
    
    /// parcela : op_unario parcela_unario | parcela_nao_unario
    fn parcela(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritSub
            | TipoToken::Circunflexo
            | TipoToken::Ident
            | TipoToken::NumInt
            | TipoToken::NumReal
            | TipoToken::AbrePar => {
                let op_unario = Box::new(self.op_unario());
                let parcela_unario = Box::new(self.parcela_unario());
                NoAST::Parcela { op_unario, parcela_unario }
            }
            TipoToken::EComercial
            | TipoToken::Cadeia => self.parcela_nao_unario(),
            _ => self.erro_sintatico()
        }
    }
    
    /// parcelas : op3 parcela parcelas | <<vazio>>
    fn parcelas(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::Porcento => {
                let op3 = Box::new(self.op3());
                let parcela = Box::new(self.parcela());
                let parcelas = Box::new(self.parcelas());
                NoAST::Parcelas { op3, parcela, parcelas }
            }
            _ => NoAST::Vazio
        }
    }
    
    /// parcela_unario : circunflexo identificador
    ///     | IDENT '(' expressao expressoes ')'
    ///     | '(' expressao ')'
    ///     | NUM_INT
    ///     | NUM_REAL
    fn parcela_unario(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::Circunflexo => {
                let circunflexo = Box::new(self.circunflexo());
                let identificador = Box::new(self.identificador());
                NoAST::ParcelaUnario1 { circunflexo, identificador }
            }
            TipoToken::Ident => {
                match self.lookahead(2).tipo() {
                    TipoToken::AbrePar => {
                        let ident = match self.match_(TipoToken::Ident) {
                            Ok(token) => Box::new(NoAST::Ident(token)),
                            Err(erro_sintatico) => return erro_sintatico
                        };
                        match self.match_(TipoToken::AbrePar) {
                            Ok(_) => {},
                            Err(erro_sintatico) => return erro_sintatico
                        }
                        let expressao = Box::new(self.expressao());
                        let expressoes = Box::new(self.expressoes());
                        match self.match_(TipoToken::FechaPar) {
                            Ok(_) => {},
                            Err(erro_sintatico) => return erro_sintatico
                        }
                        NoAST::ParcelaUnario2 { ident, expressao, expressoes }
                    }
                    _ => self.identificador()
                }
            }
            TipoToken::AbrePar => {
                match self.match_(TipoToken::AbrePar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let expressao = Box::new(self.expressao());
                match self.match_(TipoToken::FechaPar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                NoAST::ParcelaUnario3 { expressao }
            }
            TipoToken::NumInt => match self.match_(TipoToken::NumInt) {
                Ok(token) => NoAST::NumInt(token),
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::NumReal => match self.match_(TipoToken::NumReal) {
                Ok(token) => NoAST::NumReal(token),
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => self.erro_sintatico()
        }
    }
    
    /// parcela_nao_unario : '&' identificador | CADEIA
    fn parcela_nao_unario(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::EComercial => {
                match self.match_(TipoToken::EComercial) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let identificador = Box::new(self.identificador());
                NoAST::ParcelaNaoUnario { identificador }
            }
            TipoToken::Cadeia => match self.match_(TipoToken::Cadeia) {
                Ok(token) => NoAST::Cadeia(token),
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => self.erro_sintatico()
        }
    }
    
    /// exp_relacional : exp_aritmetica exp_relacional2
    fn exp_relacional(&mut self) -> NoAST {
        let exp_aritmetica = Box::new(self.exp_aritmetica());
        let exp_relacional2 = Box::new(self.exp_relacional2());
        NoAST::ExpRelacional { exp_aritmetica, exp_relacional2 }
    }
    
    /// exp_relacional2 : op_relacional exp_aritmetica | <<vazio>>
    fn exp_relacional2(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::OpRelIgual
            | TipoToken::OpRelDif
            | TipoToken::OpRelMaiorIgual
            | TipoToken::OpRelMenorIgual
            | TipoToken::OpRelMaior
            | TipoToken::OpRelMenor => {
                let op_relacional = Box::new(self.op_relacional());
                let exp_aritmetica = Box::new(self.exp_aritmetica());
                NoAST::ExpRelacional2 { op_relacional, exp_aritmetica }
            }
            _ => NoAST::Vazio
        }
    }
    
    /// op_relacional : '=' | '<>' | '>=' | '<=' | '>' | '<'
    fn op_relacional(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::OpRelIgual => match self.match_(TipoToken::OpRelIgual) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpRelDif => match self.match_(TipoToken::OpRelDif) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpRelMaiorIgual => match self.match_(TipoToken::OpRelMaiorIgual) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpRelMenorIgual => match self.match_(TipoToken::OpRelMenorIgual) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpRelMaior => match self.match_(TipoToken::OpRelMaior) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpRelMenor => match self.match_(TipoToken::OpRelMenor) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };
        NoAST::OpRelacional(token)
    }
    
    /// expressao : termo_logico termos_logicos
    fn expressao(&mut self) -> NoAST {
        let termo_logico = Box::new(self.termo_logico());
        let termos_logicos = Box::new(self.termos_logicos());
        NoAST::Expressao { termo_logico, termos_logicos }
    }
    
    /// expressoes : ',' expressao expressoes | <<vazio>>
    fn expressoes(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                match self.match_(TipoToken::Virgula) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                let expressao = Box::new(self.expressao());
                let expressoes = Box::new(self.expressoes());
                NoAST::Expressoes { expressao, expressoes }
            }
            _ => NoAST::Vazio
        }
    }
    
    /// termo_logico : fator_logico fatores_logicos
    fn termo_logico(&mut self) -> NoAST {
        let fator_logico = Box::new(self.fator_logico());
        let fatores_logicos = Box::new(self.fatores_logicos());
        NoAST::TermoLogico { fator_logico, fatores_logicos }
    }
    
    /// termos_logicos : op_logico_1 termo_logico termos_logicos | <<vazio>>
    fn termos_logicos(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCou => {
                let op_logico_1 = Box::new(self.op_logico_1());
                let termo_logico = Box::new(self.termo_logico());
                let termos_logicos = Box::new(self.termos_logicos());
                NoAST::TermosLogicos { op_logico_1, termo_logico, termos_logicos }
            }
            _ => NoAST::Vazio
        }
    }
    
    /// fator_logico : nao parcela_logica
    fn fator_logico(&mut self) -> NoAST {
        let nao = Box::new(self.nao());
        let parcela_logica = Box::new(self.parcela_logica());
        NoAST::FatorLogico { nao, parcela_logica }
    }
    
    /// fatores_logicos : op_logico_2 fator_logico fatores_logicos | <<vazio>>
    fn fatores_logicos(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCe => {
                let op_logico_2 = Box::new(self.op_logico_2());
                let fator_logico = Box::new(self.fator_logico());
                let fatores_logicos = Box::new(self.fatores_logicos());
                NoAST::FatoresLogicos { op_logico_2, fator_logico, fatores_logicos }
            }
            _ => NoAST::Vazio
        }
    }
    
    /// nao : 'nao' | <<vazio>>
    fn nao(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCnao => match self.match_(TipoToken::PCnao) {
                Ok(_) => NoAST::Nao,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => NoAST::Vazio
        }
    }
    
    /// parcela_logica : constante_logica | exp_relacional
    fn parcela_logica(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCverdadeiro | TipoToken::PCfalso => self.constante_logica(),
            TipoToken::OpAritSub
            | TipoToken::Circunflexo
            | TipoToken::Ident
            | TipoToken::NumInt
            | TipoToken::NumReal
            | TipoToken::AbrePar
            | TipoToken::EComercial
            | TipoToken::Cadeia  => self.exp_relacional(),
            _ => self.erro_sintatico()
        }
    }
    
    /// constante_logica : 'verdadeiro' | 'falso'
    fn constante_logica(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::PCverdadeiro => match self.match_(TipoToken::PCverdadeiro) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PCfalso => match self.match_(TipoToken::PCfalso) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };
        NoAST::ConstanteLogica(token)
    }
    
    /// op_logico_1 : 'ou'
    fn op_logico_1(&mut self) -> NoAST {
        match self.match_(TipoToken::PCou) {
            Ok(_) => NoAST::OpLogico1,
            Err(erro_sintatico) => return erro_sintatico
        }
    }
    
    /// op_logico_2 : 'e'
    fn op_logico_2(&mut self) -> NoAST {
        match self.match_(TipoToken::PCe) {
            Ok(_) => NoAST::OpLogico2,
            Err(erro_sintatico) => return erro_sintatico
        }
    }
}