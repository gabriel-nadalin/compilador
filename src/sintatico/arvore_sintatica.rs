use crate::lexico::token::Token;

pub enum NoAST {
    // programa : declaracoes 'algoritmo' corpo 'fim_algoritmo'
    Programa {
        declaracoes: Box<NoAST>,
        corpo: Box<NoAST>,
    },
    
    // declaracoes : declaracao declaracoes | <<vazio>>
    Declaracoes {
        declaracao: Box<NoAST>,
        declaracoes: Box<NoAST>,
    },
    // declaracao : declaracao_local | declaracao_global
    
    // declaracao_local :
    //     'declare' variavel
    DeclaracaoVariavel { variavel: Box<NoAST>},
    
    //     | 'tipo' IDENT ':' tipo
    DeclaracaoTipo {
        ident: Box<NoAST>,
        tipo: Box<NoAST>,
    },
    
    //     | 'constante' IDENT ':' tipo_basico '=' valor_constante
    DeclaracaoConstante {
        ident: Box<NoAST>,
        tipo_basico: Box<NoAST>,
        valor_constante: Box<NoAST>,
    },
    
    // valor_constante : CADEIA | NUM_INT | NUM_REAL | 'verdadeiro' | 'falso'
    ValorConstante (Token),
    
    // variavel : identificador identificadores ':' tipo
    Variavel {
        identificador: Box<NoAST>,
        identificadores: Box<NoAST>,
        tipo: Box<NoAST>,
    },
    // tipo : registro | tipo_estendido
    
    // identificador : IDENT identificador2 dimensao
    Identificador {
        ident: Box<NoAST>,
        identificador2: Box<NoAST>,
        dimensao: Box<NoAST>,
    },

    // identificador2 : '.' IDENT identificador2 | <<vazio>>
    Identificador2 {
        ident: Box<NoAST>,
        identificador2: Box<NoAST>,
    },

    // identificadores: ',' identificador identificadores | <<vazio>>
    Identificadores {
        identificador: Box<NoAST>,
        identificadores: Box<NoAST>,
    },

    // dimensao : '[' exp_aritmetica ']' dimensao | <<vazio>>
    Dimensao {
        exp_aritmetica: Box<NoAST>,
        dimensao: Box<NoAST>,
    },

    // tipo_estendido : circunflexo tipo_basico_ident
    TipoExtendido {
        circunflexo: Box<NoAST>,
        tipo_basico_ident: Box<NoAST>,
    },
    // tipo_basico_ident : tipo_basico
    //    | IDENT
    Ident (Token),

    // tipo_basico : 'literal' | 'inteiro' | 'real' | 'logico'
    TipoBasico (Token),
    
    // circunflexo: '^' | <<vazio>>
    Circunflexo,

    // registro : 'registro' variaveis 'fim_Registro'
    Registro {
        variaveis: Box<NoAST>,
    },

    // variaveis : variavel variaveis | <<vazio>>
    Variaveis {
        variavel: Box<NoAST>,
        variaveis: Box<NoAST>,
    },

    // declaracao_global :
    //     'procedimento' IDENT '(' parametros ')' declaracoes_locais cmds 'fim_procedimento'
    DeclaracaoProcedimento {
        ident: Box<NoAST>,
        parametros: Box<NoAST>,
        declaracoes_locais: Box<NoAST>,
        cmds: Box<NoAST>,
    },

    //     | 'funcao' IDENT '(' parametros ')' ':' tipo_estendido declaracoes_locais cmds 'fim_funcao'
    DeclaracaoFuncao {
        ident: Box<NoAST>,
        parametros: Box<NoAST>,
        tipo_estendido: Box<NoAST>,
        declaracoes_locais: Box<NoAST>,
        cmds: Box<NoAST>,
    },

    // declaracoes_locais : declaracao_local declaracoes_locais | <<vazio>>
    DeclaracoesLocais {
        declaracao_local: Box<NoAST>,
        declaracoes_locais: Box<NoAST>,
    },

    // parametro : var identificador identificadores ':' tipo_estendido
    Parametro {
        var: Box<NoAST>,
        identificador: Box<NoAST>,
        identificadores: Box<NoAST>,
        tipo_estendido: Box<NoAST>,
    },

    // parametros : parametro parametros2 | <<vazio>>
    Parametros {
        parametro: Box<NoAST>,
        parametros2: Box<NoAST>,
    },
    
    // parametros2 : ',' parametro parametros2 | <<vazio>>
    Parametros2 {
        parametro: Box<NoAST>,
        parametros2: Box<NoAST>,
    },

    // var : 'var' | <<vazio>>
    Var,

    // corpo : declaracoes_locais cmds
    Corpo {
        declaracoes_locais: Box<NoAST>,
        cmds: Box<NoAST>,
    },
    
