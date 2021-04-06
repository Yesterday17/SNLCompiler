use crate::predict::{PredictTable, PredictValue};
use crate::construct::{ConstructTable, ASTNodeValue};
use snl_utils::ast::{Positional, Program};
use snl_utils::token::Token;
use snl_utils::tokens::Tokens;

pub struct Parser {
    predict: PredictTable,

    construct: Vec<&'static str>,
    constructor: ConstructTable,

    tokens: Tokens,

    stack: Vec<PredictValue>,
    stack_offset: Vec<usize>,

    params: Vec<ASTNodeValue>,
    param_offset: Vec<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            predict: Default::default(),

            construct: Default::default(),
            constructor: Default::default(),

            tokens: Tokens::new(tokens),

            stack: vec![PredictValue::NonTerminal("Program")],
            stack_offset: Default::default(),

            params: Default::default(),
            param_offset: Default::default(),
        }
    }

    pub fn parse(&mut self) -> Result<Positional<Program>, String> {
        loop {
            if self.stack.len() == 0 && self.stack_offset.len() == 0 {
                break;
            }
            if let Some(offset) = self.stack_offset.last() {
                if offset == &self.stack.len() {
                    // remove stack offset
                    self.stack_offset.pop();
                    // remove and get offset
                    let start_offset = self.param_offset.pop().unwrap();
                    // params
                    let mut params: Vec<_> = Default::default();
                    for _ in 0..(self.params.len() - start_offset) {
                        params.push(self.params.pop().unwrap());
                    }
                    // handle rule
                    let result = self.constructor.construct(self.construct.pop().unwrap(), params)?;
                    // add new parameter
                    self.params.push(result);
                    // continue
                    continue;
                }
            }
            match self.tokens.now_token() {
                Some(current) => {
                    match self.stack.last().unwrap().clone() {
                        PredictValue::Terminal(terminal) => {
                            if terminal != current.token_type {
                                return Err(format!("Expected {:?}, got {:?}", terminal, current.token_type));
                            }
                            self.stack.pop();
                            self.tokens.move_next();
                            self.params.push(ASTNodeValue::Terminal(current.clone()))
                        }
                        PredictValue::NonTerminal(non_terminal) => {
                            match self.predict.lookup(non_terminal, current.token_type) {
                                Some(rule) => {
                                    // remove non terminal token from the top of stack
                                    self.stack.pop();
                                    // simpler procedure for empty rule
                                    if rule.is_empty() {
                                        self.params.push(ASTNodeValue::None);
                                    } else {
                                        // add non terminal name
                                        self.construct.push(non_terminal);
                                        // add stack offset
                                        self.stack_offset.push(self.stack.len());
                                        // add param start offset
                                        self.param_offset.push(self.params.len());
                                        // add values to stack
                                        for val in rule.iter().rev() {
                                            self.stack.push(val.clone())
                                        }
                                    }
                                }
                                None => return Err(format!("No predict rule found for rule: {}, token: {:?}", non_terminal, current.token_type))
                            }
                        }
                    }
                }
                None => return Err("Unexpected EOF".to_owned())
            }
        }
        Err(format!(""))
    }
}