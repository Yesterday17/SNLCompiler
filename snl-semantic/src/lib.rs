use crate::symbol::{SymbolTable, Symbol};
use snl_rdp::Program;
use snl_rdp::models::*;
use crate::error::Error;
use std::cell::RefCell;

mod error;
pub mod symbol;

pub struct Semantic {
    ast: Positional<Program>,
    symbols: RefCell<SymbolTable<Symbol>>,
    errors: RefCell<Vec<Positional<Error>>>,
}

impl Semantic {
    pub fn new(ast: Positional<Program>) -> Self {
        let result = Semantic {
            ast,
            symbols: Default::default(),
            errors: Default::default(),
        };
        result
    }

    pub fn analyze(self) -> Vec<Positional<Error>> {
        self.analyze_declare(&self.ast.declare);
        self.analyze_statement_list(&self.ast.body);
        self.errors.into_inner()
    }

    fn analyze_declare(&self, declare: &ProgramDeclare) {
        // type alias = original;
        for t in declare.type_declare.iter() {
            // alias exist in Symbol Table
            if self.symbols.borrow().has_own_property(t.name()) {
                self.errors.borrow_mut().push(Positional::from_position(
                    t.position(),
                    Error::DuplicatedIdentifier(t.name().to_owned()),
                ))
            } else {
                // analyze whether original is valid
                self.analyze_type(&t.inner().base());
                // create new alias type
                self.symbols.borrow_mut().insert(t.name().to_owned(), Symbol::Type(t.inner().base_raw()));
            }
        }

        for v in declare.variable_declare.iter() {
            // check variable type
            self.analyze_type(&Positional::from_position(v.position(), &v.type_name));
            // check variable name
            for variable_name in v.identifiers.iter() {
                if self.symbols.borrow().has_own_property(variable_name) {
                    self.errors.borrow_mut().push(Positional::from_position(
                        variable_name.position(),
                        Error::DuplicatedIdentifier(variable_name.inner().clone()),
                    ))
                } else {
                    self.symbols.borrow_mut().insert(variable_name.inner().clone(), Symbol::Variable(v.type_name.clone()));
                }
            }
        }

        for p in declare.procedure_declare.iter() {
            // check procedure name
            if self.symbols.borrow().has_own_property(p.name()) {
                self.errors.borrow_mut().push(Positional::from_position(
                    p.position(),
                    Error::DuplicatedIdentifier(p.name().to_owned()),
                ));
            } else {
                // add procedure to Symbol Table
                self.symbols.borrow_mut().insert(p.name().to_string(), Symbol::Procedure(/* TODO */));
            }

            // start analyzing current procedure
            self.symbols.borrow_mut().step_in();

            // add procedure itself to symbol table of current tier
            self.symbols.borrow_mut().insert(p.name().to_string(), Symbol::Procedure(/* TODO */));

            // parameters
            for param in p.params.iter() {
                // TODO param.inner
            }

            // check declare
            self.analyze_declare(&p.declare);

            // check body
            self.analyze_statement_list(&p.body);

            // finish analyzing current procedure
            self.symbols.borrow_mut().step_out();
        }
    }

    fn analyze_statement_list(&self, list: &StatementList) {
        //
    }

    fn analyze_type(&self, t: &Positional<&SNLType>) {
        match t.inner() {
            SNLType::Array(arr) => {
                // check array definition bounds
                if arr.lower_bound > arr.upper_bound {
                    self.errors.borrow_mut().push(Positional::from_position(
                        t.position(),
                        Error::InvalidArrayDefinition,
                    ));
                }
            }
            SNLType::Record(rec) => {
                // TODO
            }
            SNLType::Others(id) => {
                if !self.symbols.borrow().has_own_property(id) {
                    self.errors.borrow_mut().push(Positional::from_position(
                        t.position(),
                        Error::UndefinedType(id.to_owned()),
                    ))
                }
            }
            _ => {}
        }
    }
}
