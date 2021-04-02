use crate::symbol::SymbolTable;
use snl_rdp::Program;
use snl_rdp::models::{ProgramDeclare, ASTNode};

mod error;
mod symbol;

pub struct Semantic {
    ast: ASTNode<Program>,
    symbols: SymbolTable<String>,
}

impl Semantic {
    pub fn new(ast: ASTNode<Program>) -> Self {
        Semantic { ast, symbols: SymbolTable::new() }
    }

    pub fn analyze(&mut self) {
        let declare = &self.ast.declare;
    }

    fn analyze_declare(&self, declare: &ProgramDeclare) {
        //
    }
}
