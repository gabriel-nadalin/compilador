use std::vec;

use crate::{
    lexico::token::{TipoToken, Token},
    semantico::{escopos::Escopos, tabela_de_simbolos::TipoSimbolo}
};

/// regra da gramatica que cada no da arvore sintatica representa
#[derive(Debug, Clone, PartialEq)]
pub enum RegraAST {

    // programa : declaracoes 'algoritmo' corpo 'fim_algoritmo'
    Programa,
    
    // declaracoes : declaracao declaracoes | <<vazio>>
    Declaracoes,
    // declaracao : declaracao_local | declaracao_global
    
    // declaracao_local :
    //     'declare' variavel
    DeclaracaoVariavel,
    
    //     | 'tipo' IDENT ':' tipo
    DeclaracaoTipo,
    
    //     | 'constante' IDENT ':' tipo_basico '=' valor_constante
    DeclaracaoConstante,
    
    // valor_constante : CADEIA | NUM_INT | NUM_REAL | 'verdadeiro' | 'falso'
    ValorConstante (Token),
    
    // variavel : identificador identificadores ':' tipo
    Variavel,
    // tipo : registro | tipo_estendido
    
    // identificador : IDENT identificador2 dimensao
    Identificador,

    // identificador2 : '.' IDENT identificador2 | <<vazio>>
    Identificador2,

    // identificadores: ',' identificador identificadores | <<vazio>>
    Identificadores,

    // dimensao : '[' exp_aritmetica ']' dimensao | <<vazio>>
    Dimensao,

    // tipo_estendido : circunflexo tipo_basico_ident
    TipoExtendido,
    // tipo_basico_ident : tipo_basico
    //    | IDENT
    Ident (Token),

    // tipo_basico : 'literal' | 'inteiro' | 'real' | 'logico'
    TipoBasico (Token),
    
    // circunflexo: '^' | <<vazio>>
    Circunflexo,

    // registro : 'registro' variaveis 'fim_Registro' fecha_escopo
    Registro,

    // variaveis : variavel variaveis | <<vazio>>
    Variaveis,

    // declaracao_global :
    //     'procedimento' IDENT '(' parametros ')' declaracoes_locais cmds 'fim_procedimento' fecha_escopo
    DeclaracaoProcedimento,

    //     | 'funcao' IDENT '(' parametros ')' ':' tipo_estendido declaracoes_locais cmds 'fim_funcao' fecha_escopo
    DeclaracaoFuncao,

    // declaracoes_locais : declaracao_local declaracoes_locais | <<vazio>>
    DeclaracoesLocais,

    // parametro : var identificador identificadores ':' tipo_estendido
    Parametro,

    // parametros : parametro parametros2 | <<vazio>>
    Parametros,
    
    // parametros2 : ',' parametro parametros2 | <<vazio>>
    Parametros2,

    // var : 'var' | <<vazio>>
    Var,

    // corpo : declaracoes_locais cmds
    Corpo,
    
    // cmds : cmd cmds | <<vazio>>
    CMDs,
    // cmd : cmdLeia | cmdEscreva | cmdSe | cmdCaso | cmdPara | cmdEnquanto
    //     | cmdFaca | cmdAtribuicao | cmdChamada | cmdRetorne

    // cmdLeia : 'leia' '(' circunflexo identificador cmdLeia2 ')'
    CMDLeia,

    // cmdLeia2 : ',' circunflexo identificador cmdLeia2 | <<vazio>>
    CMDLeia2,

    // cmdEscreva : 'escreva' '(' expressao expressoes ')'
    CMDEscreva,

    // cmdSe : 'se' expressao 'entao' cmds senao 'fim_se'
    CMDSe,

    // senao : 'senao' cmds | <<vazio>>
    Senao,

    // cmdCaso : 'caso' exp_aritmetica 'seja' selecao senao 'fim_caso'
    CMDCaso,

    // cmdPara : 'para' IDENT '<-' exp_aritmetica 'ate' exp_aritmetica 'faca' cmds 'fim_para'
    CMDPara,

