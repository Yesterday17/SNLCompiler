use serde::Serialize;
use std::ops::Deref;
use crate::token::Token;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

#[derive(Debug, Serialize)]
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

impl Positional<String> {
    pub fn from_token_image(token: &Token) -> Self {
        Positional::new(token.line, token.column, token.image.clone())
    }

    pub fn from_token_image_raw(token: Token) -> Self {
        Positional::new(token.line, token.column, token.image)
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
    pub base: Positional<SNLType>,
    pub name: String,
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
    pub name: String,
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

impl FromStr for SNLBaseType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "integer" => Self::Integer,
            "char" => Self::Char,
            _ => unreachable!()
        })
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

impl From<SNLBaseType> for SNLType {
    fn from(ty: SNLBaseType) -> Self {
        match ty {
            SNLBaseType::Integer => SNLType::Integer,
            SNLBaseType::Char => SNLType::Char,
        }
    }
}

/// Begin with:
///  letter: integer or char
/// `[`: array
/// `{`: record
/// `#`: custom
impl SNLType {
    pub fn to_string<F>(&self, query: F) -> String
        where F: Fn(&str) -> Option<String> {
        let (result, _) = self.to_string_inner(query);
        result
    }

    fn to_string_inner<F>(&self, mut query: F) -> (String, F)
        where F: Fn(&str) -> Option<String>
    {
        let str = match self {
            SNLType::Integer => "integer".to_string(),
            SNLType::Char => "char".to_string(),
            SNLType::Array(array) => array.to_string(),
            SNLType::Record(record) => {
                let mut fields: BTreeMap<String, BTreeSet<&str>> = Default::default();
                for r in record.iter() {
                    let ty = r.type_name.to_string_inner(query);
                    query = ty.1;
                    let ty = ty.0;
                    if !fields.contains_key(&ty) {
                        fields.insert(ty, r.identifiers.iter().map(|r| r.inner.as_str()).collect());
                    } else {
                        for f in r.identifiers.iter() {
                            fields.get_mut(&ty).unwrap().insert(f.inner());
                        }
                    }
                }

                let mut result = "{".to_owned();
                let mut is_first = true;
                for (type_name, variables) in fields {
                    if !is_first {
                        result += ";";
                    } else {
                        is_first = false;
                    }

                    let mut is_first = true;
                    for var in variables {
                        if !is_first {
                            result += ",";
                        } else {
                            is_first = false;
                        }
                        result += var;
                    }
                    result += ":";
                    result += &type_name;
                }
                result += "}";
                result
            }
            SNLType::Others(others) => {
                match query(others) {
                    Some(got) => got,
                    None => format!("#{}", others),
                }
            }
        };
        (str, query)
    }
}

impl FromStr for SNLType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "integer" {
            Ok(SNLType::Integer)
        } else if s == "char" {
            Ok(SNLType::Char)
        } else if s.starts_with("[") {
            let balance = if s.ends_with("]") {
                s.len()
            } else {
                let mut balance = 0;
                for (offset, ch) in s.char_indices() {
                    if offset != 0 && balance == 0 {
                        balance = offset;
                        break;
                    }

                    if ch == '[' {
                        balance += 1;
                    } else if ch == ']' {
                        balance -= 1;
                    }
                }
                balance
            };
            let s = &s[1..(balance - 1)];
            // low..top
            let splits: Vec<_> = s.split(';').collect();
            let bounds: Vec<_> = splits[0].split("..").collect();
            Ok(SNLType::Array(SNLTypeArray {
                base: SNLBaseType::from_str(splits[1])?,
                lower_bound: usize::from_str(bounds[0]).unwrap(),
                upper_bound: usize::from_str(bounds[1]).unwrap(),
            }))
        } else if s.starts_with("{") {
            unimplemented!()
        } else if s.starts_with("#") {
            let (_, s) = s.split_at(1);
            Ok(SNLType::Others(s.to_owned()))
        } else {
            unreachable!()
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
    pub type_name: Positional<SNLType>,
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
    pub name: String,
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
    pub dot: Option<Positional<String>>,
    pub sqbr: Option<Box<Expression>>,
}

#[derive(Debug, Serialize)]
pub struct VariableRepresent {
    pub base: Positional<String>,
    pub visit: Option<VariableVisit>,
}

#[cfg(test)]
mod tests {
    use crate::ast::{SNLType, SNLTypeArray, SNLBaseType, TypedIdentifiers, Positional};
    use std::str::FromStr;

    #[test]
    fn test_type_to_signature() {
        assert_eq!(SNLType::Integer.to_string(|r| Some(r.to_string())), "integer");
        assert_eq!(SNLType::Char.to_string(|r| Some(r.to_string())), "char");
        assert_eq!(SNLType::Others("others".to_owned()).to_string(|r| Some(r.to_string())), "#others");
        assert_eq!(SNLType::Array(SNLTypeArray {
            base: SNLBaseType::Integer,
            lower_bound: 0,
            upper_bound: 10,
        }).to_string(|r| Some(r.to_string())), "[0..10;integer]");
        assert_eq!(SNLType::Record(vec![
            TypedIdentifiers {
                type_name: SNLType::Integer,
                identifiers: vec![Positional::dump("a".to_owned()), Positional::dump("c".to_owned())],
            },
            TypedIdentifiers { type_name: SNLType::Integer, identifiers: vec![Positional::dump("b".to_owned())] },
        ]).to_string(|r| Some(r.to_string())), "{a,b,c:integer}");
    }

    #[test]
    fn test_type_from_signature() {
        assert_eq!(SNLType::Integer, SNLType::from_str("integer").unwrap());
        assert_eq!(SNLType::Char, SNLType::from_str("char").unwrap());
        assert_eq!(SNLType::Others("test".to_owned()), SNLType::from_str("#test").unwrap());
        assert_eq!(SNLType::Array(SNLTypeArray {
            base: SNLBaseType::Integer,
            lower_bound: 0,
            upper_bound: 10,
        }), SNLType::from_str("[0..10;integer]").unwrap());
    }
}