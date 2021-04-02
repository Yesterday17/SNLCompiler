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
                // param type
                self.analyze_type(&Positional::from_position(
                    param.position(),
                    &param.definition.type_name,
                ));
                // param name
                for param_name in param.definition.identifiers.iter() {
                    if self.symbols.borrow().has_own_property(param_name.as_str()) {
                        self.errors.borrow_mut().push(Positional::from_position(
                            param_name.position(),
                            Error::DuplicatedIdentifier(param_name.inner().to_owned()),
                        ));
                    } else {
                        self.symbols.borrow_mut().insert(
                            param_name.inner().clone(),
                            Symbol::Variable(param.definition.type_name.clone()),
                        );
                    }
                }
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
        for statement in list.iter() {
            match statement {
                Statement::Conditional(_) => {}
                Statement::Loop(_) => {}
                Statement::Input(_) => {}
                Statement::Output(_) => {}
                Statement::Return(_) => {}
                Statement::Assign(_) => {}
                Statement::Call(call) => {
                    // look for symbol in table
                    match self.symbols.borrow().query(call.name()) {
                        Some(symbol) => {
                            match symbol {
                                // TODO: check procedure signature
                                Symbol::Procedure() => {}
                                // idenfier called is not procedure
                                p => {
                                    self.errors.borrow_mut().push(Positional::from_position(
                                        call.position(),
                                        Error::TypeMismatch("Procedure".to_owned(), format!("{:?}", p)),
                                    ))
                                }
                            }
                        }
                        None => {
                            // no symbol found
                            self.errors.borrow_mut().push(Positional::from_position(
                                call.position(),
                                Error::UndefinedIdentifier(call.name().to_owned()),
                            ))
                        }
                    }
                }
            }
        }
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
