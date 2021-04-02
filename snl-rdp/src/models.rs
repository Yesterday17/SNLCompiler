use std::collections::HashMap;

pub struct Program {
    pub(crate) name: String,
    pub(crate) declare: ProgramDeclare,
    pub(crate) body: StatementList,
}

pub struct ProgramDeclare {
    pub(crate) type_declare: HashMap<String, SNLType>,
    pub(crate) variable_declare: HashMap<String, SNLType>,
    pub(crate) procedure_declare: HashMap<String, ProcedureDeclare>,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum SNLBaseType {
    Integer,
    Char,
}

#[derive(Clone)]
pub enum SNLType {
    Integer,
    Char,
    /// base, low, top
    Array(SNLTypeArray),
    /// TODO
    Record(),
    Others(String),
}

#[derive(Clone)]
pub struct SNLTypeArray(pub(crate) SNLBaseType, pub(crate) usize, pub(crate) usize);

pub struct ProcedureDeclare {
    pub(crate) params: Vec<Param>,
    pub(crate) declare: Box<ProgramDeclare>,
    pub(crate) body: StatementList,
}

pub type StatementList = Vec<Statement>;

pub enum Statement {
    Conditional(ConditionalStatement),
    Loop(),
    Input(String),
    Output(Expression),
    Return(Expression),
    Assign(AssignStatement),
    Call(),
}

pub struct ConditionalStatement {
    pub(crate) condition: (),
    pub(crate) body: StatementList,
    pub(crate) else_body: StatementList,
}

pub struct AssignStatement {
    pub(crate) name: String,
    pub(crate) value: Expression,
}

pub struct ExpressionTemplate<Next> {
    pub(crate) left: Next,
    pub(crate) op: Option<String>,
    pub(crate) right: Option<Box<Self>>,
}

pub type Expression = ExpressionTemplate<ExpressionTerm>;

pub type ExpressionTerm = ExpressionTemplate<ExpressionFactor>;

pub enum ExpressionFactor {
    Bracket(Box<Expression>),
    Constant(u32),
    Variable(String),
}

pub struct Param {
    pub(crate) is_var: bool,
    pub(crate) type_name: SNLType,
    pub(crate) identifiers: Vec<String>,
}