    // cmds : cmd cmds | <<vazio>>
    CMDs {
        cmd: Box<NoAST>,
        cmds: Box<NoAST>,
    },
    // cmd : cmdLeia | cmdEscreva | cmdSe | cmdCaso | cmdPara | cmdEnquanto
    //     | cmdFaca | cmdAtribuicao | cmdChamada | cmdRetorne

    // cmdLeia : 'leia' '(' circunflexo identificador cmdLeia2 ')'
    CMDLeia {
        circunflexo: Box<NoAST>,
        identificador: Box<NoAST>,
        cmd_leia2: Box<NoAST>,
    },

    // cmdLeia2 : ',' circunflexo identificador cmdLeia2 | <<vazio>>
    CMDLeia2 {
        circunflexo: Box<NoAST>,
        identificador: Box<NoAST>,
        cmd_leia2: Box<NoAST>,
    },

    // cmdEscreva : 'escreva' '(' expressao expressoes ')'
    CMDEscreva {
        expressao: Box<NoAST>,
        expressoes: Box<NoAST>,
    },

    // cmdSe : 'se' expressao 'entao' cmds senao 'fim_se'
    CMDSe {
        expressao: Box<NoAST>,
        cmds: Box<NoAST>,
        senao: Box<NoAST>,
    },

    // senao : 'senao' cmds | <<vazio>>
    Senao { cmds: Box<NoAST> },

    // cmdCaso : 'caso' exp_aritmetica 'seja' selecao senao 'fim_caso'
    CMDCaso {
        exp_aritmetica: Box<NoAST>,
        selecao: Box<NoAST>,
        senao: Box<NoAST>,
    },

    // cmdPara : 'para' IDENT '<-' exp_aritmetica 'ate' exp_aritmetica 'faca' cmds 'fim_para'
    CMDPara {
        ident: Box<NoAST>,
        exp_aritmetica_1: Box<NoAST>,
        exp_aritmetica_2: Box<NoAST>,
        cmds: Box<NoAST>,
    },

    // cmdEnquanto : 'enquanto' expressao 'faca' cmds 'fim_enquanto'
    CMDEnquanto {
        expressao: Box<NoAST>,
        cmds: Box<NoAST>,
    },

    // cmdFaca : 'faca' cmds 'ate' expressao
    CMDFaca {
        cmds: Box<NoAST>,
        expressao: Box<NoAST>,
    },

    // cmdAtribuicao : circunflexo identificador '<-' expressao
    CMDAtribuicao {
        circunflexo: Box<NoAST>,
        identificador: Box<NoAST>,
        expressao: Box<NoAST>,
    },

    // cmdChamada : IDENT '(' expressao expressoes ')'
    CMDChamada {
        ident: Box<NoAST>,
        expressao: Box<NoAST>,
        expressoes: Box<NoAST>,
    },

    // cmdRetorne : 'retorne' expressao
    CMDRetorne { expressao: Box<NoAST> },

    // selecao : item_selecao selecao | <<vazio>>
    Selecao {
        item_selecao: Box<NoAST>,
        selecao: Box<NoAST>,
    },

    // item_selecao : constantes ':' cmds
    ItemSelecao {
        constantes: Box<NoAST>,
        cmds: Box<NoAST>,
    },

    // constantes : numero_intervalo numero_intervalos
    Constantes {
        numero_intervalo: Box<NoAST>,
        numero_intervalos: Box<NoAST>,
    },

    // numero_intervalo : op_unario NUM_INT numero_intervalo2
    NumeroIntervalo {
        op_unario: Box<NoAST>,
        num_int: Box<NoAST>,
        numero_intervalo2: Box<NoAST>,
    },
    
    // numero_intervalos : ',' numero_intervalo numero_intervalos | <<vazio>>
    NumeroIntervalos {
        numero_intervalo: Box<NoAST>,
        numero_intervalos: Box<NoAST>,
    },

    // numero_intervalo2 : '..' op_unario NUM_INT | <<vazio>>
    NumeroIntervalo2 {
        op_unario: Box<NoAST>,
        num_int: Box<NoAST>,
    },

    // op_unario : '-' | <<vazio>>
    OpUnario,

    // exp_aritmetica : termo termos
    ExpAritmetica {
        termo: Box<NoAST>,
        termos: Box<NoAST>,
    },

    // termo : fator fatores
    Termo {
        fator: Box<NoAST>,
        fatores: Box<NoAST>,
    },

    // termos : op1 termo termos | <<vazio>>
    Termos {
        op1: Box<NoAST>,
        termo: Box<NoAST>,
        termos: Box<NoAST>,
    },

    // op1 : '+' | '-'
    Op1 (Token),

    // fator : parcela parcelas
    Fator {
        parcela: Box<NoAST>,
        parcelas: Box<NoAST>,
    },

    // fatores : op2 fator fatores | <<vazio>>
    Fatores {
        op2: Box<NoAST>,
        fator: Box<NoAST>,
        fatores: Box<NoAST>,
    },
    
