use std::collections::HashMap;

#[derive(Clone)]
pub enum TipoSimbolo {
    Cadeia,
    Real,
    Inteiro,
    Logico,
    Registro,
    Endereco(Box<TipoSimbolo>),
    Invalido,
}

#[derive(Clone)]
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
}

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
        self.tabela.insert(nome.to_string(), Simbolo::new(nome.to_string(), tipo.clone())).unwrap();
    }

    pub fn verificar(self, nome: &str) -> Simbolo {
        self.tabela.get(nome).unwrap().clone()
    }

    pub fn existe(&self, nome: &str) -> bool {
        self.tabela.contains_key(nome)
    }
}