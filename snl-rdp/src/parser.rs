use snl_lexer::token::{Token, TokenType};
use snl_utils::Tokens;
use crate::models::*;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Parser {
    inner: Tokens,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            inner: Tokens::new(tokens),
        }
    }

    pub fn parse(&self) -> Result<(), String> {
        self.parse_program()
    }

    fn parse_program(&self) -> Result<(), String> {
        let name = self.parse_program_head()?;
        let declare = self.parse_declare_part()?;
        self.parse_program_body()?;
        Ok(())
    }

    fn parse_program_head(&self) -> Result<String, String> {
        self.inner.take(TokenType::Program)?;
        let program_name = self.inner.take(TokenType::Identifer)?.image.clone();
        Ok(program_name)
    }

    fn parse_declare_part(&self) -> Result<ProgramDeclare, String> {
        let dec_type = if TokenType::Type == self.inner.current() {
            self.parse_declare_type()?
        } else {
            Default::default()
        };

        let dec_var = if TokenType::Var == self.inner.current() {
            self.parse_declare_var()?
        } else {
            Default::default()
        };

        let dec_proc = if TokenType::Procedure == self.inner.current() {
            self.parse_declare_proc()?
        } else {
            Default::default()
        };

        Ok(ProgramDeclare {
            dec_type,
            dec_var,
            dec_proc,
        })
    }

    fn parse_declare_type(&self) -> Result<HashMap<String, SNLType>, String> {
        let mut declare = HashMap::new();
        self.inner.take(TokenType::Type)?;
        loop {
            let name = self.inner.take(TokenType::Identifer)?;
            self.inner.take(TokenType::Equal)?;
            let inner_type = self.parse_type_name()?;
            self.inner.take(TokenType::Semicolon)?;
            declare.insert(name.image.clone(), inner_type);
            if TokenType::Identifer != self.inner.current() {
                break;
            }
        }

        Ok(declare)
    }

    fn parse_declare_var(&self) -> Result<HashMap<String, SNLType>, String> {
        let mut result = HashMap::new();
        self.inner.take(TokenType::Var)?;
        loop {
            let type_name = self.parse_type_name()?;
            let ids = self.parse_identifier_list()?;
            self.inner.take(TokenType::Semicolon)?;
            for id in ids {
                result.insert(id, type_name.clone());
            }

            if self.inner.current() == TokenType::Procedure || self.inner.current() == TokenType::Begin {
                break;
            }
        }
        Ok(result)
    }

    fn parse_declare_proc(&self) -> Result<HashMap<String, ProcedureDeclare>, String> {
        // TODO
        Ok(HashMap::new())
    }

    fn parse_type_name(&self) -> Result<SNLType, String> {
        let next = self.inner.current();
        match next {
            TokenType::Integer => {
                self.inner.move_next();
                Ok(SNLType::Integer)
            }
            TokenType::Char => {
                self.inner.move_next();
                Ok(SNLType::Char)
            }
            TokenType::Array => {
                Ok(SNLType::Array(self.parse_array_type()?))
            }
            TokenType::Record => {
                unimplemented!();
                // TODO
                Ok(SNLType::Record())
            }
            TokenType::Identifer => {
                let name = self.inner.current_token().image.clone();
                self.inner.move_next();
                Ok(SNLType::Others(name))
            }
            _ => Err(format!("unexpected token {:?}", next))
        }
    }

    fn parse_array_type(&self) -> Result<SNLTypeArray, String> {
        self.inner.take(TokenType::Array)?;
        self.inner.take(TokenType::SquareBracketOpen)?;
        let low = self.inner.take(TokenType::Int)?.image.as_str();
        self.inner.take(TokenType::DotDot)?;
        let top = self.inner.take(TokenType::Int)?.image.as_str();
        self.inner.take(TokenType::SquareBracketClose)?;
        self.inner.take(TokenType::Of)?;
        let base = self.inner.look_after();
        let base = match base {
            Some(TokenType::Integer) => SNLBaseType::Integer,
            Some(TokenType::Char) => SNLBaseType::Char,
            _ => return Err(format!("unexpected base type {:?}", base))
        };
        Ok(SNLTypeArray(base, usize::from_str(low).unwrap(), usize::from_str(top).unwrap()))
    }

    fn parse_identifier_list(&self) -> Result<Vec<String>, String> {
        let mut ids = Vec::new();
        let mut need_comma = false;
        loop {
            match self.inner.current() {
                TokenType::Identifer => {
                    if need_comma {
                        break;
                    } else {
                        ids.push(self.inner.current_token().image.clone());
                        need_comma = true;
                        self.inner.move_next();
                    }
                }
                TokenType::Comma => {
                    if !need_comma {
                        return Err(format!("unexpected ','"));
                    } else {
                        need_comma = false;
                        self.inner.move_next();
                    }
                }
                _ => break,
            }
        }
        Ok(ids)
    }

    fn parse_program_body(&self) -> Result<(), String> {
        // TODO
        Ok(())
    }
}