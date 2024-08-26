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

    fn new_line(&mut self) {
        self.saida += "\n";
        for _ in 0..self.identacao {
            self.saida += "\t";
        }
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
                self.saida += "#include <stdio.h>\n#include <stdlib.h>";
                self.visit(&filhos[0]);
                self.saida += "\n\nint main(void) {";
                self.identacao += 1;
                self.visit(&filhos[1]);
                self.new_line();
                self.saida += "return 0;\n}\n";
                self.identacao -= 1;
            }

            // declaracoes : declaracao declaracoes | <<vazio>>
            // RegraAST::Declaracoes => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }
            // declaracao : declaracao_local | declaracao_global
            
            // declaracao_local :
            //     'declare' variavel
            // RegraAST::DeclaracaoVariavel => {
            //     self.visit(&no.filhos()[0])
            // }

            //     | 'tipo' IDENT ':' tipo
            RegraAST::DeclaracaoTipo => {

            }

            //     | 'constante' IDENT ':' tipo_basico '=' valor_constante
            RegraAST::DeclaracaoConstante => {
                self.new_line();
                self.new_line();
                self.saida += &format!("#define {} ", filhos[0].texto());
                self.visit(&filhos[2]);
            }

            // valor_constante : CADEIA | NUM_INT | NUM_REAL | 'verdadeiro' | 'falso'
            RegraAST::ValorConstante (token) => {
                self.saida += &match token.lexema().as_ref() {
                    "verdadeiro" => "true".to_string(),
                    "falso" => "false".to_string(),
                    _ => token.lexema()
                }
            }

            // variavel : identificador identificadores ':' tipo
            RegraAST::Variavel => {
                self.new_line();
                let tipo = filhos[2].tipo(&escopos);

                self.saida += match tipo {
                    TipoSimbolo::Cadeia => "char ",
                    TipoSimbolo::Inteiro => "int ",
                    TipoSimbolo::Real => "float ",
                    _ => ""
                };

                self.visit(&filhos[0]);
                if tipo == TipoSimbolo::Cadeia {
                    self.saida += "[80]"
                }

                let mut identificadores = &filhos[1];
                while *identificadores.regra() != RegraAST::Vazio {
                    self.visit(identificadores);
                    if tipo == TipoSimbolo::Cadeia {
                        self.saida += "[80]"
                    }
                    identificadores = &identificadores.filhos()[1]
                }
                self.saida += ";";
                
                let escopo_atual = self.escopos.escopo_atual();
                for ident in no.idents() {
                    escopo_atual.inserir(&ident.lexema(), &tipo);
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
            RegraAST::Identificadores => {
                self.saida += &no.texto()
            }

            // dimensao : '[' exp_aritmetica ']' dimensao | <<vazio>>
            RegraAST::Dimensao => {}

            // tipo_estendido : circunflexo tipo_basico_ident
            RegraAST::TipoExtendido => {}
            // tipo_basico_ident : tipo_basico
            //    | IDENT
            RegraAST::Ident (token) => {
                self.saida += &token.lexema();
            }

            // tipo_basico : 'literal' | 'inteiro' | 'real' | 'logico'
            RegraAST::TipoBasico (token) => {}

            // circunflexo: '^' | <<vazio>>
            RegraAST::Circunflexo => {}

            // registro : 'registro' variaveis 'fim_Registro' fecha_escopo
            RegraAST::Registro => {}

            // variaveis : variavel variaveis | <<vazio>>
            RegraAST::Variaveis => {}

            // declaracao_global :
            //     'procedimento' IDENT '(' parametros ')' declaracoes_locais cmds 'fim_procedimento' fecha_escopo
            RegraAST::DeclaracaoProcedimento => {
                self.new_line();
                self.new_line();
                let nome = filhos[0].texto();
                self.saida += &format!("void {}( ", nome);
                self.visit(&filhos[1]);
                self.saida += ") {";
                self.identacao += 1;

                self.escopos.escopo_atual().inserir(&nome, &no.tipo(&escopos));
                self.escopos.novo_escopo(TipoSimbolo::Vazio);

                self.visit(&filhos[2]);
                self.visit(&filhos[3]);

                self.identacao -= 1;
                self.new_line();
                self.saida += "}";
                self.visit(&filhos[4]);
            }

            //     | 'funcao' IDENT '(' parametros ')' ':' tipo_estendido declaracoes_locais cmds 'fim_funcao' fecha_escopo
            RegraAST::DeclaracaoFuncao => {}

            // declaracoes_locais : declaracao_local declaracoes_locais | <<vazio>>
            // RegraAST::DeclaracoesLocais => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }

            // parametro : var identificador identificadores ':' tipo_estendido
            RegraAST::Parametro => {}

            // parametros : parametro parametros2 | <<vazio>>
            RegraAST::Parametros => {}

            // parametros2 : ',' parametro parametros2 | <<vazio>>
            RegraAST::Parametros2 => {}

            // var : 'var' | <<vazio>>
            RegraAST::Var => {}

            // corpo : declaracoes_locais cmds
            // RegraAST::Corpo => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }

            // cmds : cmd cmds | <<vazio>>
            // RegraAST::CMDs => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }
            // cmd : cmdLeia | cmdEscreva | cmdSe | cmdCaso | cmdPara | cmdEnquanto
            //     | cmdFaca | cmdAtribuicao | cmdChamada | cmdRetorne

            // cmdLeia : 'leia' '(' circunflexo identificador cmdLeia2 ')'
            RegraAST::CMDLeia => {
                self.new_line();
                self.saida += "scanf(\"%";
                self.saida += match filhos[1].tipo(&escopos) {
                    TipoSimbolo::Cadeia => "s",
                    TipoSimbolo::Inteiro => "d",
                    TipoSimbolo::Real => "f",
                    _ => ""
                };
                self.saida += "\", ";
                if filhos[1].tipo(&escopos) != TipoSimbolo::Cadeia {
                    self.saida += "&";
                }
                self.visit(&filhos[1]);
                self.saida += ");";
                self.visit(&filhos[2]);
            }

            // cmdLeia2 : ',' circunflexo identificador cmdLeia2 | <<vazio>>
            RegraAST::CMDLeia2 => {
                self.new_line();
                self.saida += "scanf(\"%";
                self.saida += match filhos[1].tipo(&escopos) {
                    TipoSimbolo::Cadeia => "s",
                    TipoSimbolo::Inteiro => "d",
                    TipoSimbolo::Real => "f",
                    _ => ""
                };
                self.saida += "\", ";
                if filhos[1].tipo(&escopos) != TipoSimbolo::Cadeia {
                    self.saida += "&";
                }
                self.visit(&filhos[1]);
                self.saida += ");";
                self.new_line();
                self.visit(&filhos[2]);
            }

            // cmdEscreva : 'escreva' '(' expressao expressoes ')'
            RegraAST::CMDEscreva => {
                self.new_line();
                self.saida += "printf(\"%";
                self.saida += match filhos[0].tipo(&escopos) {
                    TipoSimbolo::Cadeia => "s",
                    TipoSimbolo::Inteiro => "d",
                    TipoSimbolo::Real => "f",
                    _ => ""
                };
                let mut expressoes = &filhos[1];
                while *expressoes.regra() != RegraAST::Vazio {
                    self.saida += "%";
                    self.saida += match expressoes.filhos()[0].tipo(&escopos) {
                        TipoSimbolo::Cadeia => "s",
                        TipoSimbolo::Inteiro => "d",
                        TipoSimbolo::Real => "f",
                        _ => ""
                    };
                    expressoes = &expressoes.filhos()[1];
                }
                self.saida += "\", ";
                self.saida += &(filhos[0].texto() + &filhos[1].texto() + ");");
            }

            // cmdSe : 'se' expressao 'entao' cmds senao 'fim_se'
            RegraAST::CMDSe => {
                self.new_line();
                self.saida += "if (";
                self.visit(&filhos[0]);
                self.saida += ") {";
                self.identacao += 1;
                self.visit(&filhos[1]);
                self.identacao -= 1;
                self.new_line();
                self.saida += "}";
                if *filhos[2].regra() != RegraAST::Vazio {
                    self.saida += " else {";
                    self.identacao += 1;
                    self.visit(&filhos[2]);
                    self.identacao -= 1;
                    self.new_line();
                    self.saida += "}";
                }
            }

            // senao : 'senao' cmds | <<vazio>>
            // RegraAST::Senao => {
            //     self.visit(&filhos[0]);
            // }

            // cmdCaso : 'caso' exp_aritmetica 'seja' selecao senao 'fim_caso'
            RegraAST::CMDCaso => {
                self.new_line();
                self.saida += "switch (";
                self.visit(&filhos[0]);
                self.saida += ") {";
                self.identacao += 1;
                self.visit(&filhos[1]);
                if *filhos[2].regra() != RegraAST::Vazio {
                    self.new_line();
                    self.saida += "default:";
                    self.identacao += 1;
                    self.visit(&filhos[2]);
                }
                self.identacao -= 2;
                self.new_line();
                self.saida += "}";
            }

            // cmdPara : 'para' IDENT '<-' exp_aritmetica 'ate' exp_aritmetica 'faca' cmds 'fim_para'
            RegraAST::CMDPara => {
                self.new_line();
                self.saida += &format!("for ({} = ", filhos[0].texto());
                self.visit(&filhos[1]);
                self.saida += &format!("; {} <= ", filhos[0].texto());
                self.visit(&filhos[2]);
                self.saida += &format!("; {}++) {{", filhos[0].texto());
                self.identacao += 1;
                self.visit(&filhos[3]);
                self.identacao -= 1;
                self.new_line();
                self.saida += "}";
            }

            // cmdEnquanto : 'enquanto' expressao 'faca' cmds 'fim_enquanto'
            RegraAST::CMDEnquanto => {
                self.new_line();
                self.saida += "while (";
                self.visit(&filhos[0]);
                self.saida += ") {";
                self.identacao += 1;
                self.visit(&filhos[1]);
                self.identacao -= 1;
                self.new_line();
                self.saida += "}"
            }

            // cmdFaca : 'faca' cmds 'ate' expressao
            RegraAST::CMDFaca => {
                self.new_line();
                self.saida += "do {";
                self.identacao += 1;
                self.visit(&filhos[0]);
                self.identacao -= 1;
                self.new_line();
                self.saida += "} while (";
                self.visit(&filhos[1]);
                self.saida += ");"
            }

            // cmdAtribuicao : circunflexo identificador '<-' expressao
            RegraAST::CMDAtribuicao => {
                self.new_line();
                self.saida += &(filhos[1].texto() + " = " + &filhos[2].texto() + ";");
            }

            // cmdChamada : IDENT '(' expressao expressoes ')'
            RegraAST::CMDChamada => {}

            // cmdRetorne : 'retorne' expressao
            RegraAST::CMDRetorne => {}

            // selecao : item_selecao selecao | <<vazio>>
            // RegraAST::Selecao => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }

            // item_selecao : constantes ':' cmds
            RegraAST::ItemSelecao => {
                self.visit(&filhos[0]);
                self.identacao += 1;
                self.visit(&filhos[1]);
                self.new_line();
                self.saida += "break;";
                self.identacao -= 1;
            }

            // constantes : numero_intervalo numero_intervalos
            RegraAST::Constantes => {
                let (num1, num2) = filhos[0].intervalo();
                for i in num1..num2 + 1 {
                    self.new_line();
                    self.saida += &format!("case {}:", i);
                }
            }

            // numero_intervalo : op_unario NUM_INT numero_intervalo2
            // RegraAST::NumeroIntervalo => {}

            // numero_intervalos : ',' numero_intervalo numero_intervalos | <<vazio>>
            // RegraAST::NumeroIntervalos => {}

            // numero_intervalo2 : '..' op_unario NUM_INT | <<vazio>>
            // RegraAST::NumeroIntervalo2 => {}

            // op_unario : '-' | <<vazio>>
            RegraAST::OpUnario => {
                self.saida += "-";
            }

            // exp_aritmetica : termo termos
            // RegraAST::ExpAritmetica => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }

            // termo : fator fatores
            // RegraAST::Termo => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }

            // termos : op1 termo termos | <<vazio>>
            // RegraAST::Termos => {}

            // op1 : '+' | '-'
            RegraAST::Op1 (token) => {
                self.saida += &format!(" {} ", token.lexema());
            }

            // fator : parcela parcelas
            // RegraAST::Fator => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }

            // fatores : op2 fator fatores | <<vazio>>
            // RegraAST::Fatores => {}

            // op2 : '*' | '/'
            RegraAST::Op2 (token) => {
                self.saida += &format!(" {} ", token.lexema());
            }

            // parcela : op_unario parcela_unario | parcela_nao_unario
            // RegraAST::Parcela => {}

            // parcelas : op3 parcela parcelas | <<vazio>>
            // RegraAST::Parcelas => {}

            // op3 : '%'
            RegraAST::Op3 => {
                self.saida += " % ";
            }

            // parcela_unario : circunflexo identificador
            // RegraAST::ParcelaUnario1 => {}

            //     | IDENT '(' expressao expressoes ')'
            // RegraAST::ParcelaUnario2 => {}

            //     | '(' expressao ')'
            // RegraAST::ParcelaUnario3 => {}

            //     | NUM_INT
            RegraAST::NumInt (token) => {
                self.saida += &token.lexema();
            }

            //     | NUM_REAL
            RegraAST::NumReal (token) => {
                self.saida += &token.lexema();
            }

            // parcela_nao_unario : '&' identificador
            // RegraAST::ParcelaNaoUnario => {}

            //     | CADEIA
            RegraAST::Cadeia (token) => {
                self.saida += &token.lexema();
            }

            // exp_relacional : exp_aritmetica exp_relacional2
            // RegraAST::ExpRelacional => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }

            // exp_relacional2 : op_relacional exp_aritmetica | <<vazio>>
            // RegraAST::ExpRelacional2 => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }

            // op_relacional : '=' | '<>' | '>=' | '<=' | '>' | '<'
            RegraAST::OpRelacional (token) => {
                self.saida += &match token.lexema().as_ref() {
                    "=" => " == ".to_string(),
                    "<>" => " != ".to_string(),
                    _ => format!(" {} ", token.lexema())
                }
            }

            // expressao : termo_logico termos_logicos
            // RegraAST::Expressao => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }

            // expressoes : ',' expressao expressoes | <<vazio>>
            RegraAST::Expressoes => {}

            // termo_logico : fator_logico fatores_logicos
            // RegraAST::TermoLogico => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            // }

            // termos_logicos : op_logico_1 termo_logico termos_logicos | <<vazio>>
            // RegraAST::TermosLogicos => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            //     self.visit(&filhos[2]);
            // }

            // fator_logico : nao parcela_logica
            RegraAST::FatorLogico => {
                self.visit(&filhos[0]);
                if *filhos[0].regra() != RegraAST::Vazio {
                    self.saida += "(";
                }
                self.visit(&filhos[1]);
                if *filhos[0].regra() != RegraAST::Vazio {
                    self.saida += ")";
                }
            }
            // parcela_logica : constante_logica
            //     | exp_relacional

            // fatores_logicos : op_logico_2 fator_logico fatores_logicos | <<vazio>>
            // RegraAST::FatoresLogicos => {
            //     self.visit(&filhos[0]);
            //     self.visit(&filhos[1]);
            //     self.visit(&filhos[2]);
            // }

            // nao : 'nao' | <<vazio>>
            RegraAST::Nao => {
                self.saida += "!"
            }

            // constante_logica : 'verdadeiro' | 'falso'
            RegraAST::ConstanteLogica(token) => {
                self.saida += match token.lexema().as_ref() {
                    "verdadeiro" => "true",
                    "falso" => "false",
                    _ => ""
                }
            }

            // op_logico_1 : 'ou'
            RegraAST::OpLogico1 => {
                self.saida += " || "
            }

            // op_logico_2 : 'e'
            RegraAST::OpLogico2 => {
                self.saida += " && "
            }

            RegraAST::FechaEscopo => {
                self.escopos.abandonar_escopo()
            }

            _ => {
                for filho in filhos {
                    self.visit(filho)
                }
            }
        }
    }
}
