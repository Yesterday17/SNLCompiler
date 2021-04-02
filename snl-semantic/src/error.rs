#[derive(Debug)]
pub enum Error {
    /// Identifier with the same name exists at current tier
    DuplicatedIdentifier(String),
    /// Identified use is not found in symbol table
    UndefinedIdentifier(String),

    /// Unknown type used
    UndefinedType(String),
    /// Variable type `got` is different from `expected`
    ///
    /// `(got, expect)`
    UncompatableType(String, String),

    /// Invalid array, for example, lower bound is larger than higher bound
    InvalidArrayDefinition,
    /// Constant array visit index is lower than lower bound or higher than high bound
    ///
    /// `(index, low, high)`
    ArrayIndexOutbound(u32, u32, u32),
    /// Unexpected array index
    UnexpectedArrayIndex,

    /// Field is only available in Records
    UnexpectedField,
    /// Undefined field in records
    UndefinedRecordField(String),

    /// Different types between asssigner and assignee
    ///
    /// `(got, expected)`
    AssignTypeMismatch(String, String),
    /// Assignee can not accept any data
    InvalidAssignee,

    /// Procedure call parameter type mismatch
    CallParameterTypeMismatch,
    /// Procedure call parameter count mismatch
    CallParameterCountMismatch,
    InvalidBoolExpression,

    /// Read only accepts Integer and Char
    InvalidReadType(String),
}