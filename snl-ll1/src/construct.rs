use snl_utils::token::{Token, TokenType};
use snl_utils::ast::*;
use std::collections::HashMap;
use std::str::FromStr;

pub enum ASTNodeValue {
    /// Used when rule A -> '' is used
    None,

    Terminal(Token),
    Int(usize),
    String(String),

    Program(Positional<Program>),
    ProgramHead(Positional<String>),

    DeclarePart(ProgramDeclare),

    TypeDeclaration(PositionalVec<TypeDeclare>),
    TypeName(Positional<SNLType>),
    BaseType(Positional<SNLBaseType>),
    ArrayType(Positional<SNLTypeArray>),
    RecordType(SNLTypeRecord),

    VarDeclaration(PositionalVec<TypedIdentifiers>),
    IdentifierList(PositionalVec<String>),
    ProcedureDeclaration(PositionalVec<ProcedureDeclare>),

    Param(Positional<Param>),
    ParamList(PositionalVec<Param>),
    FieldDecList(Vec<TypedIdentifiers>),

    Statement(Statement),
    StatementList(StatementList),

    VariableVisit(VariableVisit),
    VariableVisitDot(Positional<String>),
    VariableVisitSqbr(Box<Expression>),
    CallStatementRest(Vec<Expression>),
    AssignStatementRest((Option<VariableVisit>, Expression)),

    Operator(String),
    Variable(VariableRepresent),
    Factor(Positional<ExpressionFactor>),
    Term(ExpressionTerm),
    TermPostFix((String, Positional<Box<ExpressionTerm>>)),

    Expression(Expression),
    ExpressionPostFix((String, Positional<Box<Expression>>)),
}

