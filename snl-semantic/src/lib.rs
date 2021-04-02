use crate::symbol::SymbolTable;
use snl_rdp::Program;
use snl_rdp::models::*;
use crate::error::Error;
use std::cell::RefCell;

mod error;
mod symbol;

pub struct Semantic {
    ast: Positional<Program>,
    symbols: RefCell<SymbolTable<String>>,
    errors: RefCell<Vec<Positional<Error>>>,
}

impl Semantic {
    pub fn new(ast: Positional<Program>) -> Self {
        Semantic {
            ast,
            symbols: Default::default(),
            errors: Default::default(),
        }
    }

    pub fn analyze(self) -> Vec<Positional<Error>> {
        self.analyze_declare(&self.ast.declare);
        self.analyze_statement_list(&self.ast.body);
        self.errors.into_inner()
    }

    fn analyze_declare(&self, declare: &ProgramDeclare) {
        for t in &declare.type_declare {
            if self.symbols.borrow().has_own_property(t.name()) {
                self.errors.borrow_mut().push(Positional::from_position(
                    t.position(),
                    Error::DuplicatedIdentifier(t.name().to_owned()),
                ))
            } else {
                self.symbols.borrow_mut().insert(t.name().to_owned(), "".to_owned());
            }
        }
    }

    fn analyze_statement_list(&self, list: &StatementList) {
        //
    }
}
