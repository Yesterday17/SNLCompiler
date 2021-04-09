use std::collections::HashMap;
use snl_utils::token::TokenType;
use crate::predict::PredictValue::{NonTerminal, Terminal};
use snl_utils::token::TokenType::*;

#[derive(Clone)]
pub enum PredictValue {
    Terminal(TokenType),
    NonTerminal(&'static str),
}

pub struct PredictTable {
    inner: HashMap<PredictKey, Vec<PredictValue>>,
}

#[derive(PartialEq, Eq, Hash)]
struct PredictKey {
    non_terminal: &'static str,
    token_type: TokenType,
}

impl From<(&'static str, TokenType)> for PredictKey {
    fn from((non_terminal, token_type): (&'static str, TokenType)) -> Self {
        Self { non_terminal, token_type }
    }
}

impl Default for PredictTable {
    fn default() -> Self {
        let mut result = Self { inner: Default::default() };
        result.inner.insert(PredictKey::from(("Program", Program)), vec![NonTerminal("ProgramHead"), NonTerminal("DeclarePart"), NonTerminal("ProgramBody")]);
        result.inner.insert(PredictKey::from(("ProgramHead", Program)), vec![Terminal(Program), NonTerminal("ProgramName")]);
        result.inner.insert(PredictKey::from(("ProgramName", Identifer)), vec![Terminal(Identifer)]);
        result.inner.insert(PredictKey::from(("DeclarePart", Type)), vec![NonTerminal("TypeDec"), NonTerminal("VarDec"), NonTerminal("ProcDec")]);
        result.inner.insert(PredictKey::from(("DeclarePart", Var)), vec![NonTerminal("TypeDec"), NonTerminal("VarDec"), NonTerminal("ProcDec")]);
        result.inner.insert(PredictKey::from(("DeclarePart", Procedure)), vec![NonTerminal("TypeDec"), NonTerminal("VarDec"), NonTerminal("ProcDec")]);
        result.inner.insert(PredictKey::from(("DeclarePart", Begin)), vec![NonTerminal("TypeDec"), NonTerminal("VarDec"), NonTerminal("ProcDec")]);
        result.inner.insert(PredictKey::from(("TypeDec", Type)), vec![NonTerminal("TypeDeclaration")]);
        result.inner.insert(PredictKey::from(("TypeDec", Var)), vec![]);
        result.inner.insert(PredictKey::from(("TypeDec", Procedure)), vec![]);
        result.inner.insert(PredictKey::from(("TypeDec", Begin)), vec![]);
        result.inner.insert(PredictKey::from(("TypeDeclaration", Type)), vec![Terminal(Type), NonTerminal("TypeDecList")]);
        result.inner.insert(PredictKey::from(("TypeDecList", Identifer)), vec![NonTerminal("TypeId"), Terminal(Equal), NonTerminal("TypeName"), Terminal(Semicolon), NonTerminal("TypeDecListMore")]);
        result.inner.insert(PredictKey::from(("TypeDecListMore", Identifer)), vec![NonTerminal("TypeDecList")]);
        result.inner.insert(PredictKey::from(("TypeDecListMore", Var)), vec![]);
        result.inner.insert(PredictKey::from(("TypeDecListMore", Procedure)), vec![]);
        result.inner.insert(PredictKey::from(("TypeDecListMore", Begin)), vec![]);
        result.inner.insert(PredictKey::from(("TypeId", Identifer)), vec![Terminal(Identifer)]);
        result.inner.insert(PredictKey::from(("TypeName", Identifer)), vec![Terminal(Identifer)]);
        result.inner.insert(PredictKey::from(("TypeName", Integer)), vec![NonTerminal("BaseType")]);
        result.inner.insert(PredictKey::from(("TypeName", Char)), vec![NonTerminal("BaseType")]);
        result.inner.insert(PredictKey::from(("TypeName", Array)), vec![NonTerminal("StructureType")]);
        result.inner.insert(PredictKey::from(("TypeName", Record)), vec![NonTerminal("StructureType")]);
        result.inner.insert(PredictKey::from(("BaseType", Integer)), vec![Terminal(Integer)]);
        result.inner.insert(PredictKey::from(("BaseType", Char)), vec![Terminal(Char)]);
        result.inner.insert(PredictKey::from(("StructureType", Array)), vec![NonTerminal("ArrayType")]);
        result.inner.insert(PredictKey::from(("StructureType", Record)), vec![NonTerminal("RecordType")]);
        result.inner.insert(PredictKey::from(("ArrayType", Array)), vec![Terminal(Array), Terminal(SquareBracketOpen), NonTerminal("Low"), Terminal(DotDot), NonTerminal("Top"), Terminal(SquareBracketClose), Terminal(Of), NonTerminal("BaseType")]);
        result.inner.insert(PredictKey::from(("Low", Int)), vec![Terminal(Int)]);
        result.inner.insert(PredictKey::from(("Top", Int)), vec![Terminal(Int)]);
        result.inner.insert(PredictKey::from(("RecordType", Record)), vec![Terminal(Record), NonTerminal("FieldDecList"), Terminal(End)]);
        result.inner.insert(PredictKey::from(("FieldDecList", Integer)), vec![NonTerminal("FieldDecType"), NonTerminal("IdentifierList"), Terminal(Identifer), Terminal(Semicolon), NonTerminal("FieldDecListMore")]);
        result.inner.insert(PredictKey::from(("FieldDecList", Char)), vec![NonTerminal("FieldDecType"), NonTerminal("IdentifierList"), Terminal(Identifer), Terminal(Semicolon), NonTerminal("FieldDecListMore")]);
        result.inner.insert(PredictKey::from(("FieldDecList", Array)), vec![NonTerminal("FieldDecType"), NonTerminal("IdentifierList"), Terminal(Identifer), Terminal(Semicolon), NonTerminal("FieldDecListMore")]);
        result.inner.insert(PredictKey::from(("FieldDecListMore", Integer)), vec![NonTerminal("FieldDecList")]);
        result.inner.insert(PredictKey::from(("FieldDecListMore", Char)), vec![NonTerminal("FieldDecList")]);
        result.inner.insert(PredictKey::from(("FieldDecListMore", Array)), vec![NonTerminal("FieldDecList")]);
        result.inner.insert(PredictKey::from(("FieldDecListMore", End)), vec![]);
        result.inner.insert(PredictKey::from(("FieldDecType", Integer)), vec![NonTerminal("BaseType")]);
        result.inner.insert(PredictKey::from(("FieldDecType", Char)), vec![NonTerminal("BaseType")]);
        result.inner.insert(PredictKey::from(("FieldDecType", Array)), vec![NonTerminal("BaseType")]);
        result.inner.insert(PredictKey::from(("IdentifierList", Identifer)), vec![Terminal(Identifer), NonTerminal("IdentifierListMore")]);
        result.inner.insert(PredictKey::from(("IdentifierListMore", BracketClose)), vec![]);
        result.inner.insert(PredictKey::from(("IdentifierListMore", Semicolon)), vec![]);
        result.inner.insert(PredictKey::from(("IdentifierListMore", Comma)), vec![Terminal(Comma), Terminal(Identifer), NonTerminal("IdentifierListMore")]);
        result.inner.insert(PredictKey::from(("VarDec", Var)), vec![NonTerminal("VarDeclaration")]);
        result.inner.insert(PredictKey::from(("VarDec", Procedure)), vec![]);
        result.inner.insert(PredictKey::from(("VarDec", Begin)), vec![]);
        result.inner.insert(PredictKey::from(("VarDeclaration", Var)), vec![Terminal(Var), NonTerminal("VarDecList")]);
        result.inner.insert(PredictKey::from(("VarDecList", Identifer)), vec![NonTerminal("TypeName"), NonTerminal("IdentifierList"), Terminal(Semicolon), NonTerminal("VarDecListMore")]);
        result.inner.insert(PredictKey::from(("VarDecList", Integer)), vec![NonTerminal("TypeName"), NonTerminal("IdentifierList"), Terminal(Semicolon), NonTerminal("VarDecListMore")]);
        result.inner.insert(PredictKey::from(("VarDecList", Char)), vec![NonTerminal("TypeName"), NonTerminal("IdentifierList"), Terminal(Semicolon), NonTerminal("VarDecListMore")]);
        result.inner.insert(PredictKey::from(("VarDecList", Array)), vec![NonTerminal("TypeName"), NonTerminal("IdentifierList"), Terminal(Semicolon), NonTerminal("VarDecListMore")]);
        result.inner.insert(PredictKey::from(("VarDecList", Record)), vec![NonTerminal("TypeName"), NonTerminal("IdentifierList"), Terminal(Semicolon), NonTerminal("VarDecListMore")]);
        result.inner.insert(PredictKey::from(("VarDecListMore", Identifer)), vec![NonTerminal("VarDecList")]);
        result.inner.insert(PredictKey::from(("VarDecListMore", Integer)), vec![NonTerminal("VarDecList")]);
        result.inner.insert(PredictKey::from(("VarDecListMore", Char)), vec![NonTerminal("VarDecList")]);
        result.inner.insert(PredictKey::from(("VarDecListMore", Array)), vec![NonTerminal("VarDecList")]);
        result.inner.insert(PredictKey::from(("VarDecListMore", Record)), vec![NonTerminal("VarDecList")]);
        result.inner.insert(PredictKey::from(("VarDecListMore", Procedure)), vec![]);
        result.inner.insert(PredictKey::from(("VarDecListMore", Begin)), vec![]);
        result.inner.insert(PredictKey::from(("ProcDec", Procedure)), vec![NonTerminal("ProcDeclaration")]);
        result.inner.insert(PredictKey::from(("ProcDec", Begin)), vec![NonTerminal("ProcDeclaration")]);
        result.inner.insert(PredictKey::from(("ProcDeclaration", Procedure)), vec![Terminal(Procedure), NonTerminal("ProcName"), Terminal(BracketOpen), NonTerminal("ParamList"), Terminal(BracketClose), Terminal(Semicolon), NonTerminal("ProcDecPart"), NonTerminal("ProcBody"), NonTerminal("ProcDeclaration")]);
        result.inner.insert(PredictKey::from(("ProcDeclaration", Begin)), vec![]);
        result.inner.insert(PredictKey::from(("ProcName", Identifer)), vec![Terminal(Identifer)]);
        result.inner.insert(PredictKey::from(("ParamList", Identifer)), vec![NonTerminal("Param"), NonTerminal("ParamListMore")]);
        result.inner.insert(PredictKey::from(("ParamList", Integer)), vec![NonTerminal("Param"), NonTerminal("ParamListMore")]);
        result.inner.insert(PredictKey::from(("ParamList", Char)), vec![NonTerminal("Param"), NonTerminal("ParamListMore")]);
        result.inner.insert(PredictKey::from(("ParamList", Array)), vec![NonTerminal("Param"), NonTerminal("ParamListMore")]);
        result.inner.insert(PredictKey::from(("ParamList", Record)), vec![NonTerminal("Param"), NonTerminal("ParamListMore")]);
        result.inner.insert(PredictKey::from(("ParamList", Var)), vec![NonTerminal("Param"), NonTerminal("ParamListMore")]);
        result.inner.insert(PredictKey::from(("ParamListMore", Semicolon)), vec![Terminal(Semicolon), NonTerminal("Param"), NonTerminal("ParamListMore")]);
        result.inner.insert(PredictKey::from(("Param", Identifer)), vec![NonTerminal("TypeName"), NonTerminal("IdentifierList")]);
        result.inner.insert(PredictKey::from(("Param", Integer)), vec![NonTerminal("TypeName"), NonTerminal("IdentifierList")]);
        result.inner.insert(PredictKey::from(("Param", Char)), vec![NonTerminal("TypeName"), NonTerminal("IdentifierList")]);
        result.inner.insert(PredictKey::from(("Param", Array)), vec![NonTerminal("TypeName"), NonTerminal("IdentifierList")]);
        result.inner.insert(PredictKey::from(("Param", Record)), vec![NonTerminal("TypeName"), NonTerminal("IdentifierList")]);
        result.inner.insert(PredictKey::from(("Param", Var)), vec![Terminal(Var), NonTerminal("TypeName"), NonTerminal("IdentifierList")]);
        result.inner.insert(PredictKey::from(("ProcDecPart", Type)), vec![NonTerminal("DeclarePart")]);
        result.inner.insert(PredictKey::from(("ProcDecPart", Var)), vec![NonTerminal("DeclarePart")]);
        result.inner.insert(PredictKey::from(("ProcDecPart", Procedure)), vec![NonTerminal("DeclarePart")]);
        result.inner.insert(PredictKey::from(("ProcDecPart", Begin)), vec![NonTerminal("DeclarePart")]);
        result.inner.insert(PredictKey::from(("ProcBody", Begin)), vec![NonTerminal("ProgramBody")]);
        result.inner.insert(PredictKey::from(("ProgramBody", Begin)), vec![Terminal(Begin), NonTerminal("StatementList"), Terminal(End)]);
        result.inner.insert(PredictKey::from(("StatementList", Identifer)), vec![NonTerminal("Statement"), NonTerminal("MoreStatement")]);
        result.inner.insert(PredictKey::from(("StatementList", If)), vec![NonTerminal("Statement"), NonTerminal("MoreStatement")]);
        result.inner.insert(PredictKey::from(("StatementList", While)), vec![NonTerminal("Statement"), NonTerminal("MoreStatement")]);
        result.inner.insert(PredictKey::from(("StatementList", Read)), vec![NonTerminal("Statement"), NonTerminal("MoreStatement")]);
        result.inner.insert(PredictKey::from(("StatementList", Write)), vec![NonTerminal("Statement"), NonTerminal("MoreStatement")]);
        result.inner.insert(PredictKey::from(("StatementList", Return)), vec![NonTerminal("Statement"), NonTerminal("MoreStatement")]);
        result.inner.insert(PredictKey::from(("MoreStatement", Semicolon)), vec![Terminal(Semicolon), NonTerminal("Statement"), NonTerminal("MoreStatement")]);
        result.inner.insert(PredictKey::from(("MoreStatement", End)), vec![]);
        result.inner.insert(PredictKey::from(("MoreStatement", Else)), vec![]);
        result.inner.insert(PredictKey::from(("MoreStatement", Fi)), vec![]);
        result.inner.insert(PredictKey::from(("MoreStatement", EndWhile)), vec![]);
        result.inner.insert(PredictKey::from(("Statement", Identifer)), vec![Terminal(Identifer), NonTerminal("AssCall")]);
        result.inner.insert(PredictKey::from(("Statement", If)), vec![NonTerminal("ConditionalStatement")]);
        result.inner.insert(PredictKey::from(("Statement", While)), vec![NonTerminal("LoopStatement")]);
        result.inner.insert(PredictKey::from(("Statement", Read)), vec![NonTerminal("InputStatement")]);
        result.inner.insert(PredictKey::from(("Statement", Write)), vec![NonTerminal("OutputStatement")]);
        result.inner.insert(PredictKey::from(("Statement", Return)), vec![NonTerminal("ReturnStatement")]);
        result.inner.insert(PredictKey::from(("AssCall", Semicolon)), vec![NonTerminal("AssignmentRest")]);
        result.inner.insert(PredictKey::from(("AssCall", SquareBracketOpen)), vec![NonTerminal("AssignmentRest")]);
        result.inner.insert(PredictKey::from(("AssCall", End)), vec![NonTerminal("AssignmentRest")]);
        result.inner.insert(PredictKey::from(("AssCall", BracketOpen)), vec![NonTerminal("CallStatementRest")]);
        result.inner.insert(PredictKey::from(("AssCall", Assign)), vec![NonTerminal("AssignmentRest")]);
        result.inner.insert(PredictKey::from(("AssCall", Else)), vec![NonTerminal("AssignmentRest")]);
        result.inner.insert(PredictKey::from(("AssCall", Fi)), vec![NonTerminal("AssignmentRest")]);
        result.inner.insert(PredictKey::from(("AssCall", EndWhile)), vec![NonTerminal("AssignmentRest")]);
        result.inner.insert(PredictKey::from(("AssCall", Dot)), vec![NonTerminal("AssignmentRest")]);
        result.inner.insert(PredictKey::from(("AssignmentRest", Semicolon)), vec![NonTerminal("VariableVisit"), Terminal(Assign), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("AssignmentRest", SquareBracketOpen)), vec![NonTerminal("VariableVisit"), Terminal(Assign), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("AssignmentRest", End)), vec![NonTerminal("VariableVisit"), Terminal(Assign), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("AssignmentRest", Assign)), vec![NonTerminal("VariableVisit"), Terminal(Assign), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("AssignmentRest", Else)), vec![NonTerminal("VariableVisit"), Terminal(Assign), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("AssignmentRest", Fi)), vec![NonTerminal("VariableVisit"), Terminal(Assign), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("AssignmentRest", EndWhile)), vec![NonTerminal("VariableVisit"), Terminal(Assign), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("AssignmentRest", Dot)), vec![NonTerminal("VariableVisit"), Terminal(Assign), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("ConditionalStatement", Program)), vec![Terminal(If), NonTerminal("RelExp"), Terminal(Then), NonTerminal("StatementList"), Terminal(Else), NonTerminal("StatementList"), Terminal(Fi)]);
        result.inner.insert(PredictKey::from(("LoopStatement", While)), vec![Terminal(While), NonTerminal("RelExp"), Terminal(Do), NonTerminal("StatementList"), Terminal(EndWhile)]);
        result.inner.insert(PredictKey::from(("InputStatement", Read)), vec![Terminal(Read), Terminal(BracketOpen), Terminal(Identifer), Terminal(BracketClose)]);
        result.inner.insert(PredictKey::from(("OutputStatement", Write)), vec![Terminal(Write), Terminal(BracketOpen), NonTerminal("Exp"), Terminal(BracketClose)]);
        result.inner.insert(PredictKey::from(("ReturnStatement", Return)), vec![Terminal(Return), Terminal(BracketOpen), NonTerminal("Exp"), Terminal(BracketClose)]);
        result.inner.insert(PredictKey::from(("CallStatementRest", BracketOpen)), vec![Terminal(BracketOpen), NonTerminal("CallStatementRestExp"), Terminal(BracketClose)]);
        result.inner.insert(PredictKey::from(("CallStatementRestExp", Identifer)), vec![NonTerminal("Exp"), NonTerminal("CommaExp")]);
        result.inner.insert(PredictKey::from(("CallStatementRestExp", Int)), vec![NonTerminal("Exp"), NonTerminal("CommaExp")]);
        result.inner.insert(PredictKey::from(("CallStatementRestExp", BracketOpen)), vec![NonTerminal("Exp"), NonTerminal("CommaExp")]);
        result.inner.insert(PredictKey::from(("CallStatementRestExp", BracketClose)), vec![]);
        result.inner.insert(PredictKey::from(("CommaExp", Comma)), vec![Terminal(Comma), NonTerminal("Exp"), NonTerminal("CommaExp")]);
        result.inner.insert(PredictKey::from(("CommaExp", BracketClose)), vec![]);
        result.inner.insert(PredictKey::from(("RelExp", Identifer)), vec![NonTerminal("Exp"), NonTerminal("CmdOp"), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("RelExp", Int)), vec![NonTerminal("Exp"), NonTerminal("CmdOp"), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("RelExp", BracketOpen)), vec![NonTerminal("Exp"), NonTerminal("CmdOp"), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("Exp", Identifer)), vec![NonTerminal("Term"), NonTerminal("ExpPostFix")]);
        result.inner.insert(PredictKey::from(("Exp", Int)), vec![NonTerminal("Term"), NonTerminal("ExpPostFix")]);
        result.inner.insert(PredictKey::from(("Exp", BracketOpen)), vec![NonTerminal("Term"), NonTerminal("ExpPostFix")]);
        result.inner.insert(PredictKey::from(("ExpPostFix", Equal)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", Semicolon)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", SquareBracketClose)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", End)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", Comma)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", BracketClose)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", Then)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", Else)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", Fi)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", Do)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", EndWhile)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", LessThan)), vec![]);
        result.inner.insert(PredictKey::from(("ExpPostFix", Add)), vec![NonTerminal("AddOp"), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("ExpPostFix", Minus)), vec![NonTerminal("AddOp"), NonTerminal("Exp")]);
        result.inner.insert(PredictKey::from(("Term", Identifer)), vec![NonTerminal("Factor"), NonTerminal("TermPostFix")]);
        result.inner.insert(PredictKey::from(("Term", Int)), vec![NonTerminal("Factor"), NonTerminal("TermPostFix")]);
        result.inner.insert(PredictKey::from(("Term", BracketOpen)), vec![NonTerminal("Factor"), NonTerminal("TermPostFix")]);
        result.inner.insert(PredictKey::from(("TermPostFix", Equal)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", Semicolon)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", SquareBracketClose)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", End)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", Comma)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", BracketClose)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", Then)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", Else)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", Fi)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", Do)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", EndWhile)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", LessThan)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", Add)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", Minus)), vec![]);
        result.inner.insert(PredictKey::from(("TermPostFix", Multiply)), vec![NonTerminal("MultOp"), NonTerminal("Term")]);
        result.inner.insert(PredictKey::from(("TermPostFix", Divide)), vec![NonTerminal("MultOp"), NonTerminal("Term")]);
        result.inner.insert(PredictKey::from(("Factor", Identifer)), vec![NonTerminal("Variable")]);
        result.inner.insert(PredictKey::from(("Factor", Int)), vec![Terminal(Int)]);
        result.inner.insert(PredictKey::from(("Factor", BracketOpen)), vec![Terminal(BracketOpen), NonTerminal("Exp"), Terminal(BracketClose)]);
        result.inner.insert(PredictKey::from(("Variable", Identifer)), vec![Terminal(Identifer), NonTerminal("VariableVisit")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Equal)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Semicolon)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", SquareBracketOpen)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", SquareBracketClose)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", End)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Comma)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", BracketClose)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Assign)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Then)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Else)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Fi)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Do)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", EndWhile)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Dot)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", LessThan)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Add)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Minus)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Multiply)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisit", Divide)), vec![NonTerminal("VariableVisitField"), NonTerminal("VariableVisitIndex")]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Equal)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Semicolon)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", SquareBracketOpen)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", SquareBracketClose)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", End)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Comma)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", BracketClose)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Assign)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Then)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Else)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Fi)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Do)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", EndWhile)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Dot)), vec![Terminal(Dot), Terminal(Identifer)]);
        result.inner.insert(PredictKey::from(("VariableVisitField", LessThan)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Add)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Minus)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Multiply)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitField", Divide)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Equal)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Semicolon)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", SquareBracketOpen)), vec![Terminal(SquareBracketOpen), NonTerminal("Exp"), Terminal(SquareBracketClose)]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", SquareBracketClose)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", End)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Comma)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", BracketClose)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Assign)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Then)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Else)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Fi)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Do)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", EndWhile)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Dot)), vec![Terminal(Dot), Terminal(Identifer)]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", LessThan)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Add)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Minus)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Multiply)), vec![]);
        result.inner.insert(PredictKey::from(("VariableVisitIndex", Divide)), vec![]);
        result.inner.insert(PredictKey::from(("CmdOp", Equal)), vec![Terminal(Equal)]);
        result.inner.insert(PredictKey::from(("CmdOp", LessThan)), vec![Terminal(LessThan)]);
        result.inner.insert(PredictKey::from(("AddOp", Add)), vec![Terminal(Add)]);
        result.inner.insert(PredictKey::from(("AddOp", Minus)), vec![Terminal(Minus)]);
        result.inner.insert(PredictKey::from(("MultOp", Multiply)), vec![Terminal(Multiply)]);
        result.inner.insert(PredictKey::from(("MultOp", Divide)), vec![Terminal(Divide)]);
        result
    }
}

impl PredictTable {
    pub fn lookup(&self, non_terminal: &'static str, token_type: TokenType) -> Option<&Vec<PredictValue>> {
        self.inner.get(&PredictKey { non_terminal, token_type })
    }
}