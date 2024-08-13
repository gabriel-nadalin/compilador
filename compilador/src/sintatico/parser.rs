use crate::{
    lexico::{
        Lexico,
        token::{Token, TipoToken}
    },
    sintatico::arvore_sintatica::{NoAST, RegraAST}
};

const BUFFER_SIZE: usize = 10;

/// analisador sintatico\
/// transforma stream de tokens em uma arvore sintatica ou apresenta o primeiro erro sintatico encontrado
pub struct Parser {
    buffer_tokens: Vec<Token>,
    lex: Lexico,
    end: bool,
}

impl Parser {

    /// retorna instancia de parser
    pub fn new(lex: Lexico) -> Self {
        let buffer_tokens = vec![];
        let mut parser = Self {
            buffer_tokens,
            lex,
            end: false,
        };
        parser.read_token();
        parser
    }

    /// recarrega buffer, le proximo token e retorna token consumido
    fn read_token(&mut self) -> Token {

        // remove token anterior
        let anterior = if self.buffer_tokens.len() > 0 {
            self.buffer_tokens.remove(0)
        } else {
            Token::new(TipoToken::Vazio, "".to_string(), self.lex.line())
        };

        // recarrega buffer
        while self.buffer_tokens.len() < BUFFER_SIZE && !self.end {
            let next = self.lex.next_token();
            if next.tipo() == TipoToken::Fim {
                self.end = true;
            }
            self.buffer_tokens.push(next);
        }

        // println!("Lido: {}", &self.lookahead(1).tipo_string());
        anterior
    }

    /// avalia se `tipo` corresponde ao tipo do proximo token\
    /// retorna proximo token ou erro sintatico caso contrario
    fn match_(&mut self, tipo: TipoToken) -> Result<Token, NoAST> {
        if self.lookahead(1).tipo() == tipo {
            // println!("Match: {}", &self.lookahead(1).tipo_string());
            Ok(self.read_token())
        } else {
            Err(self.erro_sintatico())
        }
    }

    /// retorna uma copia do `k`esimo token do buffer, sem altera-lo
    fn lookahead(&mut self, k: usize) -> Token {
        let len = self.buffer_tokens.len();
        if len == 0 {
            return Token::new(TipoToken::Vazio, "".to_string(), self.lex.line());
        }
        if k - 1 >= len {
            return self.buffer_tokens[len - 1].copy()
        }
        self.buffer_tokens[k - 1].copy()
    }

    /// retorna mensagem de erro constando linha e lexema atuais\
    /// todo erro lexico ou sintatico eh propagado ate o no raiz recursivamente
    fn erro_sintatico(&mut self) -> NoAST {
        let linha = self.lookahead(1).linha();
        let lexema = self.lookahead(1).lexema();
        let mensagem = if self.lookahead(1).tipo() == TipoToken::Erro {
            lexema
        } else {
            format!("Linha {}: erro sintatico proximo a {}\n", linha, lexema)
        };
        
        NoAST::new(RegraAST::Erro { mensagem }, vec![]) 
    }

    // as funcoes a seguir representam as leis de formacao da gramatica da linguagem LA

    /// inicia a analise sintatica\
    /// programa : declaracoes 'algoritmo' corpo 'fim_algoritmo'
    pub fn programa(&mut self) -> NoAST {
        let mut filhos = vec![];

        let declaracoes = self.declaracoes();
        if declaracoes.is_erro() {
            return declaracoes
        }
        filhos.push(declaracoes);

        match self.match_(TipoToken::PCalgoritmo) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let corpo = self.corpo();
        if corpo.is_erro() {
            return corpo
        }
        filhos.push(corpo);

        match self.match_(TipoToken::PCfimAlgoritmo) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        match self.match_(TipoToken::Fim) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        NoAST::new(RegraAST::Programa, filhos)
    }

