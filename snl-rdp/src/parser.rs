use snl_utils::token::{Token, TokenType};
use snl_utils::tokens::Tokens;
use snl_utils::ast::*;
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

    pub fn parse(&self) -> Result<Positional<Program>, String> {
        self.parse_program()
    }

    fn parse_program(&self) -> Result<Positional<Program>, String> {
        let name = self.parse_program_head()?;
        let declare = self.parse_declare_part()?;
        let body = self.parse_program_body()?;
        Ok(Positional::from_position(name.position(), Program {
            name: name.into_inner(),
            declare,
            body,
        }))
    }

    fn parse_program_head(&self) -> Result<Positional<String>, String> {
        let program = self.inner.take(TokenType::Program)?;
        let program_name = self.inner.take(TokenType::Identifier)?.image.clone();
        Ok(Positional::from_token(program, program_name))
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
            self.parse_declare_procedure()?
        } else {
            Default::default()
        };

        Ok(ProgramDeclare {
            type_declare,
            variable_declare,
            procedure_declare,
        })
    }

    fn parse_declare_type(&self) -> Result<PositionalVec<TypeDeclare>, String> {
        let mut declare = Vec::new();
        self.inner.take(TokenType::Type)?;
        loop {
            let name = self.inner.take(TokenType::Identifier)?;
            self.inner.take(TokenType::Equal)?;
            let inner_type = self.parse_type_name(true)?;
            self.inner.take(TokenType::Semicolon)?;
            declare.push(Positional::from_token(name, TypeDeclare {
                base: inner_type,
                name: name.image.clone(),
            }));
            if TokenType::Identifier != self.inner.current() {
                break;
            }
        }
        Ok(declare)
    }

    fn parse_declare_var(&self) -> Result<PositionalVec<TypedIdentifiers>, String> {
        let mut result = PositionalVec::new();
        self.inner.take(TokenType::Var)?;
        loop {
            let type_name = self.parse_type_name(true)?;
            let ids = self.parse_identifier_list()?;
            self.inner.take(TokenType::Semicolon)?;
            result.push(Positional::from_position(
                type_name.position(),
                TypedIdentifiers {
                    type_name,
                    identifiers: ids,
                },
            ));

            if self.inner.current() == TokenType::Procedure || self.inner.current() == TokenType::Begin {
                break;
            }
        }
        Ok(result)
    }

    fn parse_declare_procedure(&self) -> Result<PositionalVec<ProcedureDeclare>, String> {
        let mut result = PositionalVec::new();
        loop {
            self.inner.take(TokenType::Procedure)?;
            let name = self.inner.take(TokenType::Identifier)?;
            self.inner.take(TokenType::BracketOpen)?;
            let params = self.parse_param_list()?;
            self.inner.take(TokenType::BracketClose)?;
            self.inner.take(TokenType::Semicolon)?;
            let declare = self.parse_declare_part()?;
            let body = self.parse_program_body()?;
            result.push(Positional::from_token(name, ProcedureDeclare {
                name: name.image.clone(),
                params,
                declare: Box::new(declare),
                body,
            }));
            if TokenType::Procedure != self.inner.current() {
                break;
            }
        }
        Ok(result)
    }

    fn parse_type_name(&self, full: bool) -> Result<Positional<SNLType>, String> {
        let next = self.inner.current();
        let next_pos = (self.inner.current_token().line, self.inner.current_token().column);
        match next {
            TokenType::Integer => {
                self.inner.move_next();
                return Ok(Positional::from_position(next_pos, SNLType::Integer));
            }
            TokenType::Char => {
                self.inner.move_next();
                return Ok(Positional::from_position(next_pos, SNLType::Char));
            }
            TokenType::Array => {
                return Ok(Positional::from_position(next_pos, SNLType::Array(self.parse_array_type()?)));
            }
            _ => {}
        }
        if full {
            match next {
                TokenType::Record => {
                    return Ok(Positional::from_position(next_pos, SNLType::Record(self.parse_record_type()?)));
                }
                TokenType::Identifier => {
                    let name = self.inner.current_token().image.clone();
                    self.inner.move_next();
                    return Ok(Positional::from_position(next_pos, SNLType::Others(name)));
                }
                _ => {}
            }
        }
        Err(format!("unexpected token {:?}", next))
    }

    fn parse_array_type(&self) -> Result<SNLTypeArray, String> {
        self.inner.take(TokenType::Array)?;
        self.inner.take(TokenType::SquareBracketOpen)?;
        let low = self.inner.take(TokenType::Int)?.image.as_str();
        self.inner.take(TokenType::DotDot)?;
        let top = self.inner.take(TokenType::Int)?.image.as_str();
        self.inner.take(TokenType::SquareBracketClose)?;
        self.inner.take(TokenType::Of)?;
        let base = self.inner.current();
        let base = match base {
            TokenType::Integer => SNLBaseType::Integer,
            TokenType::Char => SNLBaseType::Char,
            t => {
                let token = self.inner.current_token();
                return Err(format!("unexpected base type {:?} at line {}, column {}", t, token.line, token.column));
            }
        };
        self.inner.move_next();
        Ok(SNLTypeArray {
            base,
            lower_bound: usize::from_str(low).unwrap(),
            upper_bound: usize::from_str(top).unwrap(),
        })
    }

    fn parse_record_type(&self) -> Result<SNLTypeRecord, String> {
        let mut records = Vec::new();
        self.inner.take(TokenType::Record)?;
        loop {
            let type_name = self.parse_type_name(false)?;
            let identifiers = self.parse_identifier_list()?;
            self.inner.take(TokenType::Semicolon)?;
            records.push(TypedIdentifiers { type_name: type_name, identifiers });

            match self.inner.current() {
                TokenType::Integer | TokenType::Char | TokenType::Array => {}
                _ => break,
            }
        }
        self.inner.take(TokenType::End)?;
        Ok(records)
    }

    fn parse_identifier_list(&self) -> Result<PositionalVec<String>, String> {
        let mut ids = PositionalVec::new();
        let mut need_comma = false;
        loop {
            match self.inner.current() {
                TokenType::Identifier => {
                    if need_comma {
                        break;
                    } else {
                        ids.push(Positional::from_token_image(self.inner.current_token()));
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
        let statement = self.parse_statement()?.ok_or(format!("empty statement list"))?;
        statements.push(statement);
        while TokenType::Semicolon == self.inner.current() {
            let pos = self.inner.take(TokenType::Semicolon)?;
            let statement = self.parse_statement()?.ok_or(format!("statement expected near line {}, column {}", pos.line, pos.column))?;
            statements.push(statement);
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
            TokenType::Identifier => {
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
        let condition = self.parse_relation_expression()?;
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
        self.inner.take(TokenType::While)?;
        let condition = self.parse_relation_expression()?;
        self.inner.take(TokenType::Do)?;
        let body = self.parse_statement_list()?;
        self.inner.take(TokenType::EndWhile)?;
        Ok(Statement::Loop(LoopStatement { condition, body }))
    }

    fn parse_input_statement(&self) -> Result<Statement, String> {
        self.inner.take(TokenType::Read)?;
        self.inner.take(TokenType::BracketOpen)?;
        let name = self.inner.take(TokenType::Identifier)?;
        self.inner.take(TokenType::BracketClose)?;
        Ok(Statement::Input(Positional::from_token(name, name.image.clone())))
    }

    fn parse_output_statement(&self) -> Result<Statement, String> {
        self.inner.take(TokenType::Write)?;
        self.inner.take(TokenType::BracketOpen)?;
        let exp = self.parse_expression()?;
        self.inner.take(TokenType::BracketClose)?;
        Ok(Statement::Output(exp))
    }

    fn parse_return_statement(&self) -> Result<Statement, String> {
        self.inner.take(TokenType::Return)?;
        self.inner.take(TokenType::BracketOpen)?;
        let exp = self.parse_expression()?;
        self.inner.take(TokenType::BracketClose)?;
        Ok(Statement::Return(exp))
    }

    fn parse_call_statement(&self) -> Result<Statement, String> {
        let mut params = Vec::new();
        let name = self.inner.take(TokenType::Identifier)?;
        self.inner.take(TokenType::BracketOpen)?;
        loop {
            if TokenType::BracketClose == self.inner.current() {
                break;
            }
            if params.len() != 0 {
                self.inner.take(TokenType::Comma)?;
            }
            let param = self.parse_expression()?;
            params.push(param);
        }
        self.inner.take(TokenType::BracketClose)?;
        Ok(Statement::Call(Positional::from_token(&name, CallStatement {
            name: name.image.clone(),
            params,
        })))
    }

    fn parse_assign_statement(&self) -> Result<Statement, String> {
        let base = self.inner.take(TokenType::Identifier)?;
        let visit = self.parse_variable_visit()?;
        self.inner.take(TokenType::Assign)?;
        let value = self.parse_expression()?;
        Ok(Statement::Assign(AssignStatement {
            variable: VariableRepresent {
                base: Positional::from_token(base, base.image.clone()),
                visit,
            },
            value,
        }))
    }

    fn parse_relation_expression(&self) -> Result<RelationExpression, String> {
        let left = self.parse_expression()?;
        let op = match self.inner.current() {
            TokenType::LessThan | TokenType::Equal => {
                let op = self.inner.current_token().image.clone();
                self.inner.move_next();
                op
            }
            t => {
                let token = self.inner.current_token();
                return Err(format!("unexpected token {:?} at line {}, column {}", t, token.line, token.column));
            }
        };
        let right = self.parse_expression()?;
        Ok(RelationExpression {
            left,
            op,
            right,
        })
    }

    fn parse_expression(&self) -> Result<Expression, String> {
        let left = self.parse_term()?;
        let (op, right) = match self.inner.current() {
            TokenType::Add | TokenType::Minus => {
                let op = self.inner.current_token().image.clone();
                self.inner.move_next();
                let right = self.parse_expression()?;
                (Some(op), Some(Positional::from_position(right.left.position(), Box::new(right))))
            }
            _ => { (None, None) }
        };
        Ok(Expression {
            left: Positional::from_position(left.left.position(), left),
            op,
            right,
        })
    }

    fn parse_term(&self) -> Result<ExpressionTerm, String> {
        let left = self.parse_factor()?;
        let (op, right) = match self.inner.current() {
            TokenType::Multiply | TokenType::Divide => {
                let op = self.inner.current_token().image.clone();
                self.inner.move_next();
                let right_token = self.inner.current_token();
                let right = self.parse_term()?;
                (Some(op), Some(Positional::from_token(right_token, Box::new(right))))
            }
            _ => { (None, None) }
        };
        Ok(ExpressionTerm {
            left,
            op,
            right,
        })
    }

    fn parse_factor(&self) -> Result<Positional<ExpressionFactor>, String> {
        let pos = self.inner.current_token().position();
        let inner = match self.inner.current() {
            TokenType::BracketOpen => {
                self.inner.take(TokenType::BracketOpen)?;
                let exp = self.parse_expression()?;
                self.inner.take(TokenType::BracketClose)?;
                ExpressionFactor::Bracket(Box::new(exp))
            }
            TokenType::Int => {
                let num = self.inner.take(TokenType::Int)?.image.as_str();
                ExpressionFactor::Constant(u32::from_str(num).unwrap())
            }
            TokenType::Identifier => {
                let base = self.inner.take(TokenType::Identifier)?;
                let visit = self.parse_variable_visit()?;
                ExpressionFactor::Variable(VariableRepresent {
                    base: Positional::from_token_image(base),
                    visit,
                })
            }
            _ => return Err(format!("unexpected factor token: {:?}", self.inner.current()))
        };
        Ok(Positional::from_position(pos, inner))
    }

    fn parse_param_list(&self) -> Result<PositionalVec<Param>, String> {
        let mut result = PositionalVec::new();
        loop {
            if TokenType::BracketClose == self.inner.current() {
                break;
            }
            if result.len() != 0 {
                self.inner.take(TokenType::Comma)?;
            }
            let param = self.parse_param()?;
            result.push(param);
        }
        Ok(result)
    }

    fn parse_param(&self) -> Result<Positional<Param>, String> {
        let is_var = TokenType::Var == self.inner.current();
        if is_var {
            self.inner.move_next();
        }

        let type_name = self.parse_type_name(true)?;
        let identifiers = self.parse_identifier_list()?;
        Ok(Positional::from_position(type_name.position(), Param {
            is_var,
            definition: TypedIdentifiers {
                type_name,
                identifiers,
            },
        }))
    }

    fn parse_variable_visit(&self) -> Result<Option<VariableVisit>, String> {
        let dot = if TokenType::Dot == self.inner.current() {
            self.inner.take(TokenType::Dot)?;
            Some(self.inner.take(TokenType::Identifier)?)
        } else {
            None
        };

        let sqbr = if TokenType::SquareBracketOpen == self.inner.current() {
            self.inner.take(TokenType::SquareBracketOpen)?;
            let exp = self.parse_expression()?;
            self.inner.take(TokenType::SquareBracketClose)?;
            Some(Box::new(exp))
        } else {
            None
        };
        Ok(if let (None, None) = (&dot, &sqbr) {
            None
        } else {
            Some(VariableVisit {
                dot: dot.map(|d| Positional::from_token(d, d.image.clone())),
                sqbr,
            })
        })
    }
}