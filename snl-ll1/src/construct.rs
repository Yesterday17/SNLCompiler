use snl_utils::token::{Token, TokenType};
use snl_utils::ast::*;
use std::collections::HashMap;
use std::str::FromStr;

pub enum ASTNodeValue {
    /// Used when rule A -> '' is used
    None,

    Terminal(Token),

    Program(Program),
    ProgramHead(String),

    DeclarePart(ProgramDeclare),

    TypeDeclaration(PositionalVec<TypeDeclare>),
    TypeId(String),
    TypeName(Positional<SNLType>),
    BaseType(Positional<SNLBaseType>),
    StructureType(Positional<SNLType>),
    ArrayType(Positional<SNLTypeArray>),
    Low(usize),
    Top(usize),
    RecordType(SNLTypeRecord),

    VarDeclaration(PositionalVec<TypedIdentifiers>),
    IdentifierList(PositionalVec<String>),
    ProcedureDeclaration(PositionalVec<ProcedureDeclare>),
    Statement(Statement),
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
        table.0.insert("Low", construct_low);
        table.0.insert("Top", construct_top);
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
    ($v: ident) => {$v.pop().unwrap()};
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

macro_rules! node_optional {
    ($input: ident, $ext: ident) => {
        match pop!($input) {
            ASTNodeValue::$ext(result) => Some(result),
            ASTNodeValue::None         => None,
            _ => unreachable!()
        }
    };
}

#[inline]
fn token(input: &mut Vec<ASTNodeValue>) -> Token {
    match input.pop().unwrap() {
        ASTNodeValue::Terminal(token) => token,
        _ => unreachable!()
    }
}

#[inline]
fn identifier(input: &mut Vec<ASTNodeValue>) -> Positional<String> {
    Positional::from_token_image_raw(token(input))
}

fn construct_program(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_program_head(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    pop!(input);
    Ok(input.pop().unwrap())
}

fn construct_program_name(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let token = token(&mut input);
    Ok(ASTNodeValue::ProgramHead(token.image))
}

fn construct_declare_part(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let type_declare = match input.pop().unwrap() {
        ASTNodeValue::TypeDeclaration(dec) => dec,
        ASTNodeValue::None => Default::default(),
        _ => unreachable!()
    };
    let variable_declare = match input.pop().unwrap() {
        ASTNodeValue::VarDeclaration(dec) => dec,
        ASTNodeValue::None => Default::default(),
        _ => unreachable!()
    };
    let procedure_declare = match input.pop().unwrap() {
        ASTNodeValue::ProcedureDeclaration(dec) => dec,
        ASTNodeValue::None => Default::default(),
        _ => unreachable!()
    };
    Ok(ASTNodeValue::DeclarePart(ProgramDeclare {
        type_declare,
        variable_declare,
        procedure_declare,
    }))
}

fn construct_type_dec(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let result = match input.pop().unwrap() {
        ASTNodeValue::TypeDeclaration(dec) => dec,
        ASTNodeValue::None => Default::default(),
        _ => unreachable!()
    };
    Ok(ASTNodeValue::TypeDeclaration(result))
}

fn construct_type_declaration(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    pop!(input);
    Ok(input.pop().unwrap())
}

fn construct_type_dec_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let name = match input.pop().unwrap() {
        ASTNodeValue::TypeId(id) => id,
        _ => unreachable!()
    };
    pop!(input);
    let base = match input.pop().unwrap() {
        ASTNodeValue::TypeName(name) => name,
        _ => unreachable!()
    };
    pop!(input);

    let mut more = match input.pop().unwrap() {
        ASTNodeValue::TypeDeclaration(list) => list,
        ASTNodeValue::None => Default::default(),
        _ => unreachable!()
    };
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
    Ok(input.pop().unwrap())
}

fn construct_type_id(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let token = token(&mut input);
    Ok(ASTNodeValue::TypeId(token.image))
}

fn construct_type_name(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(ASTNodeValue::TypeName(match input.pop().unwrap() {
        ASTNodeValue::BaseType(ty) => Positional::from_position(ty.position(), ty.into_inner().into()),
        ASTNodeValue::StructureType(ty) => ty,
        ASTNodeValue::Terminal(token) => {
            Positional::from_position(token.position(), SNLType::Others(token.image))
        }
        _ => unreachable!(),
    }))
}

fn construct_base_type(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let token = token(&mut input);
    Ok(ASTNodeValue::BaseType(Positional::from_position(token.position(), match token.token_type {
        TokenType::Integer => SNLBaseType::Integer,
        TokenType::Char => SNLBaseType::Char,
        _ => unreachable!(),
    })))
}

fn construct_structure_type(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_array_type(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_low(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_top(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_record_type(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_field_dec_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_field_dec_list_more(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_field_dec_type(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_identifier_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let mut result: PositionalVec<String> = vec![identifier(&mut input)];
    match input.pop().unwrap() {
        ASTNodeValue::IdentifierList(mut list) => {
            result.append(&mut list);
        }
        ASTNodeValue::None => {}
        _ => unreachable!()
    }
    Ok(ASTNodeValue::IdentifierList(result))
}

fn construct_identifier_list_more(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_var_dec(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(input.pop().unwrap())
}

fn construct_var_declaration(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    pop!(input);
    Ok(input.pop().unwrap())
}

fn construct_var_dec_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let type_name = match input.pop().unwrap() {
        ASTNodeValue::TypeName(n) => n,
        _ => unreachable!()
    };
    let identifiers = match input.pop().unwrap() {
        ASTNodeValue::IdentifierList(l) => l,
        _ => unreachable!()
    };
    pop!(input);
    let mut more = match input.pop().unwrap() {
        ASTNodeValue::VarDeclaration(dec) => dec,
        ASTNodeValue::None => Default::default(),
        _ => unreachable!()
    };
    // FIXME: position
    more.insert(0, Positional::from_position((0, 0), TypedIdentifiers {
        type_name: type_name.into_inner(),
        identifiers,
    }));
    Ok(ASTNodeValue::VarDeclaration(more))
}

fn construct_var_dec_list_more(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(input.pop().unwrap())
}

fn construct_proc_dec(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let result = match input.pop().unwrap() {
        ASTNodeValue::ProcedureDeclaration(dec) => dec,
        ASTNodeValue::None => Default::default(),
        _ => unreachable!()
    };
    Ok(ASTNodeValue::ProcedureDeclaration(result))
}

fn construct_proc_declaration(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_proc_name(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_param_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_param_list_more(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_param(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_proc_dec_part(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_proc_body(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_program_body(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_statement_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_more_statement(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
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
    unimplemented!()
}

fn construct_assignment_rest(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
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
    Ok(ASTNodeValue::Statement(Statement::Input(identifier(&mut input))))
}

fn construct_output_statement(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
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
                TokenType::BracketOpen => {
                    match input.pop().unwrap() {
                        ASTNodeValue::Expression(exp) => ExpressionFactor::Bracket(Box::new(exp)),
                        _ => unreachable!()
                    }
                }
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
    let id = identifier(&mut input);
    let visit = match input.pop().unwrap() {
        ASTNodeValue::VariableVisit(visit) => {
            Some(visit)
        }
        ASTNodeValue::None => None,
        _ => unreachable!()
    };
    Ok(ASTNodeValue::Variable(VariableRepresent { base: id, visit }))
}

fn construct_variable_visit(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    // field
    let dot = match input.pop().unwrap() {
        ASTNodeValue::VariableVisitDot(dot) => Some(dot),
        ASTNodeValue::None => None,
        _ => unreachable!()
    };
    // index
    let sqbr = match input.pop().unwrap() {
        ASTNodeValue::VariableVisitSqbr(sqbr) => Some(sqbr),
        ASTNodeValue::None => None,
        _ => unreachable!()
    };

    Ok(if let (None, None) = (&dot, &sqbr) {
        ASTNodeValue::None
    } else {
        ASTNodeValue::VariableVisit(VariableVisit { dot, sqbr })
    })
}

fn construct_variable_visit_field(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_variable_visit_index(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_op(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(ASTNodeValue::Operator(token(&mut input).image))
}
