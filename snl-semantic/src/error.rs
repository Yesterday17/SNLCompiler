pub enum Error {
    DuplicatedIdentifier(String),
    UndefinedIdentifier(String),
    /// expected, got
    UnexpectedIdentifierType(String, String),
    ArrayIndexOutbound,
    // TODO: 数组成员变量和域变量的引用不合法
    AssignTypeMismatch,
    AssigneeNotIdentifer,
    ProcedureCallParameterTypeMismatch,
    ProcedureCallParameterCountMismatch,
    ProcedureNotDefined,
    InvalidBoolExpression,
}