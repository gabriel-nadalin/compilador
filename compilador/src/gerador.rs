use crate::semantico::{
    escopos::Escopos,
    tabela_de_simbolos::TipoSimbolo,
    visitor::Visitor,
};

use crate::sintatico::arvore_sintatica::{NoAST, RegraAST};


/// gerador de codigo em C
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

    /// retorna codigo gerado em linguagem C
    pub fn saida(&self) -> &str {
        &self.saida
    }

    /// adiciona nova linha na saida e o numero correto de espacamentos para identacao
    fn new_line(&mut self) {
        self.saida += "\n";
        for _ in 0..self.identacao {
            self.saida += "\t";
        }
    }
}

impl Visitor for Gerador {

    /// gera trecho de codigo em linguagem C referente a cada no da arvore sintatica
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
                println!("{}", self.identacao);
            }
            
            // declaracao_local :
            //     'declare' variavel
            //     | 'tipo' IDENT ':' tipo
            RegraAST::DeclaracaoTipo => {
                self.new_line();
                self.saida += "typedef ";
                self.visit(&filhos[1]);
                self.visit(&filhos[0]);
                self.saida += ";";
                let escopo_atual = self.escopos.escopo_atual();
                escopo_atual.inserir(&filhos[0].texto(), &filhos[1].tipo(&escopos));
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
                let mut tipo = filhos[2].tipo(&escopos);

                let ponteiro = if let TipoSimbolo::Ponteiro(tipo_point) = tipo {
                    tipo = *tipo_point;
                    true
                } else { false };

                self.visit(&filhos[2]);
                self.saida += " ";

                if ponteiro { self.saida += "*" }
                self.visit(&filhos[0]);
                if tipo == TipoSimbolo::Cadeia {
                    self.saida += "[80]"
                }

                let mut identificadores = &filhos[1];
                while *identificadores.regra() != RegraAST::Vazio {
                    let identificador = &identificadores.filhos()[0];
                    self.saida += ", ";
                    if ponteiro { self.saida += "*" }
                    self.visit(&identificador);
                    if tipo == TipoSimbolo::Cadeia {
                        self.saida += "[80]"
                    }
                    identificadores = &identificadores.filhos()[1]
                }
                self.saida += ";";
                
                let escopo_atual = self.escopos.escopo_atual();
                for ident in no.idents() {
                    let nome = ident.lexema();
                    escopo_atual.inserir(&nome, &tipo);

                    if let TipoSimbolo::Registro(ref atributos) = tipo {
                        for atributo in atributos {
                            let tipo = atributo.tipo(&escopos);
                            let idents = atributo.idents();
                            for ident in idents {
                                let nome = format!("{}.{}", nome, ident.lexema());
                                escopo_atual.inserir(&nome, &tipo)
                            }
                        }
                    }
                }
            }
            // tipo : registro | tipo_estendido

            // identificador : IDENT identificador2 dimensao
            RegraAST::Identificador => {
                self.saida += &no.texto()
            }

            // tipo_estendido : circunflexo tipo_basico_ident
            RegraAST::TipoExtendido => {
                self.visit(&filhos[1]);
            }
            // tipo_basico_ident : tipo_basico
            //    | IDENT
            RegraAST::Ident (token) => {
                self.saida += &token.lexema();
            }

            // tipo_basico : 'literal' | 'inteiro' | 'real' | 'logico'
            RegraAST::TipoBasico (token) => {
                self.saida += match token.lexema().as_ref() {
                    "literal" => "char",
                    "inteiro" => "int",
                    "real" => "float",
                    _ => ""
                }
            }

            // circunflexo: '^' | <<vazio>>
            RegraAST::Circunflexo => {
                self.saida += "*";
            }

            // registro : 'registro' variaveis 'fim_Registro' fecha_escopo
            RegraAST::Registro => {
                self.saida += "struct {";
                self.identacao += 1;
                self.visit(&filhos[0]);
                self.identacao -= 1;
                self.new_line();
                self.saida += "} ";
            }

            // declaracao_global :
            //     'procedimento' IDENT '(' parametros ')' declaracoes_locais cmds 'fim_procedimento' fecha_escopo
            RegraAST::DeclaracaoProcedimento => {
                self.new_line();
                self.new_line();
                let nome = filhos[0].texto();
                self.saida += &format!("void {}(", nome);
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
            RegraAST::DeclaracaoFuncao => {
                self.new_line();
                self.new_line();
                let nome = filhos[0].texto();

                self.visit(&filhos[2]);
                self.saida += &format!(" {}(", nome);
                self.visit(&filhos[1]);
                self.saida += ") {";
                self.identacao += 1;

                self.escopos.escopo_atual().inserir(&nome, &no.tipo(&escopos));
                self.escopos.novo_escopo(filhos[2].tipo(&escopos));

                self.visit(&filhos[3]);
                self.visit(&filhos[4]);

                self.identacao -= 1;
                self.new_line();
                self.saida += "}";
                self.visit(&filhos[5]);
            }

            // parametro : var identificador identificadores ':' tipo_estendido
            RegraAST::Parametro => {
                let tipo = filhos[3].tipo(&escopos);

                self.visit(&filhos[3]);
                self.saida += " ";

                if tipo == TipoSimbolo::Cadeia { self.saida += "*" }
                self.visit(&filhos[1]);
                self.escopos.escopo_atual().inserir(&filhos[1].texto(), &tipo);

                let mut identificadores = &filhos[2];
                while *identificadores.regra() != RegraAST::Vazio {
                    let identificador = &identificadores.filhos()[0];
                    self.saida += ", ";
                    self.visit(&filhos[3]);
                    self.saida += " ";
                    if tipo == TipoSimbolo::Cadeia { self.saida += "*" }
                    self.visit(&identificador);
                    self.escopos.escopo_atual().inserir(&identificador.texto(), &tipo);
                    identificadores = &identificadores.filhos()[1];
                }
            }

            // parametros2 : ',' parametro parametros2 | <<vazio>>
            RegraAST::Parametros2 => {
                self.saida += ", ";
                self.visit(&filhos[0]);
                self.visit(&filhos[1]);
            }

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
                self.visit(&filhos[0]);
                self.visit(&filhos[1]);
                self.saida += ");";
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
                if filhos[2].tipo(&escopos) == TipoSimbolo::Cadeia {
                    self.saida += "strcpy(";
                    self.visit(&filhos[1]);
                    self.saida += ", ";
                    self.visit(&filhos[2]);
                    self.saida += ");"
                } else {
                    self.visit(&filhos[0]);
                    self.visit(&filhos[1]);
                    self.saida += " = ";
                    self.visit(&filhos[2]);
                    self.saida += ";";
                }
            }

            // cmdChamada : IDENT '(' expressao expressoes ')'
            RegraAST::CMDChamada => {
                self.new_line();
                self.visit(&filhos[0]);
                self.saida += "(";
                self.visit(&filhos[1]);
                self.visit(&filhos[2]);
                self.saida += ");";
            }

            // cmdRetorne : 'retorne' expressao
            RegraAST::CMDRetorne => {
                self.new_line();
                self.saida += "return ";
                self.visit(&filhos[0]);
                self.saida += ";";
            }

            // item_selecao : constantes ':' cmds
            RegraAST::ItemSelecao => {
                self.visit(&filhos[0]);
                self.identacao += 1;
                self.visit(&filhos[1]);
                self.new_line();
                self.saida += "break;";
                self.identacao -= 1;
            }

            // numero_intervalo : op_unario NUM_INT numero_intervalo2
            RegraAST::NumeroIntervalo => {
                let (num1, num2) = no.intervalo();
                for i in num1..num2 + 1 {
                    self.new_line();
                    self.saida += &format!("case {}:", i);
                }
            }

            // op_unario : '-' | <<vazio>>
            RegraAST::OpUnario => {
                self.saida += "-";
            }

            // op1 : '+' | '-'
            RegraAST::Op1 (token) => {
                self.saida += &format!(" {} ", token.lexema());
            }

            // op2 : '*' | '/'
            RegraAST::Op2 (token) => {
                self.saida += &format!(" {} ", token.lexema());
            }

            // op3 : '%'
            RegraAST::Op3 => {
                self.saida += " % ";
            }

            // parcela_unario : IDENT '(' expressao expressoes ')'
            RegraAST::ParcelaUnario2 => {
                self.visit(&filhos[0]);
                self.saida += "(";
                self.visit(&filhos[1]);
                self.visit(&filhos[2]);
                self.saida += ")";
            }

            //     | NUM_INT
            RegraAST::NumInt (token) => {
                self.saida += &token.lexema();
            }

            //     | NUM_REAL
            RegraAST::NumReal (token) => {
                self.saida += &token.lexema();
            }

            // parcela_nao_unario : '&' identificador
            RegraAST::ParcelaNaoUnario => {
                self.saida += "&";
                self.visit(&filhos[0]);
            }

            //     | CADEIA
            RegraAST::Cadeia (token) => {
                self.saida += &token.lexema();
            }

            // op_relacional : '=' | '<>' | '>=' | '<=' | '>' | '<'
            RegraAST::OpRelacional (token) => {
                self.saida += &match token.lexema().as_ref() {
                    "=" => " == ".to_string(),
                    "<>" => " != ".to_string(),
                    _ => format!(" {} ", token.lexema())
                }
            }

            // expressoes : ',' expressao expressoes | <<vazio>>
            RegraAST::Expressoes => {
                self.saida += ", ";
                self.visit(&filhos[0]);
                self.visit(&filhos[1]);
            }

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