pub struct ConstructTable(HashMap<&'static str, fn(Vec<ASTNodeValue>) -> Result<ASTNodeValue, String>>);

impl Default for ConstructTable {
    fn default() -> Self {
        let mut table = ConstructTable(Default::default());
        table.0.insert("Program", construct_program);
        table.0.insert("ProgramHead", construct_program_head);
        table.0.insert("ProgramName", construct_program_name);
        table.0.insert("DeclarePart", construct_declare_part);
        table.0.insert("TypeDec", construct_type_dec);
        table.0.insert("TypeDeclaration", construct_type_declaration);
        table.0.insert("TypeDecList", construct_type_dec_list);
        table.0.insert("TypeDecListMore", construct_type_dec_list_more);
        table.0.insert("TypeId", construct_type_id);
        table.0.insert("TypeName", construct_type_name);
        table.0.insert("BaseType", construct_base_type);
        table.0.insert("StructureType", construct_structure_type);
        table.0.insert("ArrayType", construct_array_type);
        table.0.insert("Low", construct_int);
        table.0.insert("Top", construct_int);
        table.0.insert("RecordType", construct_record_type);
        table.0.insert("FieldDecList", construct_field_dec_list);
        table.0.insert("FieldDecListMore", construct_field_dec_list_more);
        table.0.insert("FieldDecType", construct_field_dec_type);
        table.0.insert("IdentifierList", construct_identifier_list);
        table.0.insert("IdentifierListMore", construct_identifier_list_more);
        table.0.insert("VarDec", construct_var_dec);
        table.0.insert("VarDeclaration", construct_var_declaration);
        table.0.insert("VarDecList", construct_var_dec_list);
        table.0.insert("VarDecListMore", construct_var_dec_list_more);
        table.0.insert("ProcDec", construct_proc_dec);
        table.0.insert("ProcDeclaration", construct_proc_declaration);
        table.0.insert("ProcName", construct_proc_name);
        table.0.insert("ParamList", construct_param_list);
        table.0.insert("ParamListMore", construct_param_list_more);
        table.0.insert("Param", construct_param);
        table.0.insert("ProcDecPart", construct_proc_dec_part);
        table.0.insert("ProcBody", construct_proc_body);
        table.0.insert("ProgramBody", construct_program_body);
        table.0.insert("StatementList", construct_statement_list);
        table.0.insert("MoreStatement", construct_more_statement);
        table.0.insert("Statement", construct_statement);
        table.0.insert("AssCall", construct_ass_call);
        table.0.insert("AssignmentRest", construct_assignment_rest);
        table.0.insert("ConditionalStatement", construct_conditional_statement);
        table.0.insert("LoopStatement", construct_loop_statement);
        table.0.insert("InputStatement", construct_input_statement);
        table.0.insert("OutputStatement", construct_output_statement);
        table.0.insert("ReturnStatement", construct_return_statement);
        table.0.insert("CallStatementRest", construct_call_statement_rest);
        table.0.insert("CallStatementRestExp", construct_call_statement_rest_exp);
        table.0.insert("CommaExp", construct_comma_exp);
        table.0.insert("RelExp", construct_rel_exp);
        table.0.insert("Exp", construct_exp);
        table.0.insert("ExpPostFix", construct_exp_postfix);
        table.0.insert("Term", construct_term);
        table.0.insert("TermPostFix", construct_term_postfix);
        table.0.insert("Factor", construct_factor);
        table.0.insert("Variable", construct_variable);
        table.0.insert("VariableVisit", construct_variable_visit);
        table.0.insert("VariableVisitField", construct_variable_visit_field);
        table.0.insert("VariableVisitIndex", construct_variable_visit_index);
        table.0.insert("CmdOp", construct_op);
        table.0.insert("AddOp", construct_op);
        table.0.insert("MultOp", construct_op);

        table
    }
}

impl ConstructTable {
    pub fn construct(&self, ty: &'static str, input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
        self.0[ty](input)
    }
}

macro_rules! pop {
    ($v: ident) => {
        $v.pop().unwrap()
    };
    ($v: ident, $t: expr) => {
        for _ in 0..$t {
            $v.pop().unwrap();
        }
    };
}

macro_rules! token {
    ($v: ident) => {
        match pop!($v) {
            ASTNodeValue::Terminal(token) => token,
            _ => unreachable!()
        }
    };
}

macro_rules! node {
    ($input: ident, $ext: ident) => {
        match pop!($input) {
            ASTNodeValue::$ext(result) => result,
            _ => unreachable!()
        }
    };
}

macro_rules! node_default {
    ($input: ident, $ext: ident) => {
        match pop!($input) {
            ASTNodeValue::$ext(result) => result,
            ASTNodeValue::None         => Default::default(),
            _ => unreachable!()
        }
    };
}

macro_rules! node_optional {
    ($input: ident, $ext: ident) => {
        match pop!($input) {
            ASTNodeValue::$ext(result) => Some(result),
            ASTNodeValue::None         => None,
            _ => unreachable!()
        }
    };
}

macro_rules! identifier {
    ($input: ident) => {
        Positional::from_token_image_raw(token!($input))
    };
}

fn construct_program(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let name = node!(input, ProgramHead);
    let declare = node!(input, DeclarePart);
    let body = node!(input, StatementList);
    Ok(ASTNodeValue::Program(Positional::from_position(name.position(), Program {
        name: name.into_inner(),
        declare,
        body,
    })))
}

fn construct_program_head(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    pop!(input);
    Ok(input.pop().unwrap())
}

fn construct_program_name(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(ASTNodeValue::ProgramHead(identifier!(input)))
}

fn construct_declare_part(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let type_declare = node_default!(input, TypeDeclaration);
    let variable_declare = node_default!(input, VarDeclaration);
    let procedure_declare = node_default!(input, ProcedureDeclaration);
    Ok(ASTNodeValue::DeclarePart(ProgramDeclare {
        type_declare,
        variable_declare,
        procedure_declare,
    }))
}

fn construct_type_dec(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let result = node_default!(input, TypeDeclaration);
    Ok(ASTNodeValue::TypeDeclaration(result))
}

fn construct_type_declaration(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    pop!(input);
    Ok(pop!(input))
}

fn construct_type_dec_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let name = node!(input, String);
    pop!(input);
    let base = node!(input,TypeName);
    pop!(input);

    let mut more = node_default!(input, TypeDeclaration);
    more.insert(0, Positional::from_position(
        base.position(),
        TypeDeclare {
            base,
            name,
        },
    ));
    Ok(ASTNodeValue::TypeDeclaration(more))
}

fn construct_type_dec_list_more(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(pop!(input))
}

fn construct_type_id(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(ASTNodeValue::String(token!(input).image))
}

fn construct_type_name(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(ASTNodeValue::TypeName(match pop!(input) {
        ASTNodeValue::BaseType(ty) => Positional::from_position(ty.position(), ty.into_inner().into()),
        ASTNodeValue::ArrayType(ty) => Positional::from_position(ty.position(), SNLType::Array(ty.into_inner())),
        ASTNodeValue::RecordType(ty) => Positional::from_position(ty[0].type_name.position(), SNLType::Record(ty)),
        ASTNodeValue::Terminal(token) => {
            Positional::from_position(token.position(), SNLType::Others(token.image))
        }
        _ => unreachable!(),
    }))
}

fn construct_base_type(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let token = token!(input);
    Ok(ASTNodeValue::BaseType(Positional::from_position(token.position(), match token.token_type {
        TokenType::Integer => SNLBaseType::Integer,
        TokenType::Char => SNLBaseType::Char,
        _ => unreachable!(),
    })))
}

fn construct_structure_type(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(pop!(input))
}