    /// declaracoes : declaracao declaracoes | <<vazio>>
    fn declaracoes(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::PCdeclare
            | TipoToken::PCconstante
            | TipoToken::PCtipo
            | TipoToken::PCprocedimento
            | TipoToken::PCfuncao  =>  {
                let declaracao = self.declaracao();
                if declaracao.is_erro() {
                    return declaracao
                }
                filhos.push(declaracao);

                let declaracoes = self.declaracoes();
                if declaracoes.is_erro() {
                    return declaracoes
                }
                filhos.push(declaracoes);

                NoAST::new(RegraAST::Declaracoes, filhos)
            }
            _ => NoAST::vazio()
        }
    }

    /// declaracao : declaracao_local | declaracao_global
    fn declaracao(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCdeclare
            | TipoToken::PCconstante
            | TipoToken::PCtipo => self.declaracao_local(),
            TipoToken::PCprocedimento
            | TipoToken::PCfuncao => self.declaracao_global(),
            _ => self.erro_sintatico()
        }
    }

    /// declaracao_local : 'declare' variavel
    ///     | 'tipo' IDENT ':' tipo
    ///     | 'constante' IDENT ':' tipo_basico '=' valor_constante
    fn declaracao_local(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::PCdeclare => {
                match self.match_(TipoToken::PCdeclare) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let variavel = self.variavel();
                if variavel.is_erro() {
                    return variavel
                }
                filhos.push(variavel);

                NoAST::new(RegraAST::DeclaracaoVariavel, filhos)
            }
            TipoToken::PCtipo => {
                match self.match_(TipoToken::PCtipo) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => NoAST::new_folha(RegraAST::Ident(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                filhos.push(ident);
                
                match self.match_(TipoToken::Delim) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let tipo = self.tipo();
                if tipo.is_erro() {
                    return tipo
                }
                filhos.push(tipo);

                NoAST::new(RegraAST::DeclaracaoTipo, filhos)
            }
            TipoToken::PCconstante => {
                match self.match_(TipoToken::PCconstante) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => NoAST::new_folha(RegraAST::Ident(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                filhos.push(ident);
                
                match self.match_(TipoToken::Delim) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let tipo_basico = self.tipo_basico();
                if tipo_basico.is_erro() {
                    return tipo_basico
                }
                filhos.push(tipo_basico);

                match self.match_(TipoToken::OpRelIgual) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let valor_constante = self.valor_constante();
                if valor_constante.is_erro() {
                    return valor_constante
                }
                filhos.push(valor_constante);

                NoAST::new(RegraAST::DeclaracaoConstante, filhos)
            }
            _ => self.erro_sintatico()
        }
    }

    /// variavel : identificador identificadores ':' tipo
    fn variavel(&mut self) -> NoAST {
        let mut filhos = vec![];

        let identificador = self.identificador();
        if identificador.is_erro() {
            return identificador
        }
        filhos.push(identificador);

        let identificadores = self.identificadores();
        if identificadores.is_erro() {
            return identificadores
        }
        filhos.push(identificadores);

        match self.match_(TipoToken::Delim) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let tipo = self.tipo();
        if tipo.is_erro() {
            return tipo
        }
        filhos.push(tipo);

        NoAST::new(RegraAST::Variavel, filhos)
    }

    /// identificador : IDENT identificador2 dimensao
    fn identificador(&mut self) -> NoAST {
        let mut filhos = vec![];

        let ident = match self.match_(TipoToken::Ident) {
            Ok(token) => NoAST::new_folha(RegraAST::Ident(token)),
            Err(erro_sintatico) => return erro_sintatico
        };
        filhos.push(ident);
        
        let identificador2 = self.identificador2();
        if identificador2.is_erro() {
            return identificador2
        }
        filhos.push(identificador2);

        let dimensao = self.dimensao();
        if dimensao.is_erro() {
            return dimensao
        }
        filhos.push(dimensao);

        NoAST::new(RegraAST::Identificador, filhos)
    }

    /// identificador2 : '.' IDENT identificador2 | <<vazio>>
    fn identificador2(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::Ponto => {
                match self.match_(TipoToken::Ponto) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => NoAST::new_folha(RegraAST::Ident(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                filhos.push(ident);
                
                let identificador2 = self.identificador2();
                if identificador2.is_erro() {
                    return identificador2
                }
                filhos.push(identificador2);

                NoAST::new(RegraAST::Identificador2, filhos)
            }
            _ => NoAST::vazio()
        }
    }

    /// identificadores: ',' identificador identificadores | <<vazio>>
    fn identificadores(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                match self.match_(TipoToken::Virgula) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let identificador = self.identificador();
                if identificador.is_erro() {
                    return identificador
                }
                filhos.push(identificador);

                let identificadores = self.identificadores();
                if identificadores.is_erro() {
                    return identificadores
                }
                filhos.push(identificadores);

                NoAST::new(RegraAST::Identificadores, filhos)
            }
            _ => NoAST::vazio()
        }
    }

    /// dimensao : '[' exp_aritmetica ']' dimensao | <<vazio>>
    fn dimensao(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::AbreCol => {
                match self.match_(TipoToken::AbreCol) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let exp_aritmetica = self.exp_aritmetica();
                if exp_aritmetica.is_erro() {
                    return exp_aritmetica
                }
                filhos.push(exp_aritmetica);

                match self.match_(TipoToken::FechaCol) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let dimensao = self.dimensao();
                if dimensao.is_erro() {
                    return dimensao
                }
                filhos.push(dimensao);

                NoAST::new(RegraAST::Dimensao, filhos)
            }
            _ => NoAST::vazio()
        }
    }

    /// tipo : registro | tipo_estendido
    fn tipo(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCregistro => self.registro(),
            TipoToken::Circunflexo
            | TipoToken::PCliteral
            | TipoToken::PCinteiro
            | TipoToken::PCreal
            | TipoToken::PClogico
            | TipoToken::Ident => self.tipo_estendido(),
            _ => self.erro_sintatico()
        }
    }

    /// tipo_basico : 'literal' | 'inteiro' | 'real' | 'logico'
    fn tipo_basico(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::PCliteral => match self.match_(TipoToken::PCliteral) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PCinteiro => match self.match_(TipoToken::PCinteiro) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PCreal => match self.match_(TipoToken::PCreal) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PClogico => match self.match_(TipoToken::PClogico) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };
        
        NoAST::new_folha(RegraAST::TipoBasico(token))
    }

    /// tipo_basico_ident : tipo_basico | IDENT
    fn tipo_basico_ident(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCliteral
            | TipoToken::PCinteiro
            | TipoToken::PCreal
            | TipoToken::PClogico => self.tipo_basico(),
            TipoToken::Ident => {
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => token,
                    Err(erro_sintatico) => return erro_sintatico
                };
                NoAST::new_folha(RegraAST::Ident(ident))
            }
            _ => self.erro_sintatico()
        }
    }

    /// tipo_estendido : circunflexo tipo_basico_ident
    fn tipo_estendido(&mut self) -> NoAST {
        let mut filhos = vec![];

        let circunflexo = self.circunflexo();
        if circunflexo.is_erro() {
            return circunflexo
        }
        filhos.push(circunflexo);

        let tipo_basico_ident = self.tipo_basico_ident();
        if tipo_basico_ident.is_erro() {
            return tipo_basico_ident
        }
        filhos.push(tipo_basico_ident);

        NoAST::new(RegraAST::TipoExtendido, filhos)
    }

    /// circunflexo: '^' | <<vazio>>
    fn circunflexo(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::Circunflexo => match self.match_(TipoToken::Circunflexo) {
                Ok(_) => NoAST::new_folha(RegraAST::Circunflexo),
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => NoAST::vazio()
        }
    }

    /// valor_constante : CADEIA | NUM_INT | NUM_REAL | 'verdadeiro' | 'falso'
    fn valor_constante(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::Cadeia => match self.match_(TipoToken::Cadeia) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::NumInt => match self.match_(TipoToken::NumInt) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::NumReal => match self.match_(TipoToken::NumReal) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PCverdadeiro => match self.match_(TipoToken::PCverdadeiro) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PCfalso => match self.match_(TipoToken::PCfalso) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };

        NoAST::new_folha(RegraAST::ValorConstante(token))
    }

    /// registro : 'registro' variaveis 'fim_Registro'
    fn registro(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.match_(TipoToken::PCregistro) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let variaveis = self.variaveis();
        if variaveis.is_erro() {
            return variaveis
        }
        filhos.push(variaveis);

        match self.match_(TipoToken::PCfimRegistro) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        NoAST::new(RegraAST::Registro, filhos)
    }
    
    /// variaveis : variavel variaveis | <<vazio>>
    fn variaveis(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::Ident => {
                let variavel = self.variavel();
                if variavel.is_erro() {
                    return variavel
                }
                filhos.push(variavel);

                let variaveis = self.variaveis();
                if variaveis.is_erro() {
                    return variaveis
                }
                filhos.push(variaveis);

                NoAST::new(RegraAST::Variaveis, filhos)
            },
            _ => NoAST::vazio()
        }
    }
    
    /// declaracao_global : 'procedimento' IDENT '(' parametros ')' declaracoes_locais cmds 'fim_procedimento' fecha_escopo
    ///     | 'funcao' IDENT '(' parametros ')' ':' tipo_estendido declaracoes_locais cmds 'fim_funcao' fecha_escopo
    fn declaracao_global(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::PCprocedimento => {
                match self.match_(TipoToken::PCprocedimento) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => NoAST::new_folha(RegraAST::Ident(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                filhos.push(ident);
                
                match self.match_(TipoToken::AbrePar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let parametros = self.parametros();
                if parametros.is_erro() {
                    return parametros
                }
                filhos.push(parametros);

                match self.match_(TipoToken::FechaPar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let declaracoes_locais = self.declaracoes_locais();
                if declaracoes_locais.is_erro() {
                    return declaracoes_locais
                }
                filhos.push(declaracoes_locais);

                let cmds = self.cmds();
                if cmds.is_erro() {
                    return cmds
                }
                filhos.push(cmds);

                match self.match_(TipoToken::PCfimProcedimento) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }

                filhos.push(NoAST::new_folha(RegraAST::FechaEscopo));
                
                NoAST::new(RegraAST::DeclaracaoProcedimento, filhos)
            }
            TipoToken::PCfuncao => {
                match self.match_(TipoToken::PCfuncao) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let ident = match self.match_(TipoToken::Ident) {
                    Ok(token) => NoAST::new_folha(RegraAST::Ident(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                filhos.push(ident);
                
                match self.match_(TipoToken::AbrePar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let parametros = self.parametros();
                if parametros.is_erro() {
                    return parametros
                }
                filhos.push(parametros);

                match self.match_(TipoToken::FechaPar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                match self.match_(TipoToken::Delim) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let tipo_estendido = self.tipo_estendido();
                if tipo_estendido.is_erro() {
                    return tipo_estendido
                }
                filhos.push(tipo_estendido);

                let declaracoes_locais = self.declaracoes_locais();
                if declaracoes_locais.is_erro() {
                    return declaracoes_locais
                }
                filhos.push(declaracoes_locais);

                let cmds = self.cmds();
                if cmds.is_erro() {
                    return cmds
                }
                filhos.push(cmds);

                match self.match_(TipoToken::PCfimFuncao) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                filhos.push(NoAST::new_folha(RegraAST::FechaEscopo));

                NoAST::new(RegraAST::DeclaracaoFuncao, filhos)
            }
            _ => self.erro_sintatico()
        }
    }

    /// declaracoes_locais : declaracao_local declaracoes_locais | <<vazio>>
    fn declaracoes_locais(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::PCdeclare
            | TipoToken::PCconstante
            | TipoToken::PCtipo => {
                let declaracao_local = self.declaracao_local();
                if declaracao_local.is_erro() {
                    return declaracao_local
                }
                filhos.push(declaracao_local);

                let declaracoes_locais = self.declaracoes_locais();
                if declaracoes_locais.is_erro() {
                    return declaracoes_locais
                }
                filhos.push(declaracoes_locais);

                NoAST::new(RegraAST::DeclaracoesLocais, filhos)
            }
            _ => NoAST::vazio()
        }
    }

    /// parametro : var identificador identificadores ':' tipo_estendido
    fn parametro(&mut self) -> NoAST {
        let mut filhos = vec![];

        let var = self.var();
        if var.is_erro() {
            return var
        }
        filhos.push(var);

        let identificador = self.identificador();
        if identificador.is_erro() {
            return identificador
        }
        filhos.push(identificador);

        let identificadores = self.identificadores();
        if identificadores.is_erro() {
            return identificadores
        }
        filhos.push(identificadores);

        match self.match_(TipoToken::Delim) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let tipo_estendido = self.tipo_estendido();
        if tipo_estendido.is_erro() {
            return tipo_estendido
        }
        filhos.push(tipo_estendido);

        NoAST::new(RegraAST::Parametro, filhos)
    }

    /// parametros : parametro parametros2 | <<vazio>>
    fn parametros(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::PCvar
            | TipoToken::Ident => {
                let parametro = self.parametro();
                if parametro.is_erro() {
                    return parametro
                }
                filhos.push(parametro);

                let parametros2 = self.parametros2();
                if parametros2.is_erro() {
                    return parametros2
                }
                filhos.push(parametros2);

                NoAST::new(RegraAST::Parametros, filhos)
            }
            _ => NoAST::vazio()
        }
    }

    /// parametros2 : ',' parametro parametros2 | <<vazio>>
    fn parametros2(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                match self.match_(TipoToken::Virgula) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let parametro = self.parametro();
                if parametro.is_erro() {
                    return parametro
                }
                filhos.push(parametro);

                let parametros2 = self.parametros2();
                if parametros2.is_erro() {
                    return parametros2
                }
                filhos.push(parametros2);

                NoAST::new(RegraAST::Parametros2, filhos)
            }
            _ => NoAST::vazio()
        }
    }

    /// var : 'var' | <<vazio>>
    fn var(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCvar => match self.match_(TipoToken::PCvar) {
                Ok(_) => NoAST::new_folha(RegraAST::Var),
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => NoAST::vazio()
        }
    }

    /// corpo : declaracoes_locais cmds
    fn corpo(&mut self) -> NoAST {
        let mut filhos = vec![];

        let declaracoes_locais = self.declaracoes_locais();
        if declaracoes_locais.is_erro() {
            return declaracoes_locais
        }
        filhos.push(declaracoes_locais);

        let cmds = self.cmds();
        if cmds.is_erro() {
            return cmds
        }
        filhos.push(cmds);

        NoAST::new(RegraAST::Corpo, filhos)
    }

    /// cmd : cmdLeia | cmdEscreva | cmdSe | cmdCaso | cmdPara| cmdEnquanto
    ///     | cmdFaca | cmdAtribuicao | cmdChamada | cmdRetorne
    fn cmd(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCleia => self.cmd_leia(),
            TipoToken::PCescreva => self.cmd_escreva(),
            TipoToken::PCse => self.cmd_se(),
            TipoToken::PCcaso => self.cmd_caso(),
            TipoToken::PCpara => self.cmd_para(),
            TipoToken::PCenquanto => self.cmd_enquanto(),
            TipoToken::PCfaca => self.cmd_faca(),
            TipoToken::Circunflexo => self.cmd_atribuicao(),
            TipoToken::Ident => {
                match self.lookahead(2).tipo() {
                    TipoToken::AbrePar => self.cmd_chamada(),
                    _ => self.cmd_atribuicao()
                }
            }
            TipoToken::PCretorne => self.cmd_retorne(),
            _ => self.erro_sintatico()
        }
    }

    /// cmds : cmd cmds | <<vazio>>
    fn cmds(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::PCleia | TipoToken::PCescreva | TipoToken::PCse | TipoToken::PCcaso
            | TipoToken::PCpara | TipoToken::PCenquanto | TipoToken::PCfaca | TipoToken::Circunflexo
            | TipoToken::Ident | TipoToken::PCretorne => {
                let cmd = self.cmd();
                if cmd.is_erro() {
                    return cmd
                }
                filhos.push(cmd);

                let cmds = self.cmds();
                if cmds.is_erro() {
                    return cmds
                }
                filhos.push(cmds);

                NoAST::new(RegraAST::CMDs, filhos)
            }
            _ => NoAST::vazio()
        }
    }

    /// cmdLeia : 'leia' '(' circunflexo identificador cmdLeia2 ')'
    fn cmd_leia(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.match_(TipoToken::PCleia) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        match self.match_(TipoToken::AbrePar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let circunflexo = self.circunflexo();
        if circunflexo.is_erro() {
            return circunflexo
        }
        filhos.push(circunflexo);

        let identificador = self.identificador();
        if identificador.is_erro() {
            return identificador
        }
        filhos.push(identificador);

        let cmd_leia2 = self.cmd_leia2();
        if cmd_leia2.is_erro() {
            return cmd_leia2
        }
        filhos.push(cmd_leia2);

        match self.match_(TipoToken::FechaPar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        NoAST::new(RegraAST::CMDLeia, filhos)
    }

    /// cmdLeia2 : ',' circunflexo identificador cmdLeia2 | <<vazio>>
    fn cmd_leia2(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                match self.match_(TipoToken::Virgula) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let circunflexo = self.circunflexo();
                if circunflexo.is_erro() {
                    return circunflexo
                }
                filhos.push(circunflexo);

                let identificador = self.identificador();
                if identificador.is_erro() {
                    return identificador
                }
                filhos.push(identificador);

                let cmd_leia2 = self.cmd_leia2();
                if cmd_leia2.is_erro() {
                    return cmd_leia2
                }
                filhos.push(cmd_leia2);

                NoAST::new(RegraAST::CMDLeia2, filhos)
            }
            _ => NoAST::vazio()
        }
    }

    /// cmdEscreva : 'escreva' '(' expressao expressoes ')'
    fn cmd_escreva(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.match_(TipoToken::PCescreva) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        match self.match_(TipoToken::AbrePar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let expressao = self.expressao();
        if expressao.is_erro() {
            return expressao
        }
        filhos.push(expressao);

        let expressoes = self.expressoes();
        if expressoes.is_erro() {
            return expressoes
        }
        filhos.push(expressoes);

        match self.match_(TipoToken::FechaPar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        NoAST::new(RegraAST::CMDEscreva, filhos)
    }

    /// cmdSe : 'se' expressao 'entao' cmds senao 'fim_se'
    fn cmd_se(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.match_(TipoToken::PCse) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let expressao = self.expressao();
        if expressao.is_erro() {
            return expressao
        }
        filhos.push(expressao);

        match self.match_(TipoToken::PCentao) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let cmds = self.cmds();
        if cmds.is_erro() {
            return cmds
        }
        filhos.push(cmds);

        let senao = self.senao();
        if senao.is_erro() {
            return senao
        }
        filhos.push(senao);

        match self.match_(TipoToken::PCfimSe) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        NoAST::new(RegraAST::CMDSe, filhos)
    }

    /// senao : 'senao' cmds | <<vazio>>
    fn senao(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::PCsenao => {
                match self.match_(TipoToken::PCsenao) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let cmds = self.cmds();
                if cmds.is_erro() {
                    return cmds
                }
                filhos.push(cmds);

                NoAST::new(RegraAST::Senao, filhos)
            }
            _ => NoAST::vazio()
        }
    }

    /// cmdCaso : 'caso' exp_aritmetica 'seja' selecao senao 'fim_caso'
    fn cmd_caso(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.match_(TipoToken::PCcaso) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let exp_aritmetica = self.exp_aritmetica();
        if exp_aritmetica.is_erro() {
            return exp_aritmetica
        }
        filhos.push(exp_aritmetica);

        match self.match_(TipoToken::PCseja) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let selecao = self.selecao();
        if selecao.is_erro() {
            return selecao
        }
        filhos.push(selecao);

        let senao = self.senao();
        if senao.is_erro() {
            return senao
        }
        filhos.push(senao);

        match self.match_(TipoToken::PCfimCaso) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        NoAST::new(RegraAST::CMDCaso, filhos)
    }

    /// cmdPara : 'para' IDENT '<-' exp_aritmetica 'ate' exp_aritmetica 'faca' cmds 'fim_para'
    fn cmd_para(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.match_(TipoToken::PCpara) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let ident = match self.match_(TipoToken::Ident) {
            Ok(token) => NoAST::new_folha(RegraAST::Ident(token)),
            Err(erro_sintatico) => return erro_sintatico
        };
        filhos.push(ident);
        
        match self.match_(TipoToken::BackArrow) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let exp_aritmetica_1 = self.exp_aritmetica();
        if exp_aritmetica_1.is_erro() {
            return exp_aritmetica_1
        }
        filhos.push(exp_aritmetica_1);

        match self.match_(TipoToken::PCate) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let exp_aritmetica_2 = self.exp_aritmetica();
        if exp_aritmetica_2.is_erro() {
            return exp_aritmetica_2
        }
        filhos.push(exp_aritmetica_2);

        match self.match_(TipoToken::PCfaca) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let cmds = self.cmds();
        if cmds.is_erro() {
            return cmds
        }
        filhos.push(cmds);

        match self.match_(TipoToken::PCfimPara) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        NoAST::new(RegraAST::CMDPara, filhos)
    }

    /// cmdEnquanto : 'enquanto' expressao 'faca' cmds 'fim_enquanto'
    fn cmd_enquanto(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.match_(TipoToken::PCenquanto) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let expressao = self.expressao();
        if expressao.is_erro() {
            return expressao
        }
        filhos.push(expressao);

        match self.match_(TipoToken::PCfaca) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let cmds = self.cmds();
        if cmds.is_erro() {
            return cmds
        }
        filhos.push(cmds);

        match self.match_(TipoToken::PCfimEnquanto) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        NoAST::new(RegraAST::CMDEnquanto, filhos)
    }

    /// cmdFaca : 'faca' cmds 'ate' expressao
    fn cmd_faca(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.match_(TipoToken::PCfaca) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let cmds = self.cmds();
        if cmds.is_erro() {
            return cmds
        }
        filhos.push(cmds);

        match self.match_(TipoToken::PCate) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let expressao = self.expressao();
        if expressao.is_erro() {
            return expressao
        }
        filhos.push(expressao);

        NoAST::new(RegraAST::CMDFaca, filhos)
    }

    /// cmdAtribuicao : circunflexo identificador '<-' expressao
    fn cmd_atribuicao(&mut self) -> NoAST {
        let mut filhos = vec![];

        let circunflexo = self.circunflexo();
        if circunflexo.is_erro() {
            return circunflexo
        }
        filhos.push(circunflexo);

        let identificador = self.identificador();
        if identificador.is_erro() {
            return identificador
        }
        filhos.push(identificador);

        match self.match_(TipoToken::BackArrow) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let expressao = self.expressao();
        if expressao.is_erro() {
            return expressao
        }
        filhos.push(expressao);

        NoAST::new(RegraAST::CMDAtribuicao, filhos)
    }

    /// cmdChamada : IDENT '(' expressao expressoes ')'
    fn cmd_chamada(&mut self) -> NoAST {
        let mut filhos = vec![];

        let ident = match self.match_(TipoToken::Ident) {
            Ok(token) => NoAST::new_folha(RegraAST::Ident(token)),
            Err(erro_sintatico) => return erro_sintatico
        };
        filhos.push(ident);
        
        match self.match_(TipoToken::AbrePar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let expressao = self.expressao();
        if expressao.is_erro() {
            return expressao
        }
        filhos.push(expressao);

        let expressoes = self.expressoes();
        if expressoes.is_erro() {
            return expressoes
        }
        filhos.push(expressoes);

        match self.match_(TipoToken::FechaPar) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        NoAST::new(RegraAST::CMDChamada, filhos)
    }

    /// cmdRetorne : 'retorne' expressao
    fn cmd_retorne(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.match_(TipoToken::PCretorne) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let expressao = self.expressao();
        if expressao.is_erro() {
            return expressao
        }
        filhos.push(expressao);

        NoAST::new(RegraAST::CMDRetorne, filhos)
    }

    /// selecao : item_selecao selecao | <<vazio>>
    fn selecao(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::OpAritSub | TipoToken::NumInt => {
                let item_selecao = self.item_selecao();
                if item_selecao.is_erro() {
                    return item_selecao
                }
                filhos.push(item_selecao);

                let selecao = self.selecao();
                if selecao.is_erro() {
                    return selecao
                }
                filhos.push(selecao);

                NoAST::new(RegraAST::Selecao, filhos)
            }
            _ => NoAST::vazio()
        }
    }

    /// item_selecao : constantes ':' cmds
    fn item_selecao(&mut self) -> NoAST {
        let mut filhos = vec![];

        let constantes = self.constantes();
        if constantes.is_erro() {
            return constantes
        }
        filhos.push(constantes);

        match self.match_(TipoToken::Delim) {
            Ok(_) => {},
            Err(erro_sintatico) => return erro_sintatico
        }
        
        let cmds = self.cmds();
        if cmds.is_erro() {
            return cmds
        }
        filhos.push(cmds);

        NoAST::new(RegraAST::ItemSelecao, filhos)
    }
    
    /// constantes : numero_intervalo numero_intervalos
    fn constantes(&mut self) -> NoAST {
        let mut filhos = vec![];

        let numero_intervalo = self.numero_intervalo();
        if numero_intervalo.is_erro() {
            return numero_intervalo
        }
        filhos.push(numero_intervalo);

        let numero_intervalos = self.numero_intervalos();
        if numero_intervalos.is_erro() {
            return numero_intervalos
        }
        filhos.push(numero_intervalos);

        NoAST::new(RegraAST::Constantes, filhos)
    }
    
    /// numero_intervalo : op_unario NUM_INT numero_intervalo2
    fn numero_intervalo(&mut self) -> NoAST {
        let mut filhos = vec![];

        let op_unario = self.op_unario();
        if op_unario.is_erro() {
            return op_unario
        }
        filhos.push(op_unario);

        let num_int = match self.match_(TipoToken::NumInt) {
            Ok(token) => NoAST::new_folha(RegraAST::NumInt(token)),
            Err(erro_sintatico) => return erro_sintatico
        };
        filhos.push(num_int);
        
        let numero_intervalo2 = self.numero_intervalo2();
        if numero_intervalo2.is_erro() {
            return numero_intervalo2
        }
        filhos.push(numero_intervalo2);

        NoAST::new(RegraAST::NumeroIntervalo, filhos)
    }
    
    /// numero_intervalos : ',' numero_intervalo numero_intervalos | <<vazio>>
    fn numero_intervalos(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                match self.match_(TipoToken::Virgula) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let numero_intervalo = self.numero_intervalo();
                if numero_intervalo.is_erro() {
                    return numero_intervalo
                }
                filhos.push(numero_intervalo);

                let numero_intervalos = self.numero_intervalos();
                if numero_intervalos.is_erro() {
                    return numero_intervalos
                }
                filhos.push(numero_intervalos);

                NoAST::new(RegraAST::NumeroIntervalos, filhos)
            }
            _ => NoAST::vazio()
        }
    }
    
    /// numero_intervalo2 : '..' op_unario NUM_INT | <<vazio>>
    fn numero_intervalo2(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::PontoPonto => {
                match self.match_(TipoToken::PontoPonto) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let op_unario = self.op_unario();
                if op_unario.is_erro() {
                    return op_unario
                }
                filhos.push(op_unario);

                let num_int = match self.match_(TipoToken::NumInt) {
                    Ok(token) => NoAST::new_folha(RegraAST::NumInt(token)),
                    Err(erro_sintatico) => return erro_sintatico
                };
                filhos.push(num_int);
                
                NoAST::new(RegraAST::NumeroIntervalo2, filhos)
            }
            _ => NoAST::vazio()
        }
    }
    
    /// op_unario : '-' | <<vazio>>
    fn op_unario(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::OpAritSub => {
                match self.match_(TipoToken::OpAritSub) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                NoAST::new_folha(RegraAST::OpUnario)
            }
            _ => NoAST::vazio()
        }
    }
    
    /// exp_aritmetica : termo termos
    fn exp_aritmetica(&mut self) -> NoAST {
        let mut filhos = vec![];

        let termo = self.termo();
        if termo.is_erro() {
            return termo
        }
        filhos.push(termo);

        let termos = self.termos();
        if termos.is_erro() {
            return termos
        }
        filhos.push(termos);

        NoAST::new(RegraAST::ExpAritmetica, filhos)
    }
    
    /// termo : fator fatores
    fn termo(&mut self) -> NoAST {
        let mut filhos = vec![];

        let fator = self.fator();
        if fator.is_erro() {
            return fator
        }
        filhos.push(fator);

        let fatores = self.fatores();
        if fatores.is_erro() {
            return fatores
        }
        filhos.push(fatores);

        NoAST::new(RegraAST::Termo, filhos)
    }
    
    /// termos : op1 termo termos | <<vazio>>
    fn termos(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::OpAritSoma | TipoToken::OpAritSub => {
                let op1 = self.op1();
                if op1.is_erro() {
                    return op1
                }
                filhos.push(op1);

                let termo = self.termo();
                if termo.is_erro() {
                    return termo
                }
                filhos.push(termo);

                let termos = self.termos();
                if termos.is_erro() {
                    return termos
                }
                filhos.push(termos);

                NoAST::new(RegraAST::Termos, filhos)
            }
            _ => NoAST::vazio()
        }
    }
    
    /// fator : parcela parcelas
    fn fator(&mut self) -> NoAST {
        let mut filhos = vec![];

        let parcela = self.parcela();
        if parcela.is_erro() {
            return parcela
        }
        filhos.push(parcela);

        let parcelas = self.parcelas();
        if parcelas.is_erro() {
            return parcelas
        }
        filhos.push(parcelas);

        NoAST::new(RegraAST::Fator, filhos)
    }
    
    /// fatores : op2 fator fatores | <<vazio>>
    fn fatores(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::OpAritMult | TipoToken::OpAritDiv => {
                let op2 = self.op2();
                if op2.is_erro() {
                    return op2
                }
                filhos.push(op2);

                let fator = self.fator();
                if fator.is_erro() {
                    return fator
                }
                filhos.push(fator);

                let fatores = self.fatores();
                if fatores.is_erro() {
                    return fatores
                }
                filhos.push(fatores);

                NoAST::new(RegraAST::Fatores, filhos)
            }
            _ => NoAST::vazio()
        }
    }
    
    /// op1 : '+' | '-'
    fn op1(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::OpAritSoma => match self.match_(TipoToken::OpAritSoma) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpAritSub => match self.match_(TipoToken::OpAritSub) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };
        NoAST::new_folha(RegraAST::Op1(token))
    }
    
    /// op2 : '*' | '/'
    fn op2(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::OpAritMult => match self.match_(TipoToken::OpAritMult) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpAritDiv => match self.match_(TipoToken::OpAritDiv) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };
        NoAST::new_folha(RegraAST::Op2(token))
    }
    
    /// op3 : '%'
    fn op3(&mut self) -> NoAST {
        match self.match_(TipoToken::Porcento) {
            Ok(_) => NoAST::new_folha(RegraAST::Op3),
            Err(erro_sintatico) => erro_sintatico
        }
    }
    
    /// parcela : op_unario parcela_unario | parcela_nao_unario
    fn parcela(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::OpAritSub
            | TipoToken::Circunflexo
            | TipoToken::Ident
            | TipoToken::NumInt
            | TipoToken::NumReal
            | TipoToken::AbrePar => {
                let op_unario = self.op_unario();
                if op_unario.is_erro() {
                    return op_unario
                }
                filhos.push(op_unario);

                let parcela_unario = self.parcela_unario();
                if parcela_unario.is_erro() {
                    return parcela_unario
                }
                filhos.push(parcela_unario);

                NoAST::new(RegraAST::Parcela, filhos)
            }
            TipoToken::EComercial
            | TipoToken::Cadeia => self.parcela_nao_unario(),
            _ => self.erro_sintatico()
        }
    }
    
    /// parcelas : op3 parcela parcelas | <<vazio>>
    fn parcelas(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::Porcento => {
                let op3 = self.op3();
                if op3.is_erro() {
                    return op3
                }
                filhos.push(op3);

                let parcela = self.parcela();
                if parcela.is_erro() {
                    return parcela
                }
                filhos.push(parcela);

                let parcelas = self.parcelas();
                if parcelas.is_erro() {
                    return parcelas
                }
                filhos.push(parcelas);

                NoAST::new(RegraAST::Parcelas, filhos)
            }
            _ => NoAST::vazio()
        }
    }
    
    /// parcela_unario : circunflexo identificador
    ///     | IDENT '(' expressao expressoes ')'
    ///     | '(' expressao ')'
    ///     | NUM_INT
    ///     | NUM_REAL
    fn parcela_unario(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::Circunflexo => {
                let circunflexo = self.circunflexo();
                if circunflexo.is_erro() {
                    return circunflexo
                }
                filhos.push(circunflexo);

                let identificador = self.identificador();
                if identificador.is_erro() {
                    return identificador
                }
                filhos.push(identificador);

                NoAST::new(RegraAST::ParcelaUnario1, filhos)
            }
            TipoToken::Ident => {
                match self.lookahead(2).tipo() {
                    TipoToken::AbrePar => {
                        let ident = match self.match_(TipoToken::Ident) {
                            Ok(token) => NoAST::new_folha(RegraAST::Ident(token)),
                            Err(erro_sintatico) => return erro_sintatico
                        };
                        filhos.push(ident);
                        
                        match self.match_(TipoToken::AbrePar) {
                            Ok(_) => {},
                            Err(erro_sintatico) => return erro_sintatico
                        }
                        
                        let expressao = self.expressao();
                        if expressao.is_erro() {
                            return expressao
                        }
                        filhos.push(expressao);

                        let expressoes = self.expressoes();
                        if expressoes.is_erro() {
                            return expressoes
                        }
                        filhos.push(expressoes);

                        match self.match_(TipoToken::FechaPar) {
                            Ok(_) => {},
                            Err(erro_sintatico) => return erro_sintatico
                        }
                        
                        NoAST::new(RegraAST::ParcelaUnario2, filhos)
                    }
                    _ => self.identificador()
                }
            }
            TipoToken::AbrePar => {
                match self.match_(TipoToken::AbrePar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let expressao = self.expressao();
                if expressao.is_erro() {
                    return expressao
                }
                filhos.push(expressao);

                match self.match_(TipoToken::FechaPar) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                NoAST::new(RegraAST::ParcelaUnario3, filhos)
            }
            TipoToken::NumInt => match self.match_(TipoToken::NumInt) {
                Ok(token) => NoAST::new_folha(RegraAST::NumInt(token)),
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::NumReal => match self.match_(TipoToken::NumReal) {
                Ok(token) => NoAST::new_folha(RegraAST::NumReal(token)),
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => self.erro_sintatico()
        }
    }
    
    /// parcela_nao_unario : '&' identificador | CADEIA
    fn parcela_nao_unario(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::EComercial => {
                match self.match_(TipoToken::EComercial) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let identificador = self.identificador();
                if identificador.is_erro() {
                    return identificador
                }
                filhos.push(identificador);

                NoAST::new(RegraAST::ParcelaNaoUnario, filhos)
            }
            TipoToken::Cadeia => match self.match_(TipoToken::Cadeia) {
                Ok(token) => NoAST::new_folha(RegraAST::Cadeia(token)),
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => self.erro_sintatico()
        }
    }
    
    /// exp_relacional : exp_aritmetica exp_relacional2
    fn exp_relacional(&mut self) -> NoAST {
        let mut filhos = vec![];

        let exp_aritmetica = self.exp_aritmetica();
        if exp_aritmetica.is_erro() {
            return exp_aritmetica
        }
        filhos.push(exp_aritmetica);

        let exp_relacional2 = self.exp_relacional2();
        if exp_relacional2.is_erro() {
            return exp_relacional2
        }
        filhos.push(exp_relacional2);

        NoAST::new(RegraAST::ExpRelacional, filhos)
    }
    
    /// exp_relacional2 : op_relacional exp_aritmetica | <<vazio>>
    fn exp_relacional2(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::OpRelIgual
            | TipoToken::OpRelDif
            | TipoToken::OpRelMaiorIgual
            | TipoToken::OpRelMenorIgual
            | TipoToken::OpRelMaior
            | TipoToken::OpRelMenor => {
                let op_relacional = self.op_relacional();
                if op_relacional.is_erro() {
                    return op_relacional
                }
                filhos.push(op_relacional);

                let exp_aritmetica = self.exp_aritmetica();
                if exp_aritmetica.is_erro() {
                    return exp_aritmetica
                }
                filhos.push(exp_aritmetica);

                NoAST::new(RegraAST::ExpRelacional2, filhos)
            }
            _ => NoAST::vazio()
        }
    }
    
    /// op_relacional : '=' | '<>' | '>=' | '<=' | '>' | '<'
    fn op_relacional(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::OpRelIgual => match self.match_(TipoToken::OpRelIgual) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpRelDif => match self.match_(TipoToken::OpRelDif) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpRelMaiorIgual => match self.match_(TipoToken::OpRelMaiorIgual) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpRelMenorIgual => match self.match_(TipoToken::OpRelMenorIgual) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpRelMaior => match self.match_(TipoToken::OpRelMaior) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::OpRelMenor => match self.match_(TipoToken::OpRelMenor) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };
        NoAST::new_folha(RegraAST::OpRelacional(token))
    }
    
    /// expressao : termo_logico termos_logicos
    fn expressao(&mut self) -> NoAST {
        let mut filhos = vec![];

        let termo_logico = self.termo_logico();
        if termo_logico.is_erro() {
            return termo_logico
        }
        filhos.push(termo_logico);

        let termos_logicos = self.termos_logicos();
        if termos_logicos.is_erro() {
            return termos_logicos
        }
        filhos.push(termos_logicos);

        NoAST::new(RegraAST::Expressao, filhos)
    }
    
    /// expressoes : ',' expressao expressoes | <<vazio>>
    fn expressoes(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::Virgula => {
                match self.match_(TipoToken::Virgula) {
                    Ok(_) => {},
                    Err(erro_sintatico) => return erro_sintatico
                }
                
                let expressao = self.expressao();
                if expressao.is_erro() {
                    return expressao
                }
                filhos.push(expressao);

                let expressoes = self.expressoes();
                if expressoes.is_erro() {
                    return expressoes
                }
                filhos.push(expressoes);

                NoAST::new(RegraAST::Expressoes, filhos)
            }
            _ => NoAST::vazio()
        }
    }
    
    /// termo_logico : fator_logico fatores_logicos
    fn termo_logico(&mut self) -> NoAST {
        let mut filhos = vec![];

        let fator_logico = self.fator_logico();
        if fator_logico.is_erro() {
            return fator_logico
        }
        filhos.push(fator_logico);

        let fatores_logicos = self.fatores_logicos();
        if fatores_logicos.is_erro() {
            return fatores_logicos
        }
        filhos.push(fatores_logicos);

        NoAST::new(RegraAST::TermoLogico, filhos)
    }
    
    /// termos_logicos : op_logico_1 termo_logico termos_logicos | <<vazio>>
    fn termos_logicos(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::PCou => {
                let op_logico_1 = self.op_logico_1();
                if op_logico_1.is_erro() {
                    return op_logico_1
                }
                filhos.push(op_logico_1);

                let termo_logico = self.termo_logico();
                if termo_logico.is_erro() {
                    return termo_logico
                }
                filhos.push(termo_logico);

                let termos_logicos = self.termos_logicos();
                if termos_logicos.is_erro() {
                    return termos_logicos
                }
                filhos.push(termos_logicos);

                NoAST::new(RegraAST::TermosLogicos, filhos)
            }
            _ => NoAST::vazio()
        }
    }
    
    /// fator_logico : nao parcela_logica
    fn fator_logico(&mut self) -> NoAST {
        let mut filhos = vec![];

        let nao = self.nao();
        if nao.is_erro() {
            return nao
        }
        filhos.push(nao);

        let parcela_logica = self.parcela_logica();
        if parcela_logica.is_erro() {
            return parcela_logica
        }
        filhos.push(parcela_logica);

        NoAST::new(RegraAST::FatorLogico, filhos)
    }
    
    /// fatores_logicos : op_logico_2 fator_logico fatores_logicos | <<vazio>>
    fn fatores_logicos(&mut self) -> NoAST {
        let mut filhos = vec![];

        match self.lookahead(1).tipo() {
            TipoToken::PCe => {
                let op_logico_2 = self.op_logico_2();
                if op_logico_2.is_erro() {
                    return op_logico_2
                }
                filhos.push(op_logico_2);

                let fator_logico = self.fator_logico();
                if fator_logico.is_erro() {
                    return fator_logico
                }
                filhos.push(fator_logico);

                let fatores_logicos = self.fatores_logicos();
                if fatores_logicos.is_erro() {
                    return fatores_logicos
                }
                filhos.push(fatores_logicos);

                NoAST::new(RegraAST::FatoresLogicos, filhos)
            }
            _ => NoAST::vazio()
        }
    }
    
    /// nao : 'nao' | <<vazio>>
    fn nao(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCnao => match self.match_(TipoToken::PCnao) {
                Ok(_) => NoAST::new_folha(RegraAST::Nao),
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => NoAST::vazio()
        }
    }
    
    /// parcela_logica : constante_logica | exp_relacional
    fn parcela_logica(&mut self) -> NoAST {
        match self.lookahead(1).tipo() {
            TipoToken::PCverdadeiro | TipoToken::PCfalso => self.constante_logica(),
            TipoToken::OpAritSub
            | TipoToken::Circunflexo
            | TipoToken::Ident
            | TipoToken::NumInt
            | TipoToken::NumReal
            | TipoToken::AbrePar
            | TipoToken::EComercial
            | TipoToken::Cadeia  => self.exp_relacional(),
            _ => self.erro_sintatico()
        }
    }
    
    /// constante_logica : 'verdadeiro' | 'falso'
    fn constante_logica(&mut self) -> NoAST {
        let token = match self.lookahead(1).tipo() {
            TipoToken::PCverdadeiro => match self.match_(TipoToken::PCverdadeiro) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            TipoToken::PCfalso => match self.match_(TipoToken::PCfalso) {
                Ok(token) => token,
                Err(erro_sintatico) => return erro_sintatico
            }
            _ => return self.erro_sintatico()
        };
        NoAST::new_folha(RegraAST::ConstanteLogica(token))
    }
    
    /// op_logico_1 : 'ou'
    fn op_logico_1(&mut self) -> NoAST {
        match self.match_(TipoToken::PCou) {
            Ok(_) => NoAST::new_folha(RegraAST::OpLogico1),
            Err(erro_sintatico) => return erro_sintatico
        }
    }
    
    /// op_logico_2 : 'e'
    fn op_logico_2(&mut self) -> NoAST {
        match self.match_(TipoToken::PCe) {
            Ok(_) => NoAST::new_folha(RegraAST::OpLogico2),
            Err(erro_sintatico) => return erro_sintatico
        }
    }
}