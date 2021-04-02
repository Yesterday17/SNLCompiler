use crate::token::{Token, TokenType};
use std::str::FromStr;

#[derive(PartialEq)]
enum LexerState {
    Start,
    Comment,
    InputIdentifier,
    InputInteger,
    InputAssign,
    InputDot,
}

pub fn read_tokens(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    let mut state = LexerState::Start;
    let mut image = String::new();

    let mut line = 1u32;
    let mut column = 1u32;

    let mut start_line = 0;
    let mut start_column = 0;

    let mut chars: Vec<_> = input.chars().collect();
    chars.push(' ');
    let mut i = 0;
    loop {
        if i >= chars.len() {
            break;
        }

        let ch = chars[i];
        if ch == '\n' && state == LexerState::Start {
            line += 1;
            column = 1;
        } else {
            column += ch.len_utf8() as u32;
        }

        match state {
            LexerState::Start => {
                i += 1;
                if ch.is_whitespace() {
                    continue;
                }
                image.push(ch);

                start_line = line;
                start_column = column;
                if ch.is_alphabetic() {
                    state = LexerState::InputIdentifier;
                } else if ch.is_numeric() {
                    state = LexerState::InputInteger;
                } else if ch == ':' {
                    state = LexerState::InputAssign;
                } else if ch == '.' {
                    state = LexerState::InputDot;
                } else if ch == '{' {
                    image.pop();
                    state = LexerState::Comment;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::from_str(&image)?,
                        image,
                        line: start_line,
                        column: start_column,
                    });
                    image = String::new();
                    continue;
                }
            }
            LexerState::Comment => {
                i += 1;
                if ch != '}' {
                    continue;
                }
                state = LexerState::Start;
            }
            LexerState::InputIdentifier => {
                if ch.is_alphanumeric() {
                    i += 1;
                    image.push(ch);
                } else {
                    tokens.push(Token {
                        token_type: TokenType::from_str(&image).unwrap_or(TokenType::Identifer),
                        image: image,
                        line: start_line,
                        column: start_column,
                    });
                    image = String::new();
                    state = LexerState::Start;
                }
            }
            LexerState::InputInteger => {
                if ch.is_numeric() {
                    i += 1;
                    image.push(ch);
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Int,
                        image: image,
                        line: start_line,
                        column: start_column,
                    });
                    image = String::new();
                    state = LexerState::Start;
                }
            }
            LexerState::InputAssign => {
                state = LexerState::Start;
                if ch == '=' {
                    i += 1;
                    image.push(ch);
                    tokens.push(Token {
                        token_type: TokenType::Assign,
                        image: image,
                        line: start_line,
                        column: start_column,
                    });
                    image = String::new();
                } else {
                    return Err(format!("invalid character {} after ':', expected '='", ch));
                }
            }
            LexerState::InputDot => {
                if ch == '.' {
                    i += 1;
                    image.push(ch);
                }
                tokens.push(Token {
                    token_type: TokenType::from_str(&image)?,
                    image: image,
                    line: start_line,
                    column: start_column,
                });
                image = String::new();
                state = LexerState::Start;
            }
        }
    }

    tokens.push(Token {
        token_type: TokenType::EOF,
        image: "".to_string(),
        line,
        column,
    });
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::read_tokens;

    #[test]
    fn test_read_token() {
        let result = read_tokens(r#"program p
  type t = integer;
  var t    v1;
      char v2;
  begin
    read(v1);
    v1 := v1 + 10;
    write(v1);
end."#).unwrap();
        for token in result {
            println!("{}\t{:?}", token.line, token.token_type);
        }
    }
}