    // op2 : '*' | '/'
    Op2 (Token),
    
    // parcela : op_unario parcela_unario | parcela_nao_unario
    Parcela {
        op_unario: Box<NoAST>,
        parcela_unario: Box<NoAST>,
    },
    
    // parcelas : op3 parcela parcelas | <<vazio>>
    Parcelas {
        op3: Box<NoAST>,
        parcela: Box<NoAST>,
        parcelas: Box<NoAST>,
    },

    // op3 : '%'
    Op3,
    
    // parcela_unario : circunflexo identificador
    ParcelaUnario1 {
        circunflexo: Box<NoAST>,
        identificador: Box<NoAST>,
    },

    //     | IDENT '(' expressao expressoes ')'
    ParcelaUnario2 {
        ident: Box<NoAST>,
        expressao: Box<NoAST>,
        expressoes: Box<NoAST>,
    },

    //     | '(' expressao ')'
    ParcelaUnario3 { expressao: Box<NoAST> },

    //     | NUM_INT
    NumInt (Token),

    //     | NUM_REAL
    NumReal (Token),

    // parcela_nao_unario : '&' identificador
    ParcelaNaoUnario { identificador: Box<NoAST> },

    //     | CADEIA
    Cadeia (Token),

    // exp_relacional : exp_aritmetica exp_relacional2
    ExpRelacional {
        exp_aritmetica: Box<NoAST>,
        exp_relacional2: Box<NoAST>,
    },

    // exp_relacional2 : op_relacional exp_aritmetica | <<vazio>>
    ExpRelacional2 {
        op_relacional: Box<NoAST>,
        exp_aritmetica: Box<NoAST>,
    },

    // op_relacional : '=' | '<>' | '>=' | '<=' | '>' | '<'
    OpRelacional (Token),
    
    // expressao : termo_logico termos_logicos
    Expressao {
        termo_logico: Box<NoAST>,
        termos_logicos: Box<NoAST>,
    },

    // expressoes : ',' expressao expressoes | <<vazio>>
    Expressoes {
        expressao: Box<NoAST>,
        expressoes: Box<NoAST>,
    },

    // termo_logico : fator_logico fatores_logicos
    TermoLogico {
        fator_logico: Box<NoAST>,
        fatores_logicos: Box<NoAST>,
    },

    // termos_logicos : op_logico_1 termo_logico termos_logicos | <<vazio>>
    TermosLogicos {
        op_logico_1: Box<NoAST>,
        termo_logico: Box<NoAST>,
        termos_logicos: Box<NoAST>,
    },

    // fator_logico : nao parcela_logica
    FatorLogico {
        nao: Box<NoAST>,
        parcela_logica: Box<NoAST>,
    },
    // parcela_logica : constante_logica
    //     | exp_relacional
    
    // fatores_logicos : op_logico_2 fator_logico fatores_logicos | <<vazio>>
    FatoresLogicos {
        op_logico_2: Box<NoAST>,
        fator_logico: Box<NoAST>,
        fatores_logicos: Box<NoAST>,
    },
    
    // nao : 'nao' | <<vazio>>
    Nao,
    
    // constante_logica : 'verdadeiro' | 'falso'
    ConstanteLogica(Token),
    
    // op_logico_1 : 'ou'
    OpLogico1,

    // op_logico_2 : 'e'
    OpLogico2,

    Vazio,
    Erro { mensagem: String },
}

impl NoAST {
    pub fn get_tokens(&self) -> Vec<Token> {
        match self {
            NoAST::ValorConstante (token)
            | NoAST::Ident (token)
            | NoAST::TipoBasico (token)
            | NoAST::Op1 (token)
            | NoAST::Op2 (token)
            | NoAST::NumInt (token)
            | NoAST::NumReal (token)
            | NoAST::Cadeia (token)
            | NoAST::OpRelacional (token) => vec![token.clone()],
            _ => vec![]
        }
    }
    
    pub fn get_idents(&self) -> Vec<Token> {
        match self {
            NoAST::Ident (token) => vec![token.clone()],
            NoAST::Identificador { ident, identificador2, dimensao: _ } => {
                let mut idents = ident.get_idents();
                idents.append(&mut identificador2.get_idents());
                idents
            }
            NoAST::Identificador2 { ident, identificador2 } => {
                let mut idents = ident.get_idents();
                idents.append(&mut identificador2.get_idents());
                idents
            }
            NoAST::Identificadores { identificador, identificadores } => {
                let mut idents = identificador.get_idents();
                idents.append(&mut identificadores.get_idents());
                idents
            }
            NoAST::Variavel { identificador, identificadores, tipo: _ } => {
                let mut idents = identificador.get_idents();
                idents.append(&mut identificadores.get_idents());
                idents
            }
            _ => vec![]
        }
    }
}