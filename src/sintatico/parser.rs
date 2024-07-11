use crate::lexico::token::{Token, TipoToken};
use crate::lexico::Lexico;
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

    /// le proximo token e recarrega o buffer, se necessario
    fn read_token(&mut self) {
        if self.buffer_tokens.len() > 0 {
            self.buffer_tokens.remove(0);
        }
        while self.buffer_tokens.len() < BUFFER_SIZE && !self.end {
            let next = self.lex.next_token();
            if next.tipo() == TipoToken::Fim {
                self.end = true;
            }
            self.buffer_tokens.push(next);
        }
        println!("Lido: {}", &self.lookahead(1).tipo_string());
    }

    /// avalia se `tipo` corresponde ao tipo do proximo token, gerando erro sintatico caso contrario
    fn match_(&mut self, tipo: TipoToken) {
        if self.lookahead(1).tipo() == tipo {
            println!("Match: {}", &self.lookahead(1).tipo_string());
            self.read_token();
        } else {
            self.erro_sintatico();
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
    fn erro_sintatico(&mut self) {
        let linha = self.buffer_tokens[0].linha();
        let lexema = self.buffer_tokens[0].lexema();
        let mensagem = if self.buffer_tokens[0].tipo() == TipoToken::Erro {
            lexema + "Fim da compilacao\n"
        } else {
            "Linha ".to_string() + &linha.to_string() + ": erro sintatico proximo a " + &lexema + "\nFim da compilacao\n"
        };
        self.file_out.write_all(mensagem.as_bytes()).unwrap();
        panic!()
    }

    // as funcoes a seguir representam as leis de formacao da gramatica da linguagem LA

    /// inicia a analise sintatica\
    /// programa : declaracoes 'algoritmo' corpo 'fim_algoritmo'
    pub fn programa(&mut self) {
        self.declaracoes();
        self.match_(TipoToken::PCalgoritmo);
        self.corpo();
        self.match_(TipoToken::PCfimAlgoritmo);
        self.match_(TipoToken::Fim);
    }

    /// declaracoes : decl_local_global declaracoes | <<vazio>>
    fn declaracoes(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCdeclare
            | TipoToken::PCconstante
            | TipoToken::PCtipo
            | TipoToken::PCprocedimento
            | TipoToken::PCfuncao  =>  {
                self.decl_local_global();
                self.declaracoes();
            }
            _ => return
        }
    }

    /// decl_local_global : declaracao_local | declaracao_global
    fn decl_local_global(&mut self) {
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
    ///     | 'constante' IDENT ':' tipo_basico '=' valor_constante
    ///     | 'tipo' IDENT ':' tipo
    fn declaracao_local(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCdeclare => {
                self.match_(TipoToken::PCdeclare);
                self.variavel();
            }
            TipoToken::PCconstante => {
                self.match_(TipoToken::PCconstante);
                self.match_(TipoToken::Ident);
                self.match_(TipoToken::Delim);
                self.tipo_basico();
                self.match_(TipoToken::OpRelIgual);
                self.valor_constante();
            }
            TipoToken::PCtipo => {
                self.match_(TipoToken::PCtipo);
                self.match_(TipoToken::Ident);
                self.match_(TipoToken::Delim);
                self.tipo();
            }
            _ => self.erro_sintatico()
        }
    }

    /// variavel : identificador identificadores ':' tipo
    fn variavel(&mut self) {
        self.identificador();
        self.identificadores();
        self.match_(TipoToken::Delim);
        self.tipo();
    }

    /// identificador : IDENT identificador2 dimensao
    fn identificador(&mut self) {
        self.match_(TipoToken::Ident);
        self.identificador2();
        self.dimensao();
    }

    /// identificador2 : '.' IDENT identificador2 | <<vazio>>
    fn identificador2(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::Ponto => {
                self.match_(TipoToken::Ponto);
                self.match_(TipoToken::Ident);
                self.identificador2();
            }
            _ => return
        }
    }

    /// identificadores: ',' identificador identificadores | <<vazio>>
    fn identificadores(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                self.match_(TipoToken::Virgula);
                self.identificador();
                self.identificadores();
            }
            _ => return
        }
    }

    /// dimensao : '[' exp_aritmetica ']' dimensao | <<vazio>>
    fn dimensao(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::AbreCol => {
                self.match_(TipoToken::AbreCol);
                self.exp_aritmetica();
                self.match_(TipoToken::FechaCol);
                self.dimensao();
            }
            _ => return
        }
    }

    /// tipo : registro | tipo_estendido
    fn tipo(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCregistro => self.registro(),
            TipoToken::Circunflexo
            | TipoToken::PCliteral
            | TipoToken::PCinteiro
            | TipoToken::PCreal
            | TipoToken::PClogico => self.tipo_extendido(),
            _ => self.erro_sintatico()
        }
    }

    /// tipo_basico : 'literal' | 'inteiro' | 'real' | 'logico'
    fn tipo_basico(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCliteral => self.match_(TipoToken::PCliteral),
            TipoToken::PCinteiro => self.match_(TipoToken::PCinteiro),
            TipoToken::PCreal => self.match_(TipoToken::PCreal),
            TipoToken::PClogico => self.match_(TipoToken::PClogico),
            _ => self.erro_sintatico()
        }
    }

    /// tipo_basico_ident : tipo_basico | IDENT
    fn tipo_basico_ident(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCliteral
            | TipoToken::PCinteiro
            | TipoToken::PCreal
            | TipoToken::PClogico => self.tipo_basico(),
            TipoToken::Ident => self.match_(TipoToken::Ident),
            _ => self.erro_sintatico()
        }
    }

    /// tipo_estendido : circunflexo tipo_basico_ident
    fn tipo_extendido(&mut self) {
        self.circunflexo();
        self.tipo_basico_ident();
    }

    /// circunflexo: '^' | <<vazio>>
    fn circunflexo(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::Circunflexo => self.match_(TipoToken::Circunflexo),
            _ => return
        }
    }

    /// valor_constante : CADEIA | NUM_INT | NUM_REAL | 'verdadeiro' | 'falso'
    fn valor_constante(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::Cadeia => self.match_(TipoToken::Cadeia),
            TipoToken::NumInt => self.match_(TipoToken::NumInt),
            TipoToken::NumReal => self.match_(TipoToken::NumReal),
            TipoToken::PCverdadeiro => self.match_(TipoToken::PCverdadeiro),
            TipoToken::PCfalso => self.match_(TipoToken::PCfalso),
            _ => self.erro_sintatico()
        }
    }

    /// registro : 'registro' variaveis 'fim_Registro'
    fn registro(&mut self) {
        self.match_(TipoToken::PCregistro);
        self.variaveis();
        self.match_(TipoToken::PCfimRegistro);
    }
    
    /// variaveis : variavel variaveis | <<vazio>>
    fn variaveis(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::Ident => {
                self.variavel();
                self.variaveis();
            },
            _ => return
        }
    }
    
    /// declaracao_global : 'procedimento' IDENT '(' parametros ')' declaracoes_locais cmds 'fim_procedimento'
    ///     | 'funcao' IDENT '(' parametros ')' ':' tipo_estendido declaracoes_locais cmds 'fim_funcao'
    fn declaracao_global(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCprocedimento => {
                self.match_(TipoToken::PCprocedimento);
                self.match_(TipoToken::Ident);
                self.match_(TipoToken::AbrePar);
                self.parametros();
                self.match_(TipoToken::FechaPar);
                self.declaracoes_locais();
                self.cmds();
                self.match_(TipoToken::PCfimProcedimento);
            }
            TipoToken::PCfuncao => {
                self.match_(TipoToken::PCfuncao);
                self.match_(TipoToken::Ident);
                self.match_(TipoToken::AbrePar);
                self.parametros();
                self.match_(TipoToken::FechaPar);
                self.match_(TipoToken::Delim);
                self.tipo_extendido();
                self.declaracoes_locais();
                self.cmds();
                self.match_(TipoToken::PCfimFuncao);
            }
            _ => self.erro_sintatico()
        }
    }

    /// declaracoes_locais : declaracao_local declaracoes_locais | <<vazio>>
    fn declaracoes_locais(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCdeclare
            | TipoToken::PCconstante
            | TipoToken::PCtipo => {
                self.declaracao_local();
                self.declaracoes_locais();
            }
            _ => return
        }
    }

    /// parametro : var identificador identificadores ':' tipo_estendido
    fn parametro(&mut self) {
        self.var();
        self.identificador();
        self.identificadores();
        self.match_(TipoToken::Delim);
        self.tipo_extendido();
    }

    /// parametros : parametro parametros2 | <<vazio>>
    fn parametros(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCvar
            | TipoToken::Ident => {
                self.parametro();
                self.parametros2();
            }
            _ => return
        }
    }

    /// parametros2 : ',' parametro parametros2 | <<vazio>>
    fn parametros2(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                self.match_(TipoToken::Virgula);
                self.parametro();
                self.parametros2();
            }
            _ => return
        }
    }

    /// var : 'var' | <<vazio>>
    fn var(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCvar => {
                self.match_(TipoToken::PCvar);
            }
            _ => return
        }
    }

    /// corpo : declaracoes_locais cmds
    fn corpo(&mut self) {
        self.declaracoes_locais();
        self.cmds();
    }

    /// cmd : cmdLeia | cmdEscreva | cmdSe | cmdCaso | cmdPara| cmdEnquanto
    ///     | cmdFaca | cmdAtribuicao | cmdChamada | cmdRetorne
    fn cmd(&mut self) {
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
    fn cmds(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCleia | TipoToken::PCescreva | TipoToken::PCse | TipoToken::PCcaso
            | TipoToken::PCpara | TipoToken::PCenquanto | TipoToken::PCfaca | TipoToken::Circunflexo
            | TipoToken::Ident | TipoToken::PCretorne => {
                self.cmd();
                self.cmds();
            }
            _ => return
        }
    }

    /// cmdLeia : 'leia' '(' circunflexo identificador cmdLeia2 ')'
    fn cmd_leia(&mut self) {
        self.match_(TipoToken::PCleia);
        self.match_(TipoToken::AbrePar);
        self.circunflexo();
        self.identificador();
        self.cmd_leia2();
        self.match_(TipoToken::FechaPar);
    }

    /// cmdLeia2 : ',' circunflexo identificador cmdLeia2 | <<vazio>>
    fn cmd_leia2(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                self.match_(TipoToken::Virgula);
                self.circunflexo();
                self.identificador();
                self.cmd_leia2();
            }
            _ => return
        }
    }

    /// cmdEscreva : 'escreva' '(' expressao expressoes ')'
    fn cmd_escreva(&mut self) {
        self.match_(TipoToken::PCescreva);
        self.match_(TipoToken::AbrePar);
        self.expressao();
        self.expressoes();
        self.match_(TipoToken::FechaPar);
    }

    /// cmdSe : 'se' expressao 'entao' cmds senao 'fim_se'
    fn cmd_se(&mut self) {
        self.match_(TipoToken::PCse);
        self.expressao();
        self.match_(TipoToken::PCentao);
        self.cmds();
        self.senao();
        self.match_(TipoToken::PCfimSe);
    }

    /// senao : 'senao' cmds | <<vazio>>
    fn senao(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCsenao => {
                self.match_(TipoToken::PCsenao);
                self.cmds();
            }
            _ => return
        }
    }

    /// cmdCaso : 'caso' exp_aritmetica 'seja' selecao senao 'fim_caso'
    fn cmd_caso(&mut self) {
        self.match_(TipoToken::PCcaso);
        self.exp_aritmetica();
        self.match_(TipoToken::PCseja);
        self.selecao();
        self.senao();
        self.match_(TipoToken::PCfimCaso);
    }

    /// cmdPara : 'para' IDENT '<-' exp_aritmetica 'ate' exp_aritmetica 'faca' cmds 'fim_para'
    fn cmd_para(&mut self) {
        self.match_(TipoToken::PCpara);
        self.match_(TipoToken::Ident);
        self.match_(TipoToken::BackArrow);
        self.exp_aritmetica();
        self.match_(TipoToken::PCate);
        self.exp_aritmetica();
        self.match_(TipoToken::PCfaca);
        self.cmds();
        self.match_(TipoToken::PCfimPara);
    }

    /// cmdEnquanto : 'enquanto' expressao 'faca' cmds 'fim_enquanto'
    fn cmd_enquanto(&mut self) {
        self.match_(TipoToken::PCenquanto);
        self.expressao();
        self.match_(TipoToken::PCfaca);
        self.cmds();
        self.match_(TipoToken::PCfimEnquanto);
    }

    /// cmdFaca : 'faca' cmds 'ate' expressao
    fn cmd_faca(&mut self) {
        self.match_(TipoToken::PCfaca);
        self.cmds();
        self.match_(TipoToken::PCate);
        self.expressao();
    }

    /// cmdAtribuicao : circunflexo identificador '<-' expressao
    fn cmd_atribuicao(&mut self) {
        self.circunflexo();
        self.identificador();
        self.match_(TipoToken::BackArrow);
        self.expressao();
    }

    /// cmdChamada : IDENT '(' expressao expressoes ')'
    fn cmd_chamada(&mut self) {
        self.match_(TipoToken::Ident);
        self.match_(TipoToken::AbrePar);
        self.expressao();
        self.expressoes();
        self.match_(TipoToken::FechaPar);
    }

    /// cmdRetorne : 'retorne' expressao
    fn cmd_retorne(&mut self) {
        self.match_(TipoToken::PCretorne);
        self.expressao();
    }

    /// selecao : item_selecao selecao | <<vazio>>
    fn selecao(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritSub | TipoToken::NumInt => {
                self.item_selecao();
                self.selecao();
            }
            _ => return
        }
    }

    /// item_selecao : constantes ':' cmds
    fn item_selecao(&mut self) {
        self.constantes();
        self.match_(TipoToken::Delim);
        self.cmds();
    }
    
    /// constantes : numero_intervalo numero_intervalos
    fn constantes(&mut self) {
        self.numero_intervalo();
        self.numero_intervalos();
    }
    
    /// numero_intervalo : op_unario NUM_INT numero_intervalo2
    fn numero_intervalo(&mut self) {
        self.op_unario();
        self.match_(TipoToken::NumInt);
        self.numero_intervalo2();
    }
    
    /// numero_intervalos : ',' numero_intervalo numero_intervalos | <<vazio>>
    fn numero_intervalos(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                self.match_(TipoToken::Virgula);
                self.numero_intervalo();
                self.numero_intervalos();
            }
            _ => return
        }
    }
    
    /// numero_intervalo2 : '..' op_unario NUM_INT | <<vazio>>
    fn numero_intervalo2(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PontoPonto => {
                self.match_(TipoToken::PontoPonto);
                self.op_unario();
                self.match_(TipoToken::NumInt);
            }
            _ => return
        }
    }
    
    /// op_unario : '-' | <<vazio>>
    fn op_unario(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritSub => {
                self.match_(TipoToken::OpAritSub);
            }
            _ => return
        }
    }
    
    /// exp_aritmetica : termo termos
    fn exp_aritmetica(&mut self) {
        self.termo();
        self.termos();
    }
    
    /// termo : fator fatores
    fn termo(&mut self) {
        self.fator();
        self.fatores();
    }
    
    /// termos : op1 termo termos | <<vazio>>
    fn termos(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritSoma | TipoToken::OpAritSub => {
                self.op1();
                self.termo();
                self.termos();
            }
            _ => return
        }
    }
    
    /// fator : parcela parcelas
    fn fator(&mut self) {
        self.parcela();
        self.parcelas();
    }
    
    /// fatores : op2 fator fatores | <<vazio>>
    fn fatores(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritMult | TipoToken::OpAritDiv => {
                self.op2();
                self.fator();
                self.fatores();
            }
            _ => return
        }
    }
    
    /// op1 : '+' | '-'
    fn op1(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritSoma => self.match_(TipoToken::OpAritSoma),
            TipoToken::OpAritSub => self.match_(TipoToken::OpAritSub),
            _ => self.erro_sintatico()
        }
    }
    
    /// op2 : '*' | '/'
    fn op2(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritMult => self.match_(TipoToken::OpAritMult),
            TipoToken::OpAritDiv => self.match_(TipoToken::OpAritDiv),
            _ => self.erro_sintatico()
        }
    }
    
    /// op3 : '%'
    fn op3(&mut self) {
        self.match_(TipoToken::Porcento);
    }
    
    /// parcela : op_unario parcela_unario | parcela_nao_unario
    fn parcela(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritSub
            | TipoToken::Circunflexo
            | TipoToken::Ident
            | TipoToken::NumInt
            | TipoToken::NumReal
            | TipoToken::AbrePar => {
                self.op_unario();
                self.parcela_unario();
            }
            TipoToken::EComercial
            | TipoToken::Cadeia => self.parcela_nao_unario(),
            _ => self.erro_sintatico()
        }
    }
    
    /// parcelas : op3 parcela parcelas | <<vazio>>
    fn parcelas(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::Porcento => {
                self.op3();
                self.parcela();
                self.parcelas();
            }
            _ => return
        }
    }
    
    /// parcela_unario : circunflexo identificador
    ///     | IDENT '(' expressao expressoes ')'
    ///     | NUM_INT
    ///     | NUM_REAL
    ///     | '(' expressao ')'
    fn parcela_unario(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::Circunflexo => {
                self.circunflexo();
                self.identificador();
            }
            TipoToken::Ident => {
                match self.lookahead(2).tipo() {
                    TipoToken::AbrePar => {
                        self.match_(TipoToken::Ident);
                        self.match_(TipoToken::AbrePar);
                        self.expressao();
                        self.expressoes();
                        self.match_(TipoToken::FechaPar);
                    }
                    _ => self.identificador()
                }
            }
            TipoToken::NumInt => self.match_(TipoToken::NumInt),
            TipoToken::NumReal => self.match_(TipoToken::NumReal),
            TipoToken::AbrePar => {
                self.match_(TipoToken::AbrePar);
                self.expressao();
                self.match_(TipoToken::FechaPar);
            }
            _ => self.erro_sintatico()
        }
    }
    
    /// parcela_nao_unario : '&' identificador | CADEIA
    fn parcela_nao_unario(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::EComercial => {
                self.match_(TipoToken::EComercial);
                self.identificador();
            }
            TipoToken::Cadeia => self.match_(TipoToken::Cadeia),
            _ => self.erro_sintatico()
        }
    }
    
    /// exp_relacional : exp_aritmetica exp_relacional2
    fn exp_relacional(&mut self) {
        self.exp_aritmetica();
        self.exp_relacional2();
    }
    
    /// exp_relacional2 : op_relacional exp_aritmetica | <<vazio>>
    fn exp_relacional2(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::OpRelIgual
            | TipoToken::OpRelDif
            | TipoToken::OpRelMaiorIgual
            | TipoToken::OpRelMenorIgual
            | TipoToken::OpRelMaior
            | TipoToken::OpRelMenor => {
                self.op_relacional();
                self.exp_aritmetica();
            }
            _ => return
        }
    }
    
    /// op_relacional : '=' | '<>' | '>=' | '<=' | '>' | '<'
    fn op_relacional(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::OpRelIgual => self.match_(TipoToken::OpRelIgual),
            TipoToken::OpRelDif => self.match_(TipoToken::OpRelDif),
            TipoToken::OpRelMaiorIgual => self.match_(TipoToken::OpRelMaiorIgual),
            TipoToken::OpRelMenorIgual => self.match_(TipoToken::OpRelMenorIgual),
            TipoToken::OpRelMaior => self.match_(TipoToken::OpRelMaior),
            TipoToken::OpRelMenor => self.match_(TipoToken::OpRelMenor),
            _ => self.erro_sintatico()
        }
    }
    
    /// expressao : termo_logico termos_logicos
    fn expressao(&mut self) {
        self.termo_logico();
        self.termos_logicos();
    }
    
    /// expressoes : ',' expressao expressoes | <<vazio>>
    fn expressoes(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                self.match_(TipoToken::Virgula);
                self.expressao();
                self.expressoes();
            }
            _ => return
        }
    }
    
    /// termo_logico : fator_logico fatores_logicos
    fn termo_logico(&mut self) {
        self.fator_logico();
        self.fatores_logicos();
    }
    
    /// termos_logicos : op_logico_1 termo_logico termos_logicos | <<vazio>>
    fn termos_logicos(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCou => {
                self.op_logico_1();
                self.termo_logico();
                self.termos_logicos();
            }
            _ => return
        }
    }
    
    /// fator_logico : nao parcela_logica
    fn fator_logico(&mut self) {
        self.nao();
        self.parcela_logica();
    }
    
    /// fatores_logicos : op_logico_2 fator_logico fatores_logicos | <<vazio>>
    fn fatores_logicos(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCe => {
                self.op_logico_2();
                self.fator_logico();
                self.fatores_logicos();
            }
            _ => return
        }
    }
    
    /// nao : 'nao' | <<vazio>>
    fn nao(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCnao => self.match_(TipoToken::PCnao),
            _ => return
        }
    }
    
    /// parcela_logica : constante_logica | exp_relacional
    fn parcela_logica(&mut self) {
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
    fn constante_logica(&mut self) {
        match self.lookahead(1).tipo() {
            TipoToken::PCverdadeiro => self.match_(TipoToken::PCverdadeiro),
            TipoToken::PCfalso => self.match_(TipoToken::PCfalso),
            _ => self.erro_sintatico()
        }
    }
    
    /// op_logico_1 : 'ou'
    fn op_logico_1(&mut self) {
        self.match_(TipoToken::PCou);
    }
    
    /// op_logico_2 : 'e'
    fn op_logico_2(&mut self) {
        self.match_(TipoToken::PCe);
    }
}