use crate::sintatico::arvore_sintatica::NoAST;

pub trait Visitor {
    fn visit(&mut self, no: &NoAST);

    fn traverse(&mut self, no: &NoAST) {
        self.visit(no);
        match no {
            NoAST::Programa { declaracoes, corpo } => {
                self.traverse(declaracoes);
                self.traverse(corpo);
            }
            NoAST::Declaracoes { declaracao, declaracoes } => {
                self.traverse(declaracao);
                self.traverse(declaracoes);
            }
            NoAST::DeclaracaoVariavel { variavel } => {
                self.traverse(variavel)
            }
            NoAST::DeclaracaoTipo { ident, tipo } => {
                self.traverse(ident);
                self.traverse(tipo);
            }
            NoAST::DeclaracaoConstante { ident, tipo_basico, valor_constante } => {
                self.traverse(ident);
                self.traverse(tipo_basico);
                self.traverse(valor_constante);
            }
            NoAST::ValorConstante (Token) => {
            
            }
            NoAST::Variavel { identificador, identificadores, tipo } => {
                self.traverse(identificador);
                self.traverse(identificadores);
                self.traverse(tipo);
            }
            NoAST::Identificador { ident, identificador2, dimensao } => {
                self.traverse(ident);
                self.traverse(identificador2);
                self.traverse(dimensao);
            }
            NoAST::Identificador2 { ident, identificador2 } => {
                self.traverse(ident);
                self.traverse(identificador2);
            }
            NoAST::Identificadores { identificador, identificadores } => {
                self.traverse(identificador);
                self.traverse(identificadores);
            }
            NoAST::Dimensao { exp_aritmetica, dimensao } => {
                self.traverse(exp_aritmetica);
                self.traverse(dimensao);
            }
            NoAST::TipoExtendido { circunflexo, tipo_basico_ident } => {
                self.traverse(circunflexo);
                self.traverse(tipo_basico_ident);
            }
            NoAST::Ident (Token) => {
            
            }
            NoAST::TipoBasico (Token) => {
            
            }
            NoAST::Circunflexo => {
            
            }
            NoAST::Registro { variaveis } => {
                self.traverse(variaveis);
            }
            NoAST::Variaveis { variavel, variaveis } => {
                self.traverse(variavel);
                self.traverse(variaveis);
            }
            NoAST::DeclaracaoProcedimento { ident, parametros, declaracoes_locais, cmds } => {
                self.traverse(ident);
                self.traverse(parametros);
                self.traverse(declaracoes_locais);
                self.traverse(cmds);
            }
            NoAST::DeclaracaoFuncao { ident, parametros, tipo_estendido, declaracoes_locais, cmds } => {
                self.traverse(ident);
                self.traverse(parametros);
                self.traverse(tipo_estendido);
                self.traverse(declaracoes_locais);
                self.traverse(cmds);
            }
            NoAST::DeclaracoesLocais { declaracao_local, declaracoes_locais } => {
                self.traverse(declaracao_local);
                self.traverse(declaracoes_locais);
            }
            NoAST::Parametro { var, identificador, identificadores, tipo_estendido } => {
                self.traverse(var);
                self.traverse(identificador);
                self.traverse(identificadores);
                self.traverse(tipo_estendido);
            }
            NoAST::Parametros { parametro, parametros2 } => {
                self.traverse(parametro);
                self.traverse(parametros2);
            }
            NoAST::Parametros2 { parametro, parametros2 } => {
                self.traverse(parametro);
                self.traverse(parametros2);
            }
            NoAST::Var => {
            
            }
            NoAST::Corpo { declaracoes_locais, cmds } => {
                self.traverse(declaracoes_locais);
                self.traverse(cmds);
            }
            NoAST::CMDs { cmd, cmds } => {
                self.traverse(cmd);
                self.traverse(cmds);
            }
            NoAST::CMDLeia { circunflexo, identificador, cmd_leia2 } => {
                self.traverse(circunflexo);
                self.traverse(identificador);
                self.traverse(cmd_leia2);
            }
            NoAST::CMDLeia2 { circunflexo, identificador, cmd_leia2 } => {
                self.traverse(circunflexo);
                self.traverse(identificador);
                self.traverse(cmd_leia2);
            }
            NoAST::CMDEscreva { expressao, expressoes } => {
                self.traverse(expressao);
                self.traverse(expressoes);
            }
            NoAST::CMDSe { expressao, cmds, senao } => {
                self.traverse(expressao);
                self.traverse(cmds);
                self.traverse(senao);
            }
            NoAST::Senao { cmds } => {
                self.traverse(cmds)
            }
            NoAST::CMDCaso { exp_aritmetica, selecao, senao } => {
                self.traverse(exp_aritmetica);
                self.traverse(selecao);
                self.traverse(senao);
            }
            NoAST::CMDPara { ident, exp_aritmetica_1, exp_aritmetica_2, cmds } => {
                self.traverse(ident);
                self.traverse(exp_aritmetica_1);
                self.traverse(exp_aritmetica_2);
                self.traverse(cmds);
            }
            NoAST::CMDEnquanto { expressao, cmds } => {
                self.traverse(expressao);
                self.traverse(cmds);
            }
            NoAST::CMDFaca { cmds, expressao } => {
                self.traverse(cmds);
                self.traverse(expressao);
            }
            NoAST::CMDAtribuicao { circunflexo, identificador, expressao } => {
                self.traverse(circunflexo);
                self.traverse(identificador);
                self.traverse(expressao);
            }
            NoAST::CMDChamada { ident, expressao, expressoes } => {
                self.traverse(ident);
                self.traverse(expressao);
                self.traverse(expressoes);
            }
            NoAST::CMDRetorne { expressao } => {
                self.traverse(expressao)
            }
            NoAST::Selecao { item_selecao, selecao } => {
                self.traverse(item_selecao);
                self.traverse(selecao);
            }
            NoAST::ItemSelecao { constantes, cmds } => {
                self.traverse(constantes);
                self.traverse(cmds);
            }
            NoAST::Constantes { numero_intervalo, numero_intervalos } => {
                self.traverse(numero_intervalo);
                self.traverse(numero_intervalos);
            }
            NoAST::NumeroIntervalo { op_unario, num_int, numero_intervalo2 } => {
                self.traverse(op_unario);
                self.traverse(num_int);
                self.traverse(numero_intervalo2);
            }
            NoAST::NumeroIntervalos { numero_intervalo, numero_intervalos } => {
                self.traverse(numero_intervalo);
                self.traverse(numero_intervalos);
            }
            NoAST::NumeroIntervalo2 { op_unario, num_int } => {
                self.traverse(op_unario);
                self.traverse(num_int);
            }
            NoAST::OpUnario => {
            
            }
            NoAST::ExpAritmetica { termo, termos } => {
                self.traverse(termo);
                self.traverse(termos);
            }
            NoAST::Termo { fator, fatores } => {
                self.traverse(fator);
                self.traverse(fatores);
            }
            NoAST::Termos { op1, termo, termos } => {
                self.traverse(op1);
                self.traverse(termo);
                self.traverse(termos);
            }
            NoAST::Op1 (Token) => {
            
            }
            NoAST::Fator { parcela, parcelas } => {
                self.traverse(parcela);
                self.traverse(parcelas);
            }
            NoAST::Fatores { op2, fator, fatores } => {
                self.traverse(op2);
                self.traverse(fator);
                self.traverse(fatores);
            }
            NoAST::Op2 (Token) => {
            
            }
            NoAST::Parcela { op_unario, parcela_unario } => {
                self.traverse(op_unario);
                self.traverse(parcela_unario);
            }
            NoAST::Parcelas { op3, parcela, parcelas } => {
                self.traverse(op3);
                self.traverse(parcela);
                self.traverse(parcelas);
            }
            NoAST::Op3 => {
            
            }
            NoAST::ParcelaUnario1 { circunflexo, identificador } => {
                self.traverse(circunflexo);
                self.traverse(identificador);
            }
            NoAST::ParcelaUnario2 { ident, expressao, expressoes } => {
                self.traverse(ident);
                self.traverse(expressao);
                self.traverse(expressoes);
            }
            NoAST::ParcelaUnario3 { expressao } => {
                self.traverse(expressao)
            }
            NoAST::NumInt (Token) => {
            
            }
            NoAST::NumReal (Token) => {
            
            }
            NoAST::ParcelaNaoUnario { identificador } => {
                self.traverse(identificador)
            }
            NoAST::Cadeia (Token) => {
            
            }
            NoAST::ExpRelacional { exp_aritmetica, exp_relacional2 } => {
                self.traverse(exp_aritmetica);
                self.traverse(exp_relacional2);
            }
            NoAST::ExpRelacional2 { op_relacional, exp_aritmetica } => {
                self.traverse(op_relacional);
                self.traverse(exp_aritmetica);
            }
            NoAST::OpRelacional (Token) => {
            
            }
            NoAST::Expressao { termo_logico, termos_logicos } => {
                self.traverse(termo_logico);
                self.traverse(termos_logicos);
            }
            NoAST::Expressoes { expressao, expressoes } => {
                self.traverse(expressao);
                self.traverse(expressoes);
            }
            NoAST::TermoLogico { fator_logico, fatores_logicos } => {
                self.traverse(fator_logico);
                self.traverse(fatores_logicos);
            }
            NoAST::TermosLogicos { op_logico_1, termo_logico, termos_logicos } => {
                self.traverse(op_logico_1);
                self.traverse(termo_logico);
                self.traverse(termos_logicos);
            }
            NoAST::FatorLogico { nao, parcela_logica } => {
                self.traverse(nao);
                self.traverse(parcela_logica);
            }
            NoAST::FatoresLogicos { op_logico_2, fator_logico, fatores_logicos } => {
                self.traverse(op_logico_2);
                self.traverse(fator_logico);
                self.traverse(fatores_logicos);
            }
            NoAST::Nao => {
            
            }
            NoAST::ConstanteLogica(Token) => {
            
            }
            NoAST::OpLogico1 => {
            
            }
            NoAST::OpLogico2 => {
            
            }
            NoAST::Vazio => {
            
            }
            NoAST::Erro { mensagem: String } => {
            
            }
        }
    }
}