fn construct_array_type(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let arr = token!(input);
    pop!(input);
    let low = node!(input, Int);
    pop!(input);
    let top = node!(input, Int);
    pop!(input);
    pop!(input);
    let base_type = node!(input, BaseType);
    Ok(ASTNodeValue::ArrayType(Positional::from_position(arr.position(), SNLTypeArray {
        base: base_type.into_inner(),
        lower_bound: low,
        upper_bound: top,
    })))
}

fn construct_int(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(ASTNodeValue::Int(usize::from_str(&token!(input).image).unwrap()))
}

fn construct_record_type(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    pop!(input);
    let list = node!(input, FieldDecList);
    Ok(ASTNodeValue::RecordType(list))
}

fn construct_field_dec_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let type_name = node!(input, TypeName);
    let identifiers = node!(input, IdentifierList);
    pop!(input);
    let mut list = node_default!(input, FieldDecList);
    list.insert(0, TypedIdentifiers {
        type_name,
        identifiers,
    });
    Ok(ASTNodeValue::FieldDecList(list))
}

fn construct_field_dec_list_more(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    if input.is_empty() {
        Ok(ASTNodeValue::None)
    } else {
        Ok(pop!(input))
    }
}

fn construct_field_dec_type(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    construct_type_name(input)
}

fn construct_identifier_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let id = identifier!(input);
    let mut list = node_default!(input, IdentifierList);
    list.insert(0, id);
    Ok(ASTNodeValue::IdentifierList(list))
}

fn construct_identifier_list_more(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(if input.is_empty() {
        ASTNodeValue::None
    } else {
        pop!(input);
        pop!(input)
    })
}

fn construct_var_dec(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(pop!(input))
}

fn construct_var_declaration(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    pop!(input);
    Ok(pop!(input))
}

fn construct_var_dec_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let type_name = node!(input, TypeName);
    let identifiers = node!(input, IdentifierList);
    pop!(input);
    let mut more = node_default!(input, VarDeclaration);
    more.insert(0, Positional::from_position(type_name.position(), TypedIdentifiers {
        type_name,
        identifiers,
    }));
    Ok(ASTNodeValue::VarDeclaration(more))
}

fn construct_var_dec_list_more(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(pop!(input))
}

fn construct_proc_dec(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let result = node_default!(input, ProcedureDeclaration);
    Ok(ASTNodeValue::ProcedureDeclaration(result))
}

fn construct_proc_declaration(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let start = token!(input);
    let name = node!(input, String);
    pop!(input);
    let params = node_default!(input, ParamList);
    pop!(input, 2);
    let declare = node!(input, DeclarePart);
    let body = node!(input, StatementList);
    let mut list = node_default!(input, ProcedureDeclaration);
    list.insert(0, Positional::from_position(start.position(), ProcedureDeclare {
        name,
        params,
        declare: Box::new(declare),
        body,
    }));
    Ok(ASTNodeValue::ProcedureDeclaration(list))
}

fn construct_proc_name(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(ASTNodeValue::String(identifier!(input).into_inner()))
}

fn construct_param_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(if input.is_empty() {
        ASTNodeValue::None
    } else {
        let param = node!(input, Param);
        let mut list = node_default!(input, ParamList);
        list.insert(0, param);
        ASTNodeValue::ParamList(list)
    })
}

fn construct_param_list_more(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(if input.is_empty() {
        ASTNodeValue::None
    } else {
        pop!(input);
        let param = node!(input, Param);
        let mut list = node_default!(input, ParamList);
        list.insert(0, param);
        ASTNodeValue::ParamList(list)
    })
}

fn construct_param(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let is_var = input.len() == 3;
    let pos = if is_var {
        Some(token!(input).position())
    } else {
        None
    };
    let type_name = node!(input, TypeName);
    let identifiers = node!(input, IdentifierList);
    Ok(ASTNodeValue::Param(Positional::from_position(pos.unwrap_or(type_name.position()), Param {
        is_var,
        definition: TypedIdentifiers {
            type_name,
            identifiers,
        },
    })))
}

fn construct_proc_dec_part(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(pop!(input))
}

fn construct_proc_body(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(pop!(input))
}

fn construct_program_body(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    pop!(input);
    Ok(pop!(input))
}

fn construct_statement_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let stmt = node!(input, Statement);
    let mut list = node_default!(input, StatementList);
    list.insert(0, stmt);
    Ok(ASTNodeValue::StatementList(list))
}

fn construct_more_statement(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    if input.is_empty() {
        Ok(ASTNodeValue::None)
    } else {
        pop!(input);
        let stmt = node!(input, Statement);
        let mut list = node_default!(input, StatementList);
        list.insert(0, stmt);
        Ok(ASTNodeValue::StatementList(list))
    }
}

