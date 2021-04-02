use std::collections::HashMap;

pub struct ProgramDeclare {
    pub(crate) dec_type: HashMap<String, SNLType>,
    pub(crate) dec_var: HashMap<String, SNLType>,
    pub(crate) dec_proc: HashMap<String, ProcedureDeclare>,
}

impl ProgramDeclare {
    pub fn new() -> Self {
        Self {
            dec_type: Default::default(),
            dec_var: Default::default(),
            dec_proc: Default::default(),
        }
    }
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
    name: String,
    params: HashMap<String, SNLType>,
    declare: Box<ProcedureDeclare>,
    body: ProgramBody,
}

pub struct ProgramBody {
    statements: Vec<Statement>,
}

pub struct Statement {
    //
}