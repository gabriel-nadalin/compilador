use crate::semantico::tabela_de_simbolos::{Simbolo, TabelaDeSimbolos};

/// vetor de tabelas de simbolos, cada uma representando um escopo
#[derive(Debug, Clone)]
pub struct Escopos {
    tabelas: Vec<TabelaDeSimbolos>,
}

impl Escopos {

    /// retorna instancia de escopos
    pub fn new() -> Self {
        let mut escopos = Self {
            tabelas: vec![],
        };
        escopos.novo_escopo();
        escopos
    }

    /// adiciona escopo
    pub fn novo_escopo(&mut self) {
        self.tabelas.push(TabelaDeSimbolos::new())
    }

    /// retorna escopo atual
    pub fn escopo_atual(&mut self) -> &mut TabelaDeSimbolos {
        &mut self.tabelas[0]
    }

    /// retorna todos os escopos atualmente existentes
    pub fn tabelas(&self) -> Vec<TabelaDeSimbolos> {
        self.tabelas.clone()
    }

    /// remove escopo mais recente
    pub fn abandonar_escopo(&mut self) {
        self.tabelas.pop();
    }

    /// retorna primeiro simbolo com identificador `nome`, caso exista em algum escopo
    pub fn verificar(&self, nome: &str) -> Option<Simbolo> {
        for escopo in &self.tabelas {
            if escopo.existe(nome) {
                return escopo.verificar(nome)
            }
        }
        None
    }

    /// verifica se identificador `nome` existe em algum escopo
    pub fn existe(&self, nome: &str) -> bool {
        for escopo in &self.tabelas {
            if escopo.existe(nome) {return true}
        }
        false
    }
}