    // cmdEnquanto : 'enquanto' expressao 'faca' cmds 'fim_enquanto'
    CMDEnquanto,

    // cmdFaca : 'faca' cmds 'ate' expressao
    CMDFaca,

    // cmdAtribuicao : circunflexo identificador '<-' expressao
    CMDAtribuicao,

    // cmdChamada : IDENT '(' expressao expressoes ')'
    CMDChamada,

    // cmdRetorne : 'retorne' expressao
    CMDRetorne,

    // selecao : item_selecao selecao | <<vazio>>
    Selecao,

    // item_selecao : constantes ':' cmds
    ItemSelecao,

    // constantes : numero_intervalo numero_intervalos
    Constantes,

    // numero_intervalo : op_unario NUM_INT numero_intervalo2
    NumeroIntervalo,
    
    // numero_intervalos : ',' numero_intervalo numero_intervalos | <<vazio>>
    NumeroIntervalos,

    // numero_intervalo2 : '..' op_unario NUM_INT | <<vazio>>
    NumeroIntervalo2,

    // op_unario : '-' | <<vazio>>
    OpUnario,

    // exp_aritmetica : termo termos
    ExpAritmetica,

    // termo : fator fatores
    Termo,

    // termos : op1 termo termos | <<vazio>>
    Termos,

    // op1 : '+' | '-'
    Op1 (Token),

    // fator : parcela parcelas
    Fator,

    // fatores : op2 fator fatores | <<vazio>>
    Fatores,
    
    // op2 : '*' | '/'
    Op2 (Token),
    
    // parcela : op_unario parcela_unario | parcela_nao_unario
    Parcela,
    
    // parcelas : op3 parcela parcelas | <<vazio>>
    Parcelas,

    // op3 : '%'
    Op3,
    
    // parcela_unario : circunflexo identificador
    ParcelaUnario1,

    //     | IDENT '(' expressao expressoes ')'
    ParcelaUnario2,

    //     | '(' expressao ')'
    ParcelaUnario3,

    //     | NUM_INT
    NumInt (Token),

    //     | NUM_REAL
    NumReal (Token),

    // parcela_nao_unario : '&' identificador
    ParcelaNaoUnario,

    //     | CADEIA
    Cadeia (Token),

    // exp_relacional : exp_aritmetica exp_relacional2
    ExpRelacional,

    // exp_relacional2 : op_relacional exp_aritmetica | <<vazio>>
    ExpRelacional2,

    // op_relacional : '=' | '<>' | '>=' | '<=' | '>' | '<'
    OpRelacional (Token),
    
    // expressao : termo_logico termos_logicos
    Expressao,

    // expressoes : ',' expressao expressoes | <<vazio>>
    Expressoes,

    // termo_logico : fator_logico fatores_logicos
    TermoLogico,

    // termos_logicos : op_logico_1 termo_logico termos_logicos | <<vazio>>
    TermosLogicos,

    // fator_logico : nao parcela_logica
    FatorLogico,
    // parcela_logica : constante_logica
    //     | exp_relacional
    
    // fatores_logicos : op_logico_2 fator_logico fatores_logicos | <<vazio>>
    FatoresLogicos,
    
    // nao : 'nao' | <<vazio>>
    Nao,
    
    // constante_logica : 'verdadeiro' | 'falso'
    ConstanteLogica(Token),
    
    // op_logico_1 : 'ou'
    OpLogico1,

    // op_logico_2 : 'e'
    OpLogico2,

    // fecha_escopo : <<vazio>>
    FechaEscopo,

    Vazio,
    Erro { mensagem: String },
}

/// estrutura generica para um no da arvore sintatica\
/// armazena a regra da gramatica que representa e um vetor com os nos filhos
#[derive(Debug, Clone, PartialEq)]
pub struct NoAST {
    regra: RegraAST,
    filhos: Vec<NoAST>,
}


impl NoAST {

    /// retorna novo no
    pub fn new(regra: RegraAST, filhos: Vec<NoAST>) -> Self {
        Self {
            regra,
            filhos,
        }
    }

