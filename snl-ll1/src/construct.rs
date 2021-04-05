use crate::predict::PredictValue;
use snl_utils::token::Token;

pub enum ASTNodeValue {
    Terminal(Token),
    NonTerminal()
}

pub trait ASTNodeConstruct {
    type Err;

    fn construct(input: Vec<PredictValue>) -> Result<Self, Self::Err>;
}