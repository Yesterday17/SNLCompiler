#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Identifier with the same name exists at current tier
    #[error("Duplicated identifier '{0}'")]
    DuplicatedIdentifier(String),
    /// Identified use is not found in symbol table
    #[error("Undefined identifier '{0}'")]
    UndefinedIdentifier(String),

    /// Unknown type used
    #[error("Undefined type '{0}'")]
    UndefinedType(String),
    /// Variable type `got` is different from `expected`
    #[error("Type '{got}' is not compatable with type '{expected}'")]
    UncompatableType { expected: String, got: String },

    /// Variable represent only accepts variable symbol
    #[error("Invalid variable represent '{0}'")]
    InvalidVariableRepresent(String),

    /// Invalid array, for example, lower bound is larger than higher bound
    #[error("Invalid array definition")]
    InvalidArrayDefinition,
    /// Constant array visit index is lower than lower bound or higher than high bound
    #[error("Array index({0}) out of bound({1}..{2})")]
    ArrayIndexOutbound(u32, u32, u32),
    /// Unexpected array index
    #[error("Unexpected array index, only array type can be indexed")]
    UnexpectedArrayIndex,

    /// Field is only available in Records
    #[error("Invalid field index type '{0}', only record type can be indexed")]
    InvalidFieldIndexType(String),
    /// Undefined field in records
    #[error("Undefined record field '{0}'")]
    UndefinedRecordField(String),

    /// Different types between asssigner and assignee
    #[error("Assign type '{got}' is not compatable with type '{expected}'")]
    AssignTypeMismatch { expected: String, got: String },
    /// Assignee can not accept any data
    #[error("Invalid assignee")]
    InvalidAssignee,

    /// Procedure call parameter type mismatch
    #[error("Procedure call expected type '{expected}', got '{got}'")]
    CallParameterTypeMismatch { expected: String, got: String },
    /// Procedure call parameter count mismatch
    #[error("Procedure call needs {expected} parameter(s) but got {got}")]
    CallParameterCountMismatch { expected: usize, got: usize },
    /// Expression part in RelationExpression should be integer or char
    #[error("Expressions of relation expression can only be type integer or char")]
    InvalidBoolExpression,

    /// Read only accepts Integer and Char
    #[error("Read identifier type can only be integer or char, got '{0}'")]
    InvalidReadType(String),
    /// Write only accepts Integer and Char
    #[error("Write expression type can only be integer or char, got '{0}'")]
    InvalidWriteType(String),
}