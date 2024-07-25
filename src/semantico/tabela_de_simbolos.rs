use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TipoSimbolo {
    Cadeia,
    Real,
    Inteiro,
    Logico,
    Registro,
    Endereco(Box<TipoSimbolo>),
    Vazio,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Simbolo {
    nome: String,
    tipo: TipoSimbolo,
}

impl Simbolo {
    pub fn new(nome: String, tipo: TipoSimbolo) -> Self {
        Self {
            nome,
            tipo,
        }
    }

    pub fn nome(&self) -> String {
        self.nome.clone()
    }

    pub fn tipo(&self) -> TipoSimbolo {
        self.tipo.clone()
    }
}

#[derive(Debug, Clone)]
pub struct TabelaDeSimbolos {
    tabela: HashMap<String, Simbolo>
}

impl TabelaDeSimbolos {
    pub fn new() -> Self {
        Self {
            tabela: HashMap::new()
        }
    }

    pub fn inserir(&mut self, nome: &str, tipo: &TipoSimbolo) {
        self.tabela.insert(nome.to_string(), Simbolo::new(nome.to_string(), tipo.clone()));
    }

    pub fn verificar(&self, nome: &str) -> Option<Simbolo> {
        self.tabela.get(nome).cloned()
    }

    pub fn existe(&self, nome: &str) -> bool {
        self.tabela.contains_key(nome)
    }
}