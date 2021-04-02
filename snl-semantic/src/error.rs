use snl_rdp::models::SNLType;

#[derive(Debug)]
pub enum Error {
    /// Identifier with the same name exists at current tier
    DuplicatedIdentifier(String),
    /// Identified use is not found in symbol table
    UndefinedIdentifier(String),
    /// Unknown type used
    UndefinedType(String),
    /// Different type between `expected` and `got`
    ///
    /// `(got, expect)`
    UnexpectedType(String, String),
    /// Constant array visit index is lower than lower bound or higher than high bound
    ///
    /// `(index, low, high)`
    ArrayIndexOutbound(u32, u32, u32),
    /// Different types between asssigner and assignee
    ///
    /// `(got, expected)`
    AssignTypeMismatch(SNLType, SNLType),
    InvalidAssignee,
    /// Procedure call parameter type mismatch
    CallParameterTypeMismatch,
    /// Procedure call parameter count mismatch
    CallParameterCountMismatch,
    InvalidBoolExpression,
}