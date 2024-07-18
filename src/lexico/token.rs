#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TipoToken {
    PCalgoritmo, PCdeclare, PCinteiro, PCleia, PCescreva, PCfimAlgoritmo, PCreal,
    PCliteral, PClogico, PCfimSe, PCsenao, PCentao, PCse, PCfimCaso, PCpara, PCate, PCfaca,
    PCfimPara, PCfimEnquanto, PCseja, PCcaso, PCenquanto, PCregistro, PCfimRegistro,
    PCtipo, PCfimProcedimento, PCprocedimento, PCvar, PCfuncao, PCfimFuncao, PCretorne, PCconstante,
    PCfalso, PCverdadeiro, PCnao, PCou, PCe, OpAritMult, OpAritDiv, OpAritSoma,
    OpAritSub, OpRelMenor, OpRelMenorIgual, OpRelMaiorIgual, BackArrow, OpRelMaior, OpRelIgual,
    OpRelDif, Delim, AbrePar, FechaPar, AbreCol, FechaCol, Virgula, Porcento, Circunflexo,
    EComercial, Ponto, PontoPonto, Ident, NumInt, NumReal, Cadeia, Fim, Erro, Vazio,
}

#[derive(Debug, Clone)]
pub struct Token {
    tipo: TipoToken,
    lexema: String,
    linha: u32,
}

impl Token {
    
    /// retorna instancia de token
    pub fn new(tipo: TipoToken, lexema: String, linha: u32) -> Self {
        Token {
            tipo,
            lexema,
            linha,
        }
    }

    /// get tipo
    pub fn tipo(&self) -> TipoToken {
        self.tipo
    }

    /// get lexema
    pub fn lexema(&self) -> String {
        self.lexema.to_string()
    }

    /// get linha
    pub fn linha(&self) -> u32 {
        self.linha
    }

    /// retorna string representando o tipo do token
    pub fn tipo_string(&self) -> String {
        match self.tipo {
            TipoToken::Ident => "IDENT".to_string(),
            TipoToken::NumInt => "NUM_INT".to_string(),
            TipoToken::NumReal => "NUM_REAL".to_string(),
            TipoToken::Cadeia => "CADEIA".to_string(),
            _ => "'".to_string() + &self.lexema + "'"
        }
    }

    /// retorna string "<lexema,tipo_lexema>"
    pub fn to_string(&self) -> String {
        let lexema = &self.lexema;
        let tipo = self.tipo_string();
        "<'".to_owned() + lexema + "'," + &tipo + ">"
    }

    /// retorna uma copia do token
    pub fn copy(&mut self) -> Token {
        Self::new(self.tipo(), self.lexema(), self.linha)
    }
}