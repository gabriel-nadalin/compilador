use crate::semantico::tabela_de_simbolos::{Simbolo, TabelaDeSimbolos};

#[derive(Debug, Clone)]
pub struct Escopos {
    tabelas: Vec<TabelaDeSimbolos>,
}

impl Escopos {
    pub fn new() -> Self {
        let mut escopos = Self {
            tabelas: vec![],
        };
        escopos.novo_escopo();
        escopos
    }

    pub fn novo_escopo(&mut self) {
        self.tabelas.push(TabelaDeSimbolos::new())
    }

    pub fn escopo_atual(&mut self) -> &mut TabelaDeSimbolos {
        &mut self.tabelas[0]
    }

    pub fn tabelas(&self) -> Vec<TabelaDeSimbolos> {
        self.tabelas.clone()
    }

    pub fn abandonar_escopo(&mut self) {
        self.tabelas.pop();
    }

    pub fn verificar(&self, nome: &str) -> Option<Simbolo> {
        for escopo in &self.tabelas {
            if escopo.existe(nome) {
                return escopo.verificar(nome)
            }
        }
        None
    }

    pub fn existe(&self, nome: &str) -> bool {
        for escopo in &self.tabelas {
            if escopo.existe(nome) {return true}
        }
        false
    }
}