    /// retorna no folha
    pub fn new_folha(regra: RegraAST) -> Self {
        Self {
            regra,
            filhos: vec![],
        }
    }

    /// retorna no vazio\
    /// representa a ausencia de no filho
    pub fn vazio() -> Self {
        Self {
            regra: RegraAST::Vazio,
            filhos: vec![],
        }
    }

    /// retorna regra
    pub fn regra(&self) -> &RegraAST {
        &self.regra
    }

    /// retorna referencia para vetor de filhos
    pub fn filhos(&self) -> &Vec<NoAST> {
        &self.filhos
    }

    /// verifica se `self` representa um erro sintatico
    pub fn is_erro(&self) -> bool {
        if let RegraAST::Erro { mensagem: _ } = self.regra {
            true
        } else {
            false
        }
    }

    /// retorna mensagem de erro caso `self` represente um erro sintatico
    pub fn get_erro(&self) -> Option<String> {
        if let RegraAST::Erro { mensagem } = &self.regra {
            Some(mensagem.to_string())
        } else {
            None
        }
    }

    /// retorna token caso `self` seja um no folha, `None` caso contrario
    pub fn token(&self) -> Option<Token> {
        match &self.regra {
            RegraAST::ValorConstante (token)
            | RegraAST::Ident (token)
            | RegraAST::TipoBasico (token)
            | RegraAST::Op1 (token)
            | RegraAST::Op2 (token)
            | RegraAST::NumInt (token)
            | RegraAST::NumReal (token)
            | RegraAST::Cadeia (token)
            | RegraAST::OpRelacional (token) => Some(token.clone()),
            _ => None
        }
    }

    /// retorna linha do no recursivamente
    pub fn linha(&self) -> u32 {
        match &self.regra {
            RegraAST::ValorConstante (token)
            | RegraAST::Ident (token)
            | RegraAST::TipoBasico (token)
            | RegraAST::Op1 (token)
            | RegraAST::Op2 (token)
            | RegraAST::NumInt (token)
            | RegraAST::NumReal (token)
            | RegraAST::Cadeia (token)
            | RegraAST::OpRelacional (token) => token.linha(),
            _ => {
                if self.filhos.len() > 0 {
                    for filho in &self.filhos {
                        if filho.linha() > 0 { return filho.linha()}
                    }
                    0
                }
                else {0}
            }
        }
    }
    
    /// retorna todos os identificadores presentes em filhos do no recursivamente
    pub fn idents(&self) -> Vec<Token> {
        match &self.regra {
            RegraAST::Ident (token) => vec![token.clone()],
            RegraAST::TipoExtendido
            | RegraAST::Registro
            | RegraAST::Dimensao => vec![],
            _ => {
                let mut idents = vec![];
                for filho in &self.filhos {
                    idents.append(&mut filho.idents());
                }
                idents
            }
        }
    }

    /// retorna atributos de um registro ou parametros de uma funcao ou procedimento recursivamente
    pub fn variaveis(&self, escopos: &Escopos) -> Vec<NoAST> {
        match &self.regra {
            RegraAST::Variavel => vec![self.clone()],
            RegraAST::Variaveis => {
                let mut vars = self.filhos[1].variaveis(escopos);
                vars.push(self.filhos[0].clone());
                vars
            },
            RegraAST::Registro => self.filhos[0].variaveis(escopos),
            
            RegraAST::Parametro => vec![self.clone()],
            RegraAST::Parametros => {
                let mut params = self.filhos[1].variaveis(escopos);
                params.push(self.filhos[0].clone());
                params
            },
            RegraAST::Parametros2 => {
                let mut params = self.filhos[1].variaveis(escopos);
                params.push(self.filhos[0].clone());
                params
            },
            RegraAST::DeclaracaoFuncao
            | RegraAST::DeclaracaoProcedimento => self.filhos[1].variaveis(escopos),
            
            RegraAST::ParcelaUnario2 => {
                let mut exps = self.filhos[2].variaveis(escopos);
                exps.push(self.filhos[1].clone());
                exps
            },
            RegraAST::Expressoes => {
                let mut exps = self.filhos[1].variaveis(escopos);
                exps.push(self.filhos[0].clone());
                exps
            }

            RegraAST::Ident(ident) => {
                let nome = ident.lexema();
                if escopos.existe(&nome) {
                    let tipo = escopos.verificar(&nome).unwrap().tipo();
                    match tipo {
                        TipoSimbolo::Funcao { parametros, retorno: _ } => parametros,
                        TipoSimbolo::Procedimento(parametros) => parametros,
                        TipoSimbolo::Registro(atributos) => atributos,
                        _ => vec![]
                    }
                } else {
                    vec![]
                }
            }

            _ => vec![]
        }
    }

