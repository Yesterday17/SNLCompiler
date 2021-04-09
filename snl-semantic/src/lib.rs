use crate::symbol::{SymbolTable, Symbol};
use snl_rdp::Program;
use snl_utils::ast::*;
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

                let mut symbols = self.symbols.borrow_mut();
                // get real type of aliased type
                let ty = Symbol::Type(t.inner().base().inner().to_string(|ty| -> Option<String>{
                    symbols.query_type(ty).map(|r| r.to_string())
                }));
                // add new type alias
                symbols.insert(t.name().to_owned(), ty);
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
                    let mut symbols = self.symbols.borrow_mut();
                    // look for real type(deref alias)
                    let ty = Symbol::Variable(v.type_name.to_string(|ty| -> Option<String>{
                        symbols.query_type(ty).map(|r| r.to_string())
                    }));
                    // insert variable to symbol table
                    symbols.insert(variable_name.inner().clone(), ty);
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
                            Symbol::Variable(param.definition.type_name.to_string(|r| Some(r.to_string()))),
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
                Statement::Input(input) => {
                    match self.symbols.borrow().query(input) {
                        Some(symbol) => {
                            match symbol {
                                Symbol::Variable(variable) => {
                                    // only accept integer or char
                                    if variable.starts_with("[") || variable.starts_with("{") || variable.starts_with("#") {
                                        self.errors.borrow_mut().push(Positional::from_position(
                                            input.position(),
                                            Error::InvalidReadType(variable.clone()),
                                        ))
                                    }
                                }
                                p => {
                                    self.errors.borrow_mut().push(Positional::from_position(
                                        input.position(),
                                        Error::UncompatableType("Variable".to_owned(), format!("{:?}", p)),
                                    ))
                                }
                            }
                        }
                        None => {
                            // no symbol found
                            self.errors.borrow_mut().push(Positional::from_position(
                                input.position(),
                                Error::UndefinedIdentifier(input.inner().to_owned()),
                            ))
                        }
                    }
                }
                Statement::Output(output) => {
                    let write_type = self.analyze_expression(output);
                    match write_type.as_str() {
                        // integer and char are valid types
                        // skip mixex type(empty string)
                        "integer" | "char" | "" => {}
                        _ => {
                            self.errors.borrow_mut().push(Positional::from_position(
                                output.left.position(),
                                Error::InvalidWriteType(write_type),
                            ))
                        }
                    }
                }
                Statement::Return(ret) => {
                    self.analyze_expression(ret);
                }
                Statement::Assign(assign) => {
                    let left_type = self.analyze_variable_represent(&assign.variable);
                    let right_type = self.analyze_expression(&assign.value);

                    if left_type == "" {
                        self.errors.borrow_mut().push(Positional::from_position(
                            assign.variable.base.position(),
                            Error::InvalidAssignee,
                        ))
                    } else if left_type != right_type {
                        self.errors.borrow_mut().push(Positional::from_position(
                            assign.value.left.position(),
                            Error::AssignTypeMismatch(right_type, left_type),
                        ))
                    }
                }
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
                                        Error::UncompatableType("Procedure".to_owned(), format!("{:?}", p)),
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

    fn analyze_expression(&self, exp: &Expression) -> String {
        let left_type = self.analyze_expression_term(exp.left.inner());
        match &exp.right {
            Some(right) => {
                let right_type = self.analyze_expression(right.inner());
                if left_type != right_type {
                    self.errors.borrow_mut().push(Positional::from_position(
                        exp.left.position(),
                        Error::UncompatableType(right_type, left_type.clone()),
                    ))
                }
            }
            None => {}
        }
        left_type
    }

    fn analyze_expression_term(&self, exp: &ExpressionTerm) -> String {
        let left_type = self.analyze_expression_factor(exp.left.inner());
        match &exp.right {
            Some(right) => {
                let right_type = self.analyze_expression_term(right.inner());
                if left_type != right_type {
                    self.errors.borrow_mut().push(Positional::from_position(
                        exp.left.position(),
                        Error::UncompatableType(right_type, left_type.clone()),
                    ))
                }
            }
            None => {}
        }
        left_type
    }

    fn analyze_expression_factor(&self, exp: &ExpressionFactor) -> String {
        match exp {
            ExpressionFactor::Bracket(exp) => self.analyze_expression(exp),
            ExpressionFactor::Constant(_) => SNLType::Integer.to_string(|r| Some(r.to_string())),
            ExpressionFactor::Variable(repr) => self.analyze_variable_represent(repr),
        }
    }

    fn analyze_variable_represent(&self, repr: &VariableRepresent) -> String {
        match self.symbols.borrow().query(&repr.base) {
            Some(symbol) => {
                // found symbol in table
                match symbol {
                    Symbol::Variable(current_type) => {
                        // the only valid variable represent base is Variable
                        let mut current_type = current_type.to_owned();
                        // if visit exist
                        if let Some(visit) = &repr.visit {
                            // record field
                            if let Some(field) = &visit.dot {
                                // only record type can be visited
                                if !current_type.starts_with("{") {
                                    self.errors.borrow_mut().push(Positional::from_position(
                                        field.position(),
                                        Error::InvalidFieldIndexType(current_type.clone()),
                                    ));
                                    current_type.clear();
                                } else {
                                    let mut type_got = String::new();
                                    let parts: Vec<_> = current_type[1..(current_type.len() - 1)].split(";").collect();
                                    for p in parts {
                                        let fields: Vec<_> = p.split(":").collect();
                                        let variables: Vec<_> = fields[0].split(",").collect();
                                        if variables.contains(&field.as_str()) {
                                            type_got += fields[1];
                                            break;
                                        }
                                    }
                                    // field not found in record
                                    if type_got == "" {
                                        self.errors.borrow_mut().push(Positional::from_position(
                                            field.position(),
                                            Error::UndefinedRecordField(field.inner().clone()),
                                        ));
                                    }
                                    current_type = type_got;
                                }
                            }

                            // array index
                            if !current_type.is_empty() {
                                if let Some(index) = &visit.sqbr {
                                    // only array type can be indexed
                                    if !current_type.starts_with("[") {
                                        self.errors.borrow_mut().push(Positional::from_position(
                                            index.left.position(),
                                            Error::UnexpectedArrayIndex,
                                        ));
                                        current_type.clear();
                                    } else {
                                        // get index type
                                        let index_type = self.analyze_expression(index);
                                        // only integer is valid index type
                                        if index_type != "integer" {
                                            self.errors.borrow_mut().push(Positional::from_position(
                                                index.left.position(),
                                                Error::UncompatableType(index_type, "integer".to_owned()),
                                            ));
                                            current_type.clear();
                                        } else {
                                            // return type
                                            current_type = if current_type.ends_with("integer]") {
                                                "integer".to_owned()
                                            } else {
                                                "char".to_owned()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        current_type
                    }
                    Symbol::Procedure() => {
                        // procedure is not a valid **variable** represent
                        self.errors.borrow_mut().push(Positional::from_position(
                            repr.base.position(),
                            Error::InvalidVariableRepresent("Procedure".to_owned()),
                        ));
                        String::new()
                    }
                    Symbol::Type(symbol_type) => {
                        // type (alias) is not a valid **variable** represent
                        self.errors.borrow_mut().push(Positional::from_position(
                            repr.base.position(),
                            Error::InvalidVariableRepresent(symbol_type.clone()),
                        ));
                        String::new()
                    }
                }
            }
            None => {
                // repr.base not found in symbol table
                self.errors.borrow_mut().push(Positional::from_position(
                    repr.base.position(),
                    Error::UndefinedIdentifier(repr.base.inner().clone()),
                ));
                "".to_owned()
            }
        }
    }
}
