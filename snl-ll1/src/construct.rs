use snl_utils::token::Token;
use snl_utils::ast::*;
use std::collections::HashMap;

pub enum ASTNodeValue {
    /// Used when rule A -> '' is used
    None,

    Terminal(Token),

    Program(Program),
}

pub struct ConstructTable(HashMap<&'static str, fn(&[ASTNodeValue]) -> Result<ASTNodeValue, String>>);

impl Default for ConstructTable {
    fn default() -> Self {
        let mut table = ConstructTable(Default::default());
        table.0.insert("Program", construct_program);
        table
    }
}

impl ConstructTable {
    pub fn construct(&self, ty: &'static str, input: &[ASTNodeValue]) -> Result<ASTNodeValue, String> {
        self.0[ty](input)
    }
}

fn construct_program(input: &[ASTNodeValue]) -> Result<ASTNodeValue, String> {
    Err(format!(""))
}