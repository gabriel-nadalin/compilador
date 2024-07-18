pub mod visitor;
pub mod tabela_de_simbolos;

use tabela_de_simbolos::{TabelaDeSimbolos, TipoSimbolo};
use visitor::Visitor;

use crate::sintatico::arvore_sintatica::NoAST;

pub struct Semantico {
    tabela_de_simbolos: TabelaDeSimbolos,
    erros: Vec<String>,
}

impl Semantico {
    pub fn new() -> Self {
        Self {
            tabela_de_simbolos: TabelaDeSimbolos::new(),
            erros: vec![],
        }
    }

    pub fn get_erros(&self) -> Vec<String> {
        self.erros.clone()
    }
}

impl Visitor for Semantico {
    fn visit(&mut self, no: &NoAST) {
        match no {
            NoAST::Programa { declaracoes, corpo } => {

            }
            NoAST::Declaracoes { declaracao, declaracoes } => {

            }
            NoAST::DeclaracaoVariavel { variavel } => {
                let idents = variavel.get_idents();
                // let tipo = variavel.get_tipo();
                for ident in idents {
                    let nome = ident.lexema();
                    if self.tabela_de_simbolos.existe(&nome) {
                        let mensagem = format!("Linha {}: identificador {} ja declarado anteriormente\n", ident.linha(), nome);
                        self.erros.push(mensagem);
                    } else {
                        // self.tabela_de_simbolos.inserir(&nome, &tipo)
                    }
                }
            }
            NoAST::DeclaracaoTipo { ident, tipo } => {

            }
            NoAST::DeclaracaoConstante { ident, tipo_basico, valor_constante } => {
                let idents = ident.get_tokens();
                let tipo = match tipo_basico.get_tokens()[0].lexema().as_ref() {
                    "literal" => TipoSimbolo::Cadeia,
                    "inteiro" => TipoSimbolo::Inteiro,
                    "real" => TipoSimbolo::Real,
                    "logico" => TipoSimbolo::Logico,
                    _ => TipoSimbolo::Invalido
                };
                for ident in idents {
                    let nome = ident.lexema();
                    if self.tabela_de_simbolos.existe(&nome) {
                        let mensagem = format!("Linha {}: identificador {} ja declarado anteriormente\n", ident.linha(), nome);
                        self.erros.push(mensagem);
                    } else {
                        self.tabela_de_simbolos.inserir(&nome, &tipo)
                    }
                }
            }
            NoAST::ValorConstante (Token) => {
            
            }
            NoAST::Variavel { identificador, identificadores, tipo } => {

            }
            NoAST::Identificador { ident, identificador2, dimensao } => {

            }
            NoAST::Identificador2 { ident, identificador2 } => {

            }
            NoAST::Identificadores { identificador, identificadores } => {

            }
            NoAST::Dimensao { exp_aritmetica, dimensao } => {

            }
            NoAST::TipoExtendido { circunflexo, tipo_basico_ident } => {

            }
            NoAST::Ident (Token) => {
            
            }
            NoAST::TipoBasico (Token) => {
            
            }
            NoAST::Circunflexo => {
            
            }
            NoAST::Registro { variaveis } => {

            }
            NoAST::Variaveis { variavel, variaveis } => {

            }
            NoAST::DeclaracaoProcedimento { ident, parametros, declaracoes_locais, cmds } => {

            }
            NoAST::DeclaracaoFuncao { ident, parametros, tipo_estendido, declaracoes_locais, cmds } => {

            }
            NoAST::DeclaracoesLocais { declaracao_local, declaracoes_locais } => {

            }
            NoAST::Parametro { var, identificador, identificadores, tipo_estendido } => {

            }
            NoAST::Parametros { parametro, parametros2 } => {

            }
            NoAST::Parametros2 { parametro, parametros2 } => {

            }
            NoAST::Var => {
            
            }
            NoAST::Corpo { declaracoes_locais, cmds } => {

            }
            NoAST::CMDs { cmd, cmds } => {

            }
            NoAST::CMDLeia { circunflexo, identificador, cmd_leia2 } => {

            }
            NoAST::CMDLeia2 { circunflexo, identificador, cmd_leia2 } => {

            }
            NoAST::CMDEscreva { expressao, expressoes } => {

            }
            NoAST::CMDSe { expressao, cmds, senao } => {

            }
            NoAST::Senao { cmds } => {

            }
            NoAST::CMDCaso { exp_aritmetica, selecao, senao } => {

            }
            NoAST::CMDPara { ident, exp_aritmetica_1, exp_aritmetica_2, cmds } => {

            }
            NoAST::CMDEnquanto { expressao, cmds } => {

            }
            NoAST::CMDFaca { cmds, expressao } => {

            }
            NoAST::CMDAtribuicao { circunflexo, identificador, expressao } => {

            }
            NoAST::CMDChamada { ident, expressao, expressoes } => {

            }
            NoAST::CMDRetorne { expressao } => {

            }
            NoAST::Selecao { item_selecao, selecao } => {

            }
            NoAST::ItemSelecao { constantes, cmds } => {

            }
            NoAST::Constantes { numero_intervalo, numero_intervalos } => {

            }
            NoAST::NumeroIntervalo { op_unario, num_int, numero_intervalo2 } => {

            }
            NoAST::NumeroIntervalos { numero_intervalo, numero_intervalos } => {

            }
            NoAST::NumeroIntervalo2 { op_unario, num_int } => {

            }
            NoAST::OpUnario => {
            
            }
            NoAST::ExpAritmetica { termo, termos } => {

            }
            NoAST::Termo { fator, fatores } => {

            }
            NoAST::Termos { op1, termo, termos } => {

            }
            NoAST::Op1 (Token) => {
            
            }
            NoAST::Fator { parcela, parcelas } => {

            }
            NoAST::Fatores { op2, fator, fatores } => {

            }
            NoAST::Op2 (Token) => {
            
            }
            NoAST::Parcela { op_unario, parcela_unario } => {

            }
            NoAST::Parcelas { op3, parcela, parcelas } => {

            }
            NoAST::Op3 => {
            
            }
            NoAST::ParcelaUnario1 { circunflexo, identificador } => {

            }
            NoAST::ParcelaUnario2 { ident, expressao, expressoes } => {

            }
            NoAST::ParcelaUnario3 { expressao } => {

            }
            NoAST::NumInt (Token) => {
            
            }
            NoAST::NumReal (Token) => {
            
            }
            NoAST::ParcelaNaoUnario { identificador } => {

            }
            NoAST::Cadeia (Token) => {
            
            }
            NoAST::ExpRelacional { exp_aritmetica, exp_relacional2 } => {

            }
            NoAST::ExpRelacional2 { op_relacional, exp_aritmetica } => {

            }
            NoAST::OpRelacional (Token) => {
            
            }
            NoAST::Expressao { termo_logico, termos_logicos } => {

            }
            NoAST::Expressoes { expressao, expressoes } => {

            }
            NoAST::TermoLogico { fator_logico, fatores_logicos } => {

            }
            NoAST::TermosLogicos { op_logico_1, termo_logico, termos_logicos } => {

            }
            NoAST::FatorLogico { nao, parcela_logica } => {

            }
            NoAST::FatoresLogicos { op_logico_2, fator_logico, fatores_logicos } => {

            }
            NoAST::Nao => {
            
            }
            NoAST::ConstanteLogica(Token) => {
            
            }
            NoAST::OpLogico1 => {
            
            }
            NoAST::OpLogico2 => {
            
            }
            NoAST::Vazio => {
            
            }
            NoAST::Erro { mensagem: String } => {
            
            }
        }
    }
}