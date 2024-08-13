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
            escopos: Escopos::new(TipoSimbolo::Vazio),
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
                let tipo = variavel.tipo(&escopos);

                let escopo_atual = self.escopos.escopo_atual();

                for ident in idents {
                    let nome = ident.lexema();
                    if escopo_atual.existe(&nome) {
                        let mensagem = format!("Linha {}: identificador {} ja declarado anteriormente\n", ident.linha(), nome);
                        self.erros.push(mensagem);
                    } else {
                        escopo_atual.inserir(&nome, &tipo);

                        if let TipoSimbolo::Registro(ref atributos) = tipo {
                            for atributo in atributos {
                                let tipo = atributo.tipo(&escopos);
                                let idents = atributo.idents();
                                for ident in idents {
                                    let nome = format!("{}.{}", nome, ident.lexema());
                                    escopo_atual.inserir(&ident.lexema(), &tipo);
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
                    escopo_atual.inserir(&nome, &filhos[1].tipo(&escopos));

                    if let TipoSimbolo::Registro(atributos) = filhos[1].tipo(&escopos) {
                        for atributo in atributos {
                            let tipo = atributo.tipo(&escopos);
                            let idents = atributo.idents();
                            for ident in idents {
                                let nome = format!("{}.{}", nome, ident.lexema());
                                escopo_atual.inserir(&ident.lexema(), &tipo);
                                escopo_atual.inserir(&nome, &tipo)
                            } 
                        }
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
                let filhos = no.filhos();
                let nome = filhos[0].texto() + &filhos[1].texto();

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

            // declaracao_global :
            //     'procedimento' IDENT '(' parametros ')' declaracoes_locais cmds 'fim_procedimento' fecha_escopo
            RegraAST::DeclaracaoProcedimento => {
                let filhos = no.filhos();
                let escopos = self.escopos.clone();

                let ident = filhos[0].token().unwrap();
                let nome = ident.lexema();

                
                let escopo_externo = self.escopos.escopo_atual();
                
                if escopo_externo.existe(&nome) {
                    let mensagem = format!("Linha {}: identificador {} ja declarado anteriormente\n", ident.linha(), nome);
                    self.erros.push(mensagem);
                } else {
                    escopo_externo.inserir(&ident.lexema(), &no.tipo(&escopos));
                    
                    let tipo_retorno = TipoSimbolo::Vazio;
                    self.escopos.novo_escopo(tipo_retorno);
                    
                    if let TipoSimbolo::Procedimento(parametros) = no.tipo(&escopos) {
                        let escopo_interno = self.escopos.escopo_atual();
                        for parametro in parametros {
                            let tipo = parametro.tipo(&escopos);
                            let param_idents = parametro.idents();
                            for param_ident in param_idents {
                                if let TipoSimbolo::Registro(ref atributos) = tipo {
                                    for atributo in atributos {
                                        let tipo = atributo.tipo(&escopos);
                                        let atr_idents = atributo.idents();
                                        for atr_ident in atr_idents {
                                            let nome = format!("{}.{}", param_ident.lexema(), atr_ident.lexema());
                                            escopo_interno.inserir(&nome, &tipo)
                                        }
                                    }
                                }
                                escopo_interno.inserir(&param_ident.lexema(), &tipo);
                            } 
                        }
                    }
                }
            }

            //     | 'funcao' IDENT '(' parametros ')' ':' tipo_estendido declaracoes_locais cmds 'fim_funcao' fecha_escopo
            RegraAST::DeclaracaoFuncao => {
                let filhos = no.filhos();
                let escopos = self.escopos.clone();

                let ident = filhos[0].token().unwrap();
                let nome = ident.lexema();

                
                let escopo_externo = self.escopos.escopo_atual();
                
                if escopo_externo.existe(&nome) {
                    let mensagem = format!("Linha {}: identificador {} ja declarado anteriormente\n", ident.linha(), nome);
                    self.erros.push(mensagem);
                } else {
                    escopo_externo.inserir(&ident.lexema(), &no.tipo(&escopos));
                    
                    let tipo_retorno = filhos[2].tipo(&escopos);
                    self.escopos.novo_escopo(tipo_retorno);
                    
                    if let TipoSimbolo::Funcao { parametros, retorno: _ } = no.tipo(&escopos) {
                        let escopo_interno = self.escopos.escopo_atual();
                        for parametro in parametros {
                            let tipo = parametro.tipo(&escopos);
                            let param_idents = parametro.idents();
                            for param_ident in param_idents {
                                if let TipoSimbolo::Registro(ref atributos) = tipo {
                                    for atributo in atributos {
                                        let tipo = atributo.tipo(&escopos);
                                        let atr_idents = atributo.idents();
                                        for atr_ident in atr_idents {
                                            let nome = format!("{}.{}", param_ident.lexema(), atr_ident.lexema());
                                            escopo_interno.inserir(&nome, &tipo)
                                        }
                                    }
                                }
                                escopo_interno.inserir(&param_ident.lexema(), &tipo);
                            } 
                        }
                    }
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
                    let nome = filhos[0].texto() + &ident.texto();
                    
                    let mensagem = format!("Linha {}: atribuicao nao compativel para {}\n", no.linha(), nome);
                    self.erros.push(mensagem);
                }
            }

            RegraAST::CMDChamada => {

            }

            // cmdRetorne : 'retorne' expressao
            RegraAST::CMDRetorne => {
                if self.escopos.escopo_atual().tipo_retorno() == TipoSimbolo::Vazio {
                    let mensagem = format!("Linha {}: comando retorne nao permitido nesse escopo\n", no.linha());
                    self.erros.push(mensagem)
                }
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

            // tratando chamadas de funcoes
            //     | IDENT '(' expressao expressoes ')'
            RegraAST::ParcelaUnario2 => {
                let escopos = self.escopos.clone();
                
                let params_chamada = no.variaveis(&escopos);
                
                if let TipoSimbolo::Funcao { parametros: params_funcao, retorno: _ } = no.tipo(&escopos) {
                    if params_chamada.len() != params_funcao.len() {
                        let mensagem = format!("Linha {}: incompatibilidade de parametros na chamada de {}\n",  no.linha(), no.filhos()[0].idents()[0].lexema());
                        self.erros.push(mensagem);
                        return
                    }
                    for (param_chamada, param_funcao) in params_chamada.iter().zip(params_funcao.iter()) {
                        let tipo_chamada = if let TipoSimbolo::Funcao { parametros: _, retorno } = param_chamada.tipo(&escopos) {
                            *retorno
                        } else { param_chamada.tipo(&escopos) };
                        
                        if tipo_chamada != param_funcao.tipo(&escopos) {
                            let mensagem = format!("Linha {}: incompatibilidade de parametros na chamada de {}\n",  no.linha(), no.filhos()[0].idents()[0].lexema());
                            self.erros.push(mensagem);
                            return
                        }
                    }
                }
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

            RegraAST::FechaEscopo => {
                self.escopos.abandonar_escopo()
            }

            RegraAST::Vazio => {
            
            }
            RegraAST::Erro{mensagem: _} => {
            
            }
        }
    }
}