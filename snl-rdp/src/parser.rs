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

    pub fn parse(&self) -> Result<Program, String> {
        self.parse_program()
    }

    fn parse_program(&self) -> Result<Program, String> {
        let name = self.parse_program_head()?;
        let declare = self.parse_declare_part()?;
        let body = self.parse_program_body()?;
        Ok(Program {
            name,
            declare,
            body,
        })
    }

    fn parse_program_head(&self) -> Result<String, String> {
        self.inner.take(TokenType::Program)?;
        let program_name = self.inner.take(TokenType::Identifer)?.image.clone();
        Ok(program_name)
    }

    fn parse_declare_part(&self) -> Result<ProgramDeclare, String> {
        let type_declare = if TokenType::Type == self.inner.current() {
            self.parse_declare_type()?
        } else {
            Default::default()
        };

        let variable_declare = if TokenType::Var == self.inner.current() {
            self.parse_declare_var()?
        } else {
            Default::default()
        };

        let procedure_declare = if TokenType::Procedure == self.inner.current() {
            self.parse_declare_proc()?
        } else {
            Default::default()
        };

        Ok(ProgramDeclare {
            type_declare,
            variable_declare,
            procedure_declare,
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
                // TODO
                unimplemented!();
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

    fn parse_program_body(&self) -> Result<StatementList, String> {
        self.inner.take(TokenType::Begin)?;
        let body = self.parse_statement_list()?;
        self.inner.take(TokenType::End)?;
        Ok(body)
    }

    fn parse_statement_list(&self) -> Result<StatementList, String> {
        let mut statements = Vec::new();
        loop {
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
                self.inner.take(TokenType::Semicolon)?;
            } else {
                break;
            }
        }
        Ok(statements)
    }

    fn parse_statement(&self) -> Result<Option<Statement>, String> {
        Ok(match self.inner.current() {
            TokenType::If => Some(self.parse_conditional_statement()?),
            TokenType::While => Some(self.parse_loop_statement()?),
            TokenType::Read => Some(self.parse_input_statement()?),
            TokenType::Write => Some(self.parse_output_statement()?),
            TokenType::Return => Some(self.parse_return_statement()?),
            TokenType::Identifer => {
                match self.inner.look_after() {
                    Some(TokenType::BracketOpen) => Some(self.parse_call_statement()?),
                    Some(_) => Some(self.parse_assign_statement()?),
                    None => return Err(format!("unexpected EOF after statement Identifer")),
                }
            }
            _ => None,
        })
    }

    fn parse_conditional_statement(&self) -> Result<Statement, String> {
        self.inner.take(TokenType::If)?;
        let condition = self.parse_relexp()?;
        self.inner.take(TokenType::Then)?;
        let body = self.parse_statement_list()?;
        self.inner.take(TokenType::Else)?;
        let else_body = self.parse_statement_list()?;
        self.inner.take(TokenType::Fi)?;
        Ok(Statement::Conditional(ConditionalStatement {
            condition,
            body,
            else_body,
        }))
    }

    fn parse_loop_statement(&self) -> Result<Statement, String> {
        unimplemented!()
    }

    fn parse_input_statement(&self) -> Result<Statement, String> {
        self.inner.take(TokenType::Read)?;
        self.inner.take(TokenType::BracketOpen)?;
        let name = self.inner.take(TokenType::Identifer)?.image.clone();
        self.inner.take(TokenType::BracketClose)?;
        Ok(Statement::Input(name))
    }

    fn parse_output_statement(&self) -> Result<Statement, String> {
        self.inner.take(TokenType::Write)?;
        self.inner.take(TokenType::BracketOpen)?;
        let exp = self.parse_expression()?;
        self.inner.take(TokenType::BracketClose)?;
        // TODO
        Ok(Statement::Output())
    }

    fn parse_return_statement(&self) -> Result<Statement, String> {
        unimplemented!()
    }

    fn parse_call_statement(&self) -> Result<Statement, String> {
        unimplemented!()
    }

    fn parse_assign_statement(&self) -> Result<Statement, String> {
        unimplemented!()
    }

    fn parse_relexp(&self) -> Result<(), String> {
        self.parse_expression()?;
        // TODO: CmdOp
        self.parse_expression()?;
        Ok(())
    }

    // FIXME: type
    fn parse_expression(&self) -> Result<Expression, String> {
        let left = self.parse_term()?;
        let (op, right) = match self.inner.current() {
            TokenType::Add | TokenType::Minus => {
                let op = self.inner.current_token().image.clone();
                let right = self.parse_expression()?;
                (Some(op), Some(Box::new(right)))
            }
            _ => { (None, None) }
        };
        Ok(Expression {
            left,
            op,
            right,
        })
    }

    fn parse_term(&self) -> Result<ExpressionTerm, String> {
        let left = self.parse_factor()?;
        let (op, right) = match self.inner.current() {
            TokenType::Multiply | TokenType::Divide => {
                let op = self.inner.current_token().image.clone();
                let right = self.parse_term()?;
                (Some(op), Some(Box::new(right)))
            }
            _ => { (None, None) }
        };
        Ok(ExpressionTerm {
            left,
            op,
            right,
        })
    }

    fn parse_factor(&self) -> Result<ExpressionFactor, String> {
        match self.inner.current() {
            TokenType::BracketOpen => {
                self.inner.take(TokenType::BracketOpen)?;
                self.parse_expression()?;
                self.inner.take(TokenType::BracketClose)?;
            }
            TokenType::Int => {
                self.inner.take(TokenType::Int)?;
            }
            TokenType::Identifer => {
                let variable = self.inner.take(TokenType::Identifer)?;
                // let more = self.parse_varimore()?;
            }
            _ => return Err(format!("unexpected factor token: {:?}", self.inner.current()))
        }
        // TODO
        Ok(ExpressionFactor {})
    }
}