pub mod visitor;
pub mod tabela_de_simbolos;
pub mod escopos;

use escopos::Escopos;
use tabela_de_simbolos::TipoSimbolo;
use visitor::Visitor;

use crate::sintatico::arvore_sintatica::{NoAST, RegraAST};

/// analisador semantico
pub struct Semantico {
    escopos: Escopos,
    erros: Vec<String>,
}

impl Semantico {

    /// retorna instancia de analisador semantico
    pub fn new() -> Self {
        Self {
            escopos: Escopos::new(),
            erros: vec![],
        }
    }

    /// retorna vetor de erros semanticos
    pub fn get_erros(&self) -> Vec<String> {
        self.erros.clone()
    }
}

impl Visitor for Semantico {

    /// verifica um no da arvore sintatica segundo seus requisitos semanticos especificos
    fn visit(&mut self, no: &NoAST) {
        match no.regra() {
            RegraAST::Programa => {

            }
            RegraAST::Declaracoes => {

            }

            // declaracao_local :
            //     'declare' variavel
            RegraAST::DeclaracaoVariavel => {
                let escopos = self.escopos.clone();

                let variavel = &no.filhos()[0];
                let idents = variavel.idents();
                let tipo = variavel.tipo(&self.escopos.clone());

                let escopo_atual = self.escopos.escopo_atual();
                for ident in idents {
                    let nome = ident.lexema();
                    if escopo_atual.existe(&nome) {
                        let mensagem = format!("Linha {}: identificador {} ja declarado anteriormente\n", ident.linha(), nome);
                        self.erros.push(mensagem);
                    } else {
                        escopo_atual.inserir(&nome, &tipo);
                        if let TipoSimbolo::Registro = variavel.tipo(&escopos) {
                            println!("{:?}", variavel.tipo(&escopos));
                            let atributos = variavel.filhos()[2].atributos();
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
            }

            //     | 'tipo' IDENT ':' tipo
            RegraAST::DeclaracaoTipo => {
                let filhos = no.filhos();
                let escopos = self.escopos.clone();

                let ident = filhos[0].token().unwrap();
                let nome = ident.lexema();

                let escopo_atual = self.escopos.escopo_atual();

                if escopo_atual.existe(&nome) {
                    let mensagem = format!("Linha {}: identificador {} ja declarado anteriormente\n", ident.linha(), nome);
                    self.erros.push(mensagem);
                } else {
                    if let RegraAST::Registro = filhos[1].regra() {
                        println!("aqui!");
                        let atributos = filhos[1].atributos();
                        for atributo in atributos {
                            let tipo = atributo.tipo(&escopos);
                            let idents = atributo.idents();
                            for ident in idents {
                                let nome = format!("{}.{}", nome, ident.lexema());
                                println!("{nome}");
                                escopo_atual.inserir(&nome, &tipo)
                            }
                        }
                    } else {
                        let tipo = filhos[1].tipo(&escopos);
                        escopo_atual.inserir(&nome, &tipo)
                    }
                }

            }

            //     | 'constante' IDENT ':' tipo_basico '=' valor_constante
            RegraAST::DeclaracaoConstante => {
                let filhos = no.filhos();
                let escopos = self.escopos.clone();

                let ident = filhos[0].token().unwrap();
                let nome = ident.lexema();

                let escopo_atual = self.escopos.escopo_atual();

                if escopo_atual.existe(&nome) {
                    let mensagem = format!("Linha {}: identificador {} ja declarado anteriormente\n", ident.linha(), nome);
                    self.erros.push(mensagem);
                } else {
                    let mut tipo = filhos[1].tipo(&escopos);
                    if let TipoSimbolo::Vazio = tipo {
                        tipo = TipoSimbolo::Inteiro
                    }
                    escopo_atual.inserir(&nome, &tipo)
                }
            }

            RegraAST::ValorConstante(_) => {
            
            }
            RegraAST::Variavel => {

            }

            // identificador : IDENT identificador2 dimensao
            RegraAST::Identificador => {
                let nome = no.idents()
                    .iter()
                    .map(|token| token.lexema().to_string())
                    .collect::<Vec<String>>()
                    .join(".");

                if !self.escopos.existe(&nome) {
                    let ident = no.filhos()[0].token().unwrap();
                    let mensagem = format!("Linha {}: identificador {} nao declarado\n", ident.linha(), nome);
                    self.erros.push(mensagem);
                }
            }
            RegraAST::Identificador2 => {

            }
            RegraAST::Identificadores => {

            }
            RegraAST::Dimensao => {

            }

            // tipo_estendido : circunflexo tipo_basico_ident
            RegraAST::TipoExtendido => {
                let filhos = no.filhos();
                if let RegraAST::Ident(ident) = filhos[1].regra() {
                    let nome = ident.lexema();
                    if !self.escopos.existe(&nome) {
                        let mensagem = format!("Linha {}: tipo {} nao declarado\n", ident.linha(), nome);
                        self.erros.push(mensagem);
                    }
                }

            }
            RegraAST::Ident(_) => {
            
            }
            RegraAST::TipoBasico(_) => {
            
            }
            RegraAST::Circunflexo => {
            
            }
            RegraAST::Registro => {
                
            }
            RegraAST::Variaveis => {

            }

            //     'procedimento' IDENT '(' parametros ')' declaracoes_locais cmds 'fim_procedimento'
            RegraAST::DeclaracaoProcedimento => {
                let filhos = no.filhos();
                let escopos = self.escopos.clone();

                let ident = filhos[0].token().unwrap();
                let nome = ident.lexema();

                let escopo_atual = self.escopos.escopo_atual();

                if escopo_atual.existe(&nome) {
                    let mensagem = format!("Linha {}: identificador {} ja declarado anteriormente\n", ident.linha(), nome);
                    self.erros.push(mensagem);
                } else {
                    let tipo = TipoSimbolo::Vazio;
                    escopo_atual.inserir(&nome, &tipo)
                }
            }

            //     | 'funcao' IDENT '(' parametros ')' ':' tipo_estendido declaracoes_locais cmds 'fim_funcao'
            RegraAST::DeclaracaoFuncao => {
                let filhos = no.filhos();
                let escopos = self.escopos.clone();

                let ident = filhos[0].token().unwrap();
                let nome = ident.lexema();

                let escopo_atual = self.escopos.escopo_atual();

                if escopo_atual.existe(&nome) {
                    let mensagem = format!("Linha {}: identificador {} ja declarado anteriormente\n", ident.linha(), nome);
                    self.erros.push(mensagem);
                } else {
                    let tipo = filhos[2].tipo(&escopos);
                    escopo_atual.inserir(&nome, &tipo)
                }

            }
            RegraAST::DeclaracoesLocais => {

            }
            RegraAST::Parametro => {

            }
            RegraAST::Parametros => {

            }
            RegraAST::Parametros2 => {

            }
            RegraAST::Var => {
            
            }
            RegraAST::Corpo => {

            }
            RegraAST::CMDs => {

            }
            RegraAST::CMDLeia => {

            }
            RegraAST::CMDLeia2 => {

            }
            RegraAST::CMDEscreva => {

            }
            RegraAST::CMDSe => {

            }
            RegraAST::Senao => {

            }
            RegraAST::CMDCaso => {

            }
            RegraAST::CMDPara => {

            }
            RegraAST::CMDEnquanto => {

            }
            RegraAST::CMDFaca => {

            }

            // cmdAtribuicao : circunflexo identificador '<-' expressao
            RegraAST::CMDAtribuicao => {
                let filhos = no.filhos();
                
                let ident = &filhos[1];
                let expressao = &filhos[2];

                let tipo_ident = ident.tipo(&self.escopos);
                let tipo_exp = expressao.tipo(&self.escopos);
                
                if (tipo_exp == TipoSimbolo::Real || tipo_exp == TipoSimbolo::Inteiro) && (tipo_ident == TipoSimbolo::Real || tipo_ident == TipoSimbolo::Inteiro) {
                    
                } else if tipo_exp != tipo_ident && tipo_ident != TipoSimbolo::Invalido {
                    let mut nome = if let RegraAST::Circunflexo = filhos[0].regra() {
                        "^".to_string()
                    } else {
                        "".to_string()
                    };
                    // constroi o nome do identificador 
                    nome += &filhos[1].idents()
                        .iter()
                        .map(|token| token.lexema().to_string())
                        .collect::<Vec<String>>()
                        .join(".");
                    
                    let mensagem = format!("Linha {}: atribuicao nao compativel para {}\n", no.linha(), nome);
                    self.erros.push(mensagem);
                }
            }
            RegraAST::CMDChamada => {

            }
            RegraAST::CMDRetorne => {

            }
            RegraAST::Selecao => {

            }
            RegraAST::ItemSelecao => {

            }
            RegraAST::Constantes => {

            }
            RegraAST::NumeroIntervalo => {

            }
            RegraAST::NumeroIntervalos => {

            }
            RegraAST::NumeroIntervalo2 => {

            }
            RegraAST::OpUnario => {
            
            }
            RegraAST::ExpAritmetica => {

            }
            RegraAST::Termo => {

            }
            RegraAST::Termos => {

            }
            RegraAST::Op1(_) => {
            
            }
            RegraAST::Fator => {

            }
            RegraAST::Fatores => {

            }
            RegraAST::Op2(_) => {
            
            }
            RegraAST::Parcela => {

            }
            RegraAST::Parcelas => {

            }
            RegraAST::Op3 => {
            
            }
            RegraAST::ParcelaUnario1 => {

            }
            RegraAST::ParcelaUnario2 => {

            }
            RegraAST::ParcelaUnario3 => {

            }
            RegraAST::NumInt(_) => {
            
            }
            RegraAST::NumReal(_) => {
            
            }
            RegraAST::ParcelaNaoUnario => {

            }
            RegraAST::Cadeia(_) => {
            
            }
            RegraAST::ExpRelacional => {

            }
            RegraAST::ExpRelacional2 => {

            }
            RegraAST::OpRelacional(_) => {
            
            }
            RegraAST::Expressao => {

            }
            RegraAST::Expressoes => {

            }
            RegraAST::TermoLogico => {

            }
            RegraAST::TermosLogicos => {

            }
            RegraAST::FatorLogico => {

            }
            RegraAST::FatoresLogicos => {

            }
            RegraAST::Nao => {
            
            }
            RegraAST::ConstanteLogica(_) => {
            
            }
            RegraAST::OpLogico1 => {
            
            }
            RegraAST::OpLogico2 => {
            
            }
            RegraAST::Vazio => {
            
            }
            RegraAST::Erro{mensagem: _} => {
            
            }
        }
    }
}
