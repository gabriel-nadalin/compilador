use crate::semantico::{
    escopos::Escopos,
    tabela_de_simbolos::TipoSimbolo,
    visitor::Visitor,
};

use crate::sintatico::arvore_sintatica::{NoAST, RegraAST};


/// analisador semantico
pub struct Gerador {
    escopos: Escopos,
    saida: String,
    identacao: u8,
}

impl Gerador {

    /// retorna instancia de gerador de codigo em C
    pub fn new() -> Self {
        Self {
            escopos: Escopos::new(TipoSimbolo::Vazio),
            saida: "".to_string(),
            identacao: 0,
        }
    }

    pub fn saida(&self) -> &str {
        &self.saida
    }

    fn new_line(&self) -> String {
        let mut retorno = "\n".to_string();
        for _ in 0..self.identacao {
            retorno += "\t";
        }
        retorno
    }
}

impl Visitor for Gerador {

    /// verifica um no da arvore sintatica segundo seus requisitos semanticos especificos
    fn visit(&mut self, no: &NoAST) {
        let filhos = no.filhos();
        let escopos = self.escopos.clone();

        match no.regra() {

            // programa : declaracoes 'algoritmo' corpo 'fim_algoritmo'
            RegraAST::Programa => {
                self.saida += "#include <stdio.h>\n#include <stdlib.h>\n\n";
                self.visit(&filhos[0]);
                self.saida += "\nint main(void) {\n\t";
                self.identacao += 1;
                self.visit(&filhos[1]);
                self.saida += "return 0;\n}\n";
                self.identacao -= 1;
            }

            // declaracoes : declaracao declaracoes | <<vazio>>
            RegraAST::Declaracoes => {
                self.visit(&filhos[0]);
                self.visit(&filhos[1]);
            }
            // declaracao : declaracao_local | declaracao_global
            
            // declaracao_local :
            //     'declare' variavel
            RegraAST::DeclaracaoVariavel => {
                self.visit(&no.filhos()[0])
            }

            //     | 'tipo' IDENT ':' tipo
            RegraAST::DeclaracaoTipo => {

            }

            //     | 'constante' IDENT ':' tipo_basico '=' valor_constante
            RegraAST::DeclaracaoConstante => {}

            // valor_constante : CADEIA | NUM_INT | NUM_REAL | 'verdadeiro' | 'falso'
            RegraAST::ValorConstante (Token) => {}

            // variavel : identificador identificadores ':' tipo
            RegraAST::Variavel => {
                self.saida += match filhos[2].tipo(&escopos) {
                    TipoSimbolo::Cadeia => "char ",
                    TipoSimbolo::Inteiro => "int ",
                    TipoSimbolo::Real => "float ",
                    _ => ""
                };
                self.saida += &(filhos[0].texto() + &filhos[1].texto() + ";" + &self.new_line());
                let escopo_atual = self.escopos.escopo_atual();
                escopo_atual.inserir(&filhos[0].texto(), &filhos[2].tipo(&escopos));
                for ident in filhos[1].idents() {
                    escopo_atual.inserir(&ident.lexema(), &filhos[2].tipo(&escopos))
                }
            }
            // tipo : registro | tipo_estendido

            // identificador : IDENT identificador2 dimensao
            RegraAST::Identificador => {
                self.saida += &no.texto()
            }

            // identificador2 : '.' IDENT identificador2 | <<vazio>>
            RegraAST::Identificador2 => {}

            // identificadores: ',' identificador identificadores | <<vazio>>
            RegraAST::Identificadores => {}

            // dimensao : '[' exp_aritmetica ']' dimensao | <<vazio>>
            RegraAST::Dimensao => {}

            // tipo_estendido : circunflexo tipo_basico_ident
            RegraAST::TipoExtendido => {}
            // tipo_basico_ident : tipo_basico
            //    | IDENT
            RegraAST::Ident (Token) => {}

            // tipo_basico : 'literal' | 'inteiro' | 'real' | 'logico'
            RegraAST::TipoBasico (Token) => {}

            // circunflexo: '^' | <<vazio>>
            RegraAST::Circunflexo => {}

            // registro : 'registro' variaveis 'fim_Registro' fecha_escopo
            RegraAST::Registro => {}

            // variaveis : variavel variaveis | <<vazio>>
            RegraAST::Variaveis => {}

            // declaracao_global :
            //     'procedimento' IDENT '(' parametros ')' declaracoes_locais cmds 'fim_procedimento' fecha_escopo
            RegraAST::DeclaracaoProcedimento => {}

            //     | 'funcao' IDENT '(' parametros ')' ':' tipo_estendido declaracoes_locais cmds 'fim_funcao' fecha_escopo
            RegraAST::DeclaracaoFuncao => {}

            // declaracoes_locais : declaracao_local declaracoes_locais | <<vazio>>
            RegraAST::DeclaracoesLocais => {
                print!("aqui!!");
                self.visit(&filhos[0]);
                self.visit(&filhos[1]);
            }

            // parametro : var identificador identificadores ':' tipo_estendido
            RegraAST::Parametro => {}

            // parametros : parametro parametros2 | <<vazio>>
            RegraAST::Parametros => {}

            // parametros2 : ',' parametro parametros2 | <<vazio>>
            RegraAST::Parametros2 => {}

            // var : 'var' | <<vazio>>
            RegraAST::Var => {}

            // corpo : declaracoes_locais cmds
            RegraAST::Corpo => {
                self.visit(&filhos[0]);
                self.visit(&filhos[1]);
            }

            // cmds : cmd cmds | <<vazio>>
            RegraAST::CMDs => {
                self.visit(&filhos[0]);
                self.visit(&filhos[1]);
            }
            // cmd : cmdLeia | cmdEscreva | cmdSe | cmdCaso | cmdPara | cmdEnquanto
            //     | cmdFaca | cmdAtribuicao | cmdChamada | cmdRetorne

            // cmdLeia : 'leia' '(' circunflexo identificador cmdLeia2 ')'
            RegraAST::CMDLeia => {
                self.saida += "scanf(\"%";
                self.saida += match filhos[1].tipo(&escopos) {
                    TipoSimbolo::Cadeia => "s",
                    TipoSimbolo::Inteiro => "d",
                    TipoSimbolo::Real => "f",
                    _ => ""
                };
                self.saida += "\", ";
                self.saida += if filhos[1].tipo(&escopos) != TipoSimbolo::Cadeia {"&"} else {""};
                self.saida += &(filhos[1].texto() + ");" + &self.new_line());
                self.visit(&filhos[2])
            }

            // cmdLeia2 : ',' circunflexo identificador cmdLeia2 | <<vazio>>
            RegraAST::CMDLeia2 => {
                self.saida += "scanf(\"%";
                self.saida += match filhos[1].tipo(&escopos) {
                    TipoSimbolo::Cadeia => "s",
                    TipoSimbolo::Inteiro => "d",
                    TipoSimbolo::Real => "f",
                    _ => ""
                };
                self.saida += "\", ";
                self.saida += if filhos[1].tipo(&escopos) != TipoSimbolo::Cadeia {"&"} else {""};
                self.saida += &(filhos[1].texto() + ");" + &self.new_line());
            }

            // cmdEscreva : 'escreva' '(' expressao expressoes ')'
            RegraAST::CMDEscreva => {
                self.saida += "printf(\"%";
                self.saida += match filhos[0].tipo(&escopos) {
                    TipoSimbolo::Cadeia => "s",
                    TipoSimbolo::Inteiro => "d",
                    TipoSimbolo::Real => "f",
                    _ => ""
                };
                let mut expressao = &filhos[1];
                while *expressao.regra() != RegraAST::Vazio {
                    self.saida += " %";
                    self.saida += match expressao.filhos()[0].tipo(&escopos) {
                        TipoSimbolo::Cadeia => "s",
                        TipoSimbolo::Inteiro => "d",
                        TipoSimbolo::Real => "f",
                        _ => ""
                    };
                    expressao = &expressao.filhos()[1];
                }
                self.saida += "\", ";
                self.saida += &(filhos[0].texto() + &filhos[1].texto() + ");" + &self.new_line());
            }

            // cmdSe : 'se' expressao 'entao' cmds senao 'fim_se'
            RegraAST::CMDSe => {}

            // senao : 'senao' cmds | <<vazio>>
            RegraAST::Senao => {}

            // cmdCaso : 'caso' exp_aritmetica 'seja' selecao senao 'fim_caso'
            RegraAST::CMDCaso => {}

            // cmdPara : 'para' IDENT '<-' exp_aritmetica 'ate' exp_aritmetica 'faca' cmds 'fim_para'
            RegraAST::CMDPara => {}

            // cmdEnquanto : 'enquanto' expressao 'faca' cmds 'fim_enquanto'
            RegraAST::CMDEnquanto => {}

            // cmdFaca : 'faca' cmds 'ate' expressao
            RegraAST::CMDFaca => {}

            // cmdAtribuicao : circunflexo identificador '<-' expressao
            RegraAST::CMDAtribuicao => {}

            // cmdChamada : IDENT '(' expressao expressoes ')'
            RegraAST::CMDChamada => {}

            // cmdRetorne : 'retorne' expressao
            RegraAST::CMDRetorne => {}

            // selecao : item_selecao selecao | <<vazio>>
            RegraAST::Selecao => {}

            // item_selecao : constantes ':' cmds
            RegraAST::ItemSelecao => {}

            // constantes : numero_intervalo numero_intervalos
            RegraAST::Constantes => {}

            // numero_intervalo : op_unario NUM_INT numero_intervalo2
            RegraAST::NumeroIntervalo => {}

            // numero_intervalos : ',' numero_intervalo numero_intervalos | <<vazio>>
            RegraAST::NumeroIntervalos => {}

            // numero_intervalo2 : '..' op_unario NUM_INT | <<vazio>>
            RegraAST::NumeroIntervalo2 => {}

            // op_unario : '-' | <<vazio>>
            RegraAST::OpUnario => {}

            // exp_aritmetica : termo termos
            RegraAST::ExpAritmetica => {}

            // termo : fator fatores
            RegraAST::Termo => {}

            // termos : op1 termo termos | <<vazio>>
            RegraAST::Termos => {}

            // op1 : '+' | '-'
            RegraAST::Op1 (Token) => {}

            // fator : parcela parcelas
            RegraAST::Fator => {}

            // fatores : op2 fator fatores | <<vazio>>
            RegraAST::Fatores => {}

            // op2 : '*' | '/'
            RegraAST::Op2 (Token) => {}

            // parcela : op_unario parcela_unario | parcela_nao_unario
            RegraAST::Parcela => {}

            // parcelas : op3 parcela parcelas | <<vazio>>
            RegraAST::Parcelas => {}

            // op3 : '%'
            RegraAST::Op3 => {}

            // parcela_unario : circunflexo identificador
            RegraAST::ParcelaUnario1 => {}

            //     | IDENT '(' expressao expressoes ')'
            RegraAST::ParcelaUnario2 => {}

            //     | '(' expressao ')'
            RegraAST::ParcelaUnario3 => {}

            //     | NUM_INT
            RegraAST::NumInt (Token) => {}

            //     | NUM_REAL
            RegraAST::NumReal (Token) => {}

            // parcela_nao_unario : '&' identificador
            RegraAST::ParcelaNaoUnario => {}

            //     | CADEIA
            RegraAST::Cadeia (Token) => {}

            // exp_relacional : exp_aritmetica exp_relacional2
            RegraAST::ExpRelacional => {}

            // exp_relacional2 : op_relacional exp_aritmetica | <<vazio>>
            RegraAST::ExpRelacional2 => {}

            // op_relacional : '=' | '<>' | '>=' | '<=' | '>' | '<'
            RegraAST::OpRelacional (Token) => {}

            // expressao : termo_logico termos_logicos
            RegraAST::Expressao => {}

            // expressoes : ',' expressao expressoes | <<vazio>>
            RegraAST::Expressoes => {}

            // termo_logico : fator_logico fatores_logicos
            RegraAST::TermoLogico => {}

            // termos_logicos : op_logico_1 termo_logico termos_logicos | <<vazio>>
            RegraAST::TermosLogicos => {}

            // fator_logico : nao parcela_logica
            RegraAST::FatorLogico => {}
            // parcela_logica : constante_logica
            //     | exp_relacional

            // fatores_logicos : op_logico_2 fator_logico fatores_logicos | <<vazio>>
            RegraAST::FatoresLogicos => {}

            // nao : 'nao' | <<vazio>>
            RegraAST::Nao => {}

            // constante_logica : 'verdadeiro' | 'falso'
            RegraAST::ConstanteLogica(Token) => {}

            // op_logico_1 : 'ou'
            RegraAST::OpLogico1 => {}

            // op_logico_2 : 'e'
            RegraAST::OpLogico2 => {}

            
            
            RegraAST::FechaEscopo => {
                self.escopos.abandonar_escopo()
            }

            _ => {}
        }
    }
}
