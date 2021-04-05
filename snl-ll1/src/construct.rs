use snl_utils::token::{Token, TokenType};
use snl_utils::ast::*;
use std::collections::HashMap;

pub enum ASTNodeValue {
    /// Used when rule A -> '' is used
    None,

    Terminal(Token),

    Program(Program),
    ProgramName(String),

    DeclarePart(ProgramDeclare),

    TypeDeclaration(TypeDeclare),
    TypeDecList(),
    TypeId(String),
    TypeName(SNLType),
    BaseType(SNLBaseType),
    StructureType(SNLType),
    ArrayType(SNLTypeArray),
    Low(usize),
    Top(usize),
    RecordType(SNLTypeRecord),

    IdentifierList(PositionalVec<String>),
    ProcedureDeclaration(PositionalVec<ProcedureDeclare>),
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
        table.0.insert("CmdOp", construct_cmd_op);
        table.0.insert("AddOp", construct_add_op);
        table.0.insert("MultOp", construct_mult_op);

        table
    }
}

impl ConstructTable {
    pub fn construct(&self, ty: &'static str, mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
        self.0[ty](input)
    }
}

fn construct_program(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_program_head(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(match input.pop().unwrap() {
        ASTNodeValue::ProgramName(name) => ASTNodeValue::ProgramName(name),
        _ => unreachable!(),
    })
}

fn construct_program_name(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let token = match input.pop().unwrap() {
        ASTNodeValue::Terminal(token) => token,
        _ => unreachable!(),
    };
    Ok(ASTNodeValue::ProgramName(token.image))
}

fn construct_declare_part(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_type_dec(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_type_declaration(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_type_dec_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_type_dec_list_more(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_type_id(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let token = match input.pop().unwrap() {
        ASTNodeValue::Terminal(token) => token,
        _ => unreachable!(),
    };
    Ok(ASTNodeValue::TypeId(token.image))
}

fn construct_type_name(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    Ok(ASTNodeValue::TypeName(match input.pop().unwrap() {
        ASTNodeValue::BaseType(ty) => ty.into(),
        ASTNodeValue::StructureType(ty) => ty,
        ASTNodeValue::Terminal(token) => {
            SNLType::Others(token.image)
        }
        _ => unreachable!(),
    }))
}

fn construct_base_type(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let token = match input.pop().unwrap() {
        ASTNodeValue::Terminal(token) => token,
        _ => unreachable!(),
    };
    Ok(ASTNodeValue::BaseType(match token.token_type {
        TokenType::Integer => SNLBaseType::Integer,
        TokenType::Char => SNLBaseType::Char,
        _ => unreachable!(),
    }))
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
    let mut result: PositionalVec<String> = Default::default();
    match input.pop().unwrap() {
        ASTNodeValue::Terminal(token) => result.push(Positional::from_position(token.position(), token.image)),
        _ => unreachable!(),
    };
    match input.pop().unwrap() {
        ASTNodeValue::IdentifierList(list) => {
            for value in list {
                result.push(value.clone());
            }
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
    unimplemented!()
}

fn construct_var_declaration(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_var_dec_list(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_var_dec_list_more(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_proc_dec(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    let result = match input.pop().unwrap() {
        ASTNodeValue::ProcedureDeclaration(dec) => dec,
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
    unimplemented!()
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
    unimplemented!()
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
    unimplemented!()
}

fn construct_exp_postfix(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_term(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_term_postfix(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_factor(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_variable(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_variable_visit(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_variable_visit_field(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_variable_visit_index(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_cmd_op(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_add_op(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}

fn construct_mult_op(mut input: Vec<ASTNodeValue>) -> Result<ASTNodeValue, String> {
    unimplemented!()
}