    /// retorna tipo do no recursivamente
    pub fn tipo(&self, escopos: &Escopos) -> TipoSimbolo {
        match &self.regra {
            RegraAST::ConstanteLogica(_token) => TipoSimbolo::Logico,
            RegraAST::Cadeia(_token) => TipoSimbolo::Cadeia,
            RegraAST::NumInt(_token) => TipoSimbolo::Inteiro,
            RegraAST::NumReal(_token) => TipoSimbolo::Real,
            RegraAST::Registro => TipoSimbolo::Registro(self.variaveis(escopos)),
            RegraAST::DeclaracaoProcedimento => TipoSimbolo::Procedimento(self.variaveis(escopos)),
            RegraAST::DeclaracaoFuncao => TipoSimbolo::Funcao {
                parametros: self.variaveis(escopos),
                retorno: Box::new(self.filhos[2].tipo(escopos)),
            },
            RegraAST::TipoBasico (token) => {
                match token.tipo() {
                    TipoToken::PCliteral => TipoSimbolo::Cadeia,
                    TipoToken::PCinteiro => TipoSimbolo::Inteiro,
                    TipoToken::PCreal => TipoSimbolo::Real,
                    TipoToken::PClogico => TipoSimbolo::Logico,
                    _ => TipoSimbolo::Vazio
                }
            },

            RegraAST::ParcelaNaoUnario => {
                TipoSimbolo::Ponteiro(Box::new(self.filhos[0].tipo(escopos)))
            }
            
            RegraAST::ParcelaUnario2
            | RegraAST::ParcelaUnario3 => self.filhos[0].tipo(escopos),

            RegraAST::FatorLogico
            | RegraAST::ParcelaUnario1
            | RegraAST::ExpRelacional2 => self.filhos[1].tipo(escopos),

            RegraAST::Variavel => self.filhos[2].tipo(escopos),

            RegraAST::Parametro => self.filhos[3].tipo(escopos),

            RegraAST::TipoExtendido => {
                let filhos = &self.filhos;
                if let RegraAST::Circunflexo = filhos[0].regra() {
                    TipoSimbolo::Ponteiro(Box::new(filhos[1].tipo(escopos)))
                } else {
                    filhos[1].tipo(escopos)
                }
            }

            RegraAST::Expressao
            | RegraAST::TermoLogico
            | RegraAST::ExpAritmetica
            | RegraAST::Termo
            | RegraAST::Fator => {
                if self.filhos.is_empty() {
                    return TipoSimbolo::Vazio;
                }

                let tipo1 = if let TipoSimbolo::Funcao { parametros: _, retorno } = self.filhos[0].tipo(escopos) {
                    *retorno
                } else { self.filhos[0].tipo(escopos) };

                let tipo2 = if let TipoSimbolo::Funcao { parametros: _, retorno} = self.filhos[1].tipo(escopos) {
                    *retorno
                } else { self.filhos[1].tipo(escopos) };

                if tipo1 == tipo2 || ((tipo1 == TipoSimbolo::Real || tipo1 == TipoSimbolo::Inteiro) && (tipo2 == TipoSimbolo::Real || tipo2 == TipoSimbolo::Inteiro)) || tipo2 == TipoSimbolo::Vazio {
                    tipo1
                } else {
                    TipoSimbolo::Invalido
                }
            }

            RegraAST::Expressoes
            | RegraAST::TermosLogicos
            | RegraAST::FatoresLogicos
            | RegraAST::Termos
            | RegraAST::Fatores
            | RegraAST::Parcelas => {
                if self.filhos.is_empty() {
                    return TipoSimbolo::Vazio;
                }

                let tipo1 = if let TipoSimbolo::Funcao { parametros: _, retorno } = self.filhos[1].tipo(escopos) {
                    *retorno
                } else { self.filhos[1].tipo(escopos) };

                let tipo2 = if let TipoSimbolo::Funcao { parametros: _, retorno } = self.filhos[2].tipo(escopos) {
                    *retorno
                } else { self.filhos[2].tipo(escopos) };
                
                if tipo1 == tipo2 || ((tipo1 == TipoSimbolo::Real || tipo1 == TipoSimbolo::Inteiro) && (tipo2 == TipoSimbolo::Real || tipo2 == TipoSimbolo::Inteiro)) || tipo2 == TipoSimbolo::Vazio {
                    tipo1
                } else {
                    TipoSimbolo::Invalido
                }
            }

            RegraAST::ExpRelacional => {
                if self.filhos.is_empty() {
                    return TipoSimbolo::Vazio;
                }

                let tipo1 = if let TipoSimbolo::Funcao { parametros: _, retorno } = self.filhos[0].tipo(escopos) {
                    *retorno
                } else { self.filhos[0].tipo(escopos) };

                let tipo2 = if let TipoSimbolo::Funcao { parametros: _, retorno } = self.filhos[1].tipo(escopos) {
                    *retorno
                } else { self.filhos[1].tipo(escopos) };
                
                if tipo2 == TipoSimbolo::Vazio {
                    tipo1
                } else if tipo1 == tipo2 || ((tipo1 == TipoSimbolo::Real || tipo1 == TipoSimbolo::Inteiro) && (tipo2 == TipoSimbolo::Real || tipo2 == TipoSimbolo::Inteiro)) || tipo2 == TipoSimbolo::Vazio {
                    TipoSimbolo::Logico
                } else {
                    TipoSimbolo::Invalido
                }
            }

            RegraAST::Parcela => {
                if self.filhos.len() == 1 {
                    self.filhos[0].tipo(escopos)
                } else {
                    self.filhos[1].tipo(escopos)
                }
            }

            RegraAST::Identificador => {
                let nome = format!("{}{}", self.filhos[0].texto(), self.filhos[1].texto());
                if escopos.existe(&nome) {
                    escopos.verificar(&nome).unwrap().tipo()
                } else {
                    TipoSimbolo::Invalido
                }
            }

            RegraAST::Ident(ident) => {
                let nome = ident.lexema();
                if escopos.existe(&nome) {
                    escopos.verificar(&nome).unwrap().tipo()
                } else {
                    TipoSimbolo::Invalido
                }
            }
            
            _ => TipoSimbolo::Vazio
        }
    }

    /// retorna texto do no recursivamente
    pub fn texto(&self) -> String {
        match &self.regra {
            RegraAST::ValorConstante (token)
            | RegraAST::Ident (token)
            | RegraAST::TipoBasico (token)
            | RegraAST::Op1 (token)
            | RegraAST::Op2 (token)
            | RegraAST::NumInt (token)
            | RegraAST::NumReal (token)
            | RegraAST::Cadeia (token)
            | RegraAST::OpRelacional (token) => token.lexema(),
            RegraAST::Identificador2 => format!(".{}{}", self.filhos[0].texto(), self.filhos[1].texto()),
            RegraAST::Identificadores
            | RegraAST::Expressoes => format!(", {}{}", self.filhos[0].texto(), self.filhos[1].texto()),
            RegraAST::Circunflexo => "^".to_string(),
            RegraAST::Dimensao => format!("[{}]{}", self.filhos[0].texto(), self.filhos[1].texto()),
            _ => {
                let mut texto = "".to_string();
                for filho in &self.filhos {
                    texto += &filho.texto()
                }
                texto
            }
        }
    }
}