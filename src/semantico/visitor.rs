use crate::sintatico::arvore_sintatica::NoAST;

pub trait Visitor {
    fn visit(&mut self, no: &NoAST);

    fn traverse(&mut self, no: &NoAST) {
        self.visit(no);
        for filho in no.filhos() {
            self.traverse(filho)
        }
    }
}