fn construct_statement(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(match input.pop().unwrap() {
        ASTNodeValue::Statement(statement) => ASTNodeValue::Statement(statement),
        ASTNodeValue::Terminal(token) => {
            match input.pop().unwrap() {
                ASTNodeValue::CallStatementRest(params) => {
                    ASTNodeValue::Statement(Statement::Call(Positional::from_position(token.position(), CallStatement {
                        name: token.image,
                        params,
                    })))
                }
                ASTNodeValue::AssignStatementRest((visit, exp)) => {
                    ASTNodeValue::Statement(Statement::Assign(AssignStatement {
                        variable: VariableRepresent {
                            base: Positional::from_token_image_raw(token),
                            visit,
                        },
                        value: exp,
                    }))
                }
                _ => unreachable!()
            }
        }
        _ => unreachable!()
    })
}

fn construct_ass_call(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(pop!(input))
}

fn construct_assignment_rest(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let visit = node_optional!(input, VariableVisit);
    pop!(input);
    let exp = node!(input, Expression);
    Ok(ASTNodeValue::AssignStatementRest((visit, exp)))
}

fn construct_conditional_statement(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_loop_statement(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_input_statement(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    // read ( identifier )
    pop!(input);
    pop!(input);
    Ok(ASTNodeValue::Statement(Statement::Input(identifier!(input))))
}

fn construct_output_statement(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    // write ( exp )
    pop!(input);
    pop!(input);
    let exp = node!(input, Expression);
    Ok(ASTNodeValue::Statement(Statement::Output(exp)))
}

fn construct_return_statement(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_call_statement_rest(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_call_statement_rest_exp(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_comma_exp(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_rel_exp(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_exp(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let term = node!(input, Term);
    let (op, right) = match node_optional!(input, ExpressionPostFix) {
        Some((op, right)) => (Some(op), Some(right)),
        None => (None, None),
    };
    Ok(ASTNodeValue::Expression(Expression {
        left: Positional::from_position(term.left.position(), term),
        op,
        right,
    }))
}

fn construct_exp_postfix(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(if input.is_empty() {
        ASTNodeValue::None
    } else {
        let op = node!(input, Operator);
        let exp = node!(input, Expression);
        ASTNodeValue::ExpressionPostFix((op, Positional::from_position(exp.left.position(), Box::new(exp))))
    })
}

fn construct_term(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let factor = node!(input, Factor);
    let (op, right) = match node_optional!(input, TermPostFix) {
        Some((op, right)) => (Some(op), Some(right)),
        None => (None, None),
    };
    Ok(ASTNodeValue::Term(ExpressionTerm {
        left: factor,
        op,
        right,
    }))
}

fn construct_term_postfix(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(if input.is_empty() {
        ASTNodeValue::None
    } else {
        let op = node!(input, Operator);
        let term = node!(input, Term);
        ASTNodeValue::TermPostFix((op, Positional::from_position(term.left.position(), Box::new(term))))
    })
}

fn construct_factor(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let (pos, factor) = match input.pop().unwrap() {
        ASTNodeValue::Terminal(token) => {
            let pos = token.position();
            let factor = match token.token_type {
                TokenType::BracketOpen => ExpressionFactor::Bracket(Box::new(node!(input, Expression))),
                TokenType::Int => ExpressionFactor::Constant(u32::from_str(&token.image).unwrap()),
                _ => unreachable!()
            };
            (pos, factor)
        }
        ASTNodeValue::Variable(var) => (var.base.position(), ExpressionFactor::Variable(var)),
        _ => unreachable!()
    };
    Ok(ASTNodeValue::Factor(Positional::from_position(pos, factor)))
}

fn construct_variable(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let id = identifier!(input);
    let visit = node_optional!(input, VariableVisit);
    Ok(ASTNodeValue::Variable(VariableRepresent { base: id, visit }))
}

fn construct_variable_visit(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let dot = node_optional!(input, VariableVisitDot);
    let sqbr = node_optional!(input, VariableVisitSqbr);

    Ok(if let (None, None) = (&dot, &sqbr) {
        ASTNodeValue::None
    } else {
        ASTNodeValue::VariableVisit(VariableVisit { dot, sqbr })
    })
}

fn construct_variable_visit_field(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(if input.is_empty() {
        ASTNodeValue::None
    } else {
        pop!(input);
        ASTNodeValue::VariableVisitDot(identifier!(input))
    })
}

fn construct_variable_visit_index(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(if input.is_empty() {
        ASTNodeValue::None
    } else {
        pop!(input);
        let exp = node!(input, Expression);
        ASTNodeValue::VariableVisitSqbr(Box::new(exp))
    })
}

fn construct_op(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(ASTNodeValue::Operator(token!(input).image))
}
