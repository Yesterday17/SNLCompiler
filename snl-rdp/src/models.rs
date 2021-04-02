use std::collections::HashMap;
use std::rc::Rc;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Program {
    pub(crate) name: String,
    pub(crate) declare: ProgramDeclare,
    pub(crate) body: StatementList,
}

#[derive(Debug, Serialize)]
pub struct ProgramDeclare {
    pub(crate) type_declare: HashMap<String, SNLType>,
    pub(crate) variable_declare: HashMap<String, Rc<SNLType>>,
    pub(crate) procedure_declare: HashMap<String, ProcedureDeclare>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum SNLBaseType {
    Integer,
    Char,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum SNLType {
    Integer,
    Char,
    Array(SNLTypeArray),
    Record(SNLTypeRecord),
    Others(String),
}

#[derive(Debug, Serialize)]
pub struct SNLTypeArray {
    pub(crate) base: SNLBaseType,
    pub(crate) lower_bound: usize,
    pub(crate) upper_bound: usize,
}

pub type SNLTypeRecord = Vec<TypeRecord>;

#[derive(Debug, Serialize)]
pub struct TypeRecord {
    pub(crate) type_name: SNLType,
    pub(crate) identifiers: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ProcedureDeclare {
    pub(crate) params: Vec<Param>,
    pub(crate) declare: Box<ProgramDeclare>,
    pub(crate) body: StatementList,
}

pub type StatementList = Vec<Statement>;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum Statement {
    Conditional(ConditionalStatement),
    Loop(LoopStatement),
    Input(String),
    Output(Expression),
    Return(Expression),
    Assign(AssignStatement),
    Call(CallStatement),
}

#[derive(Debug, Serialize)]
pub struct ConditionalStatement {
    pub(crate) condition: RelationExpression,
    pub(crate) body: StatementList,
    pub(crate) else_body: StatementList,
}

#[derive(Debug, Serialize)]
pub struct LoopStatement {
    pub(crate) condition: RelationExpression,
    pub(crate) body: StatementList,
}

#[derive(Debug, Serialize)]
pub struct AssignStatement {
    pub(crate) variable: VariableRepresent,
    pub(crate) value: Expression,
}

#[derive(Debug, Serialize)]
pub struct CallStatement {
    pub(crate) name: String,
    pub(crate) params: Vec<Expression>,
}

#[derive(Debug, Serialize)]
pub struct ExpressionTemplate<Next> {
    pub(crate) left: Next,
    pub(crate) op: Option<String>,
    pub(crate) right: Option<Box<Self>>,
}

pub type Expression = ExpressionTemplate<ExpressionTerm>;

pub type ExpressionTerm = ExpressionTemplate<ExpressionFactor>;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum ExpressionFactor {
    Bracket(Box<Expression>),
    Constant(u32),
    Variable(VariableRepresent),
}

#[derive(Debug, Serialize)]
pub struct RelationExpression {
    pub(crate) left: Expression,
    pub(crate) op: String,
    pub(crate) right: Expression,
}

#[derive(Debug, Serialize)]
pub struct Param {
    pub(crate) is_var: bool,
    pub(crate) type_name: SNLType,
    pub(crate) identifiers: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct VariableVisit {
    pub(crate) dot: Option<String>,
    pub(crate) sqbr: Option<Box<Expression>>,
}

#[derive(Debug, Serialize)]
pub struct VariableRepresent {
    pub(crate) base: String,
    pub(crate) visit: Option<VariableVisit>,
}