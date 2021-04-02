use serde::Serialize;
use std::ops::Deref;
use snl_lexer::token::Token;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Serialize, Clone)]
pub struct Positional<T> {
    pub line: u32,
    pub column: u32,
    inner: T,
}

impl<T> Positional<T> {
    pub fn new(line: u32, column: u32, inner: T) -> Self {
        Self {
            line,
            column,
            inner,
        }
    }

    pub fn from_position((line, column): (u32, u32), inner: T) -> Self {
        Positional::new(line, column, inner)
    }

    pub fn from_token(token: &Token, inner: T) -> Self {
        Positional::new(token.line, token.column, inner)
    }

    pub fn dump(inner: T) -> Self {
        Self::new(0, 0, inner)
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn position(&self) -> (u32, u32) {
        (self.line, self.column)
    }
}

impl<T> Deref for Positional<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub type PositionalVec<T> = Vec<Positional<T>>;

#[derive(Debug, Serialize)]
pub struct Program {
    pub name: String,
    pub declare: ProgramDeclare,
    pub body: StatementList,
}

#[derive(Debug, Serialize)]
pub struct ProgramDeclare {
    pub type_declare: PositionalVec<TypeDeclare>,
    pub variable_declare: PositionalVec<TypedIdentifiers>,
    pub procedure_declare: PositionalVec<ProcedureDeclare>,
}

#[derive(Debug, Serialize)]
pub struct TypeDeclare {
    pub(crate) base: Positional<SNLType>,
    pub(crate) name: String,
}

impl TypeDeclare {
    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    pub fn base(&self) -> Positional<&SNLType> {
        Positional::from_position(self.base.position(), &self.base.inner)
    }
}

#[derive(Debug, Serialize)]
pub struct ProcedureDeclare {
    pub(crate) name: String,
    pub params: PositionalVec<Param>,
    pub declare: Box<ProgramDeclare>,
    pub body: StatementList,
}

impl ProcedureDeclare {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum SNLBaseType {
    Integer,
    Char,
}

impl ToString for SNLBaseType {
    fn to_string(&self) -> String {
        match self {
            SNLBaseType::Integer => "integer".to_string(),
            SNLBaseType::Char => "char".to_string(),
        }
    }
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

/// Begin with:
/// `|`: integer or char
/// `[`: array
/// `{`: record
/// `#`: custom
impl ToString for SNLType {
    fn to_string(&self) -> String {
        match self {
            SNLType::Integer => "|integer".to_string(),
            SNLType::Char => "|char".to_string(),
            SNLType::Array(array) => array.to_string(),
            SNLType::Record(record) => {
                // a,b,:integer;c,d,:char
                let mut fields: BTreeMap<String, BTreeSet<&str>> = Default::default();
                for r in record.iter() {
                    let ty = r.type_name.to_string();
                    if !fields.contains_key(&ty) {
                        fields.insert(ty, r.identifiers.iter().map(|r| r.inner.as_str()).collect());
                    } else {
                        for f in r.identifiers.iter() {
                            fields.get_mut(&ty).unwrap().insert(f.inner());
                        }
                    }
                }

                let mut result = "{".to_owned();
                for (type_name, variables) in fields {
                    for var in variables {
                        result += var;
                        result += ",";
                    }
                    result += ":";
                    result += &type_name;
                    result += ";";
                }
                result += "}";
                result
            }
            SNLType::Others(others) => format!("#{}", others)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SNLTypeArray {
    pub base: SNLBaseType,
    pub lower_bound: usize,
    pub upper_bound: usize,
}

impl ToString for SNLTypeArray {
    fn to_string(&self) -> String {
        format!("[{}..{};{}]", self.lower_bound, self.upper_bound, self.base.to_string())
    }
}

pub type SNLTypeRecord = Vec<TypedIdentifiers>;

#[derive(Debug, Serialize)]
pub struct TypedIdentifiers {
    pub type_name: SNLType,
    pub identifiers: PositionalVec<String>,
}

pub type StatementList = Vec<Statement>;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum Statement {
    Conditional(ConditionalStatement),
    Loop(LoopStatement),
    Input(Positional<String>),
    Output(Expression),
    Return(Expression),
    Assign(AssignStatement),
    Call(Positional<CallStatement>),
}

#[derive(Debug, Serialize)]
pub struct ConditionalStatement {
    pub condition: RelationExpression,
    pub body: StatementList,
    pub else_body: StatementList,
}

#[derive(Debug, Serialize)]
pub struct LoopStatement {
    pub condition: RelationExpression,
    pub body: StatementList,
}

#[derive(Debug, Serialize)]
pub struct AssignStatement {
    pub variable: VariableRepresent,
    pub value: Expression,
}

#[derive(Debug, Serialize)]
pub struct CallStatement {
    pub(crate) name: String,
    pub params: Vec<Expression>,
}

impl CallStatement {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct ExpressionTemplate<Next> {
    pub left: Positional<Next>,
    pub op: Option<String>,
    pub right: Option<Positional<Box<Self>>>,
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
    pub left: Expression,
    pub op: String,
    pub right: Expression,
}

#[derive(Debug, Serialize)]
pub struct Param {
    pub is_var: bool,
    pub definition: TypedIdentifiers,
}

#[derive(Debug, Serialize)]
pub struct VariableVisit {
    pub dot: Option<String>,
    pub sqbr: Option<Box<Expression>>,
}

#[derive(Debug, Serialize)]
pub struct VariableRepresent {
    pub base: String,
    pub visit: Option<VariableVisit>,
}

#[cfg(test)]
mod tests {
    use crate::models::{SNLType, SNLTypeArray, SNLBaseType, TypedIdentifiers, Positional};

    #[test]
    fn test_type_signature() {
        assert_eq!(SNLType::Integer.to_string(), "|integer");
        assert_eq!(SNLType::Char.to_string(), "|char");
        assert_eq!(SNLType::Array(SNLTypeArray {
            base: SNLBaseType::Integer,
            lower_bound: 0,
            upper_bound: 10,
        }).to_string(), "[0..10;integer]");
        assert_eq!(SNLType::Record(vec![
            TypedIdentifiers {
                type_name: SNLType::Integer,
                identifiers: vec![Positional::dump("a".to_owned()), Positional::dump("c".to_owned())],
            },
            TypedIdentifiers { type_name: SNLType::Integer, identifiers: vec![Positional::dump("b".to_owned())] },
        ]).to_string(), "{a,b,c,:|integer;}");
        assert_eq!(SNLType::Others("others".to_owned()).to_string(), "#others");
    }
}