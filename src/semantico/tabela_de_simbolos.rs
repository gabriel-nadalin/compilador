use std::collections::HashMap;

/// representa o tipo de um simbolo da linguagem LA
#[derive(Debug, Clone, PartialEq)]
pub enum TipoSimbolo {
    Cadeia,
    Real,
    Inteiro,
    Logico,
    Registro,
    Ponteiro(Box<TipoSimbolo>),
    Vazio,
    Invalido,
}

/// representa um simbolo da linguagem LA
#[derive(Debug, Clone, PartialEq)]
pub struct Simbolo {
    nome: String,
    tipo: TipoSimbolo,
}

impl Simbolo {

    /// retorna novo simbolo
    pub fn new(nome: String, tipo: TipoSimbolo) -> Self {
        Self {
            nome,
            tipo,
        }
    }

    /// retorna nome do simbolo
    pub fn nome(&self) -> String {
        self.nome.clone()
    }

    ///retorna tipo do simbolo
    pub fn tipo(&self) -> TipoSimbolo {
        self.tipo.clone()
    }
}

/// representa um escopo
#[derive(Debug, Clone)]
pub struct TabelaDeSimbolos {
    tabela: HashMap<String, Simbolo>
}

impl TabelaDeSimbolos {

    /// retorna nova tabela de simbolos
    pub fn new() -> Self {
        Self {
            tabela: HashMap::new()
        }
    }

    /// insere simbolo
    pub fn inserir(&mut self, nome: &str, tipo: &TipoSimbolo) {
        self.tabela.insert(nome.to_string(), Simbolo::new(nome.to_string(), tipo.clone()));
    }

    /// retorna simbolo, caso exista nesse escopo
    pub fn verificar(&self, nome: &str) -> Option<Simbolo> {
        self.tabela.get(nome).cloned()
    }

    /// verifica se simbolo com identificador `nome` existe nesse escopo
    pub fn existe(&self, nome: &str) -> bool {
        self.tabela.contains_key(nome)
    }
}