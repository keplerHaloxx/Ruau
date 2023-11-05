use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    String,
    Integer,
    Float,
    Boolean,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LCurly,
    RCurly,
    Identifier,
    Keyword,
    Function,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) value: TokenValue,
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    StrValue(String),
    IntValue(i32),
    FloatValue(f32),
    BoolValue(bool),
    Identifier(String),
    Keyword(String),
    NoValue,
}

#[derive(Debug, PartialEq)]
pub enum Keywords {
    Fn,
    If,
    Let,
    Else,
    True,
    False,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let keywords: HashSet<&str> = ["if", "fn", "let", "else", "true", "false"]
        .iter()
        .cloned()
        .collect();
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_string = false;

    for c in input.chars() {
        match c {
            ' ' if !in_string => {
                push_token(&mut tokens, &mut current_token, &keywords);
            }
            ';' => {
                push_token(&mut tokens, &mut current_token, &keywords);
                tokens.push(Token {
                    token_type: TokenType::Semicolon,
                    value: TokenValue::NoValue,
                });
            }
            '(' if !in_string => {
                push_token(&mut tokens, &mut current_token, &keywords);
                tokens.push(Token {
                    token_type: TokenType::LParen,
                    value: TokenValue::NoValue,
                });
            }
            ')' if !in_string => {
                push_token(&mut tokens, &mut current_token, &keywords);
                tokens.push(Token {
                    token_type: TokenType::RParen,
                    value: TokenValue::NoValue,
                });
            }
            '{' if !in_string => {
                push_token(&mut tokens, &mut current_token, &keywords);
                tokens.push(Token {
                    token_type: TokenType::LBrace,
                    value: TokenValue::NoValue,
                });
            }
            '}' if !in_string => {
                push_token(&mut tokens, &mut current_token, &keywords);
                tokens.push(Token {
                    token_type: TokenType::RBrace,
                    value: TokenValue::NoValue,
                });
            }
            '"' | '\'' => {
                if in_string && !current_token.is_empty() {
                    tokens.push(Token {
                        token_type: TokenType::String,
                        value: TokenValue::StrValue(current_token.clone()),
                    });
                    current_token.clear();
                }
                in_string = !in_string;
            }
            '\n' => {}
            _ => {
                current_token.push(c);
            }
        }
    }

    push_token(&mut tokens, &mut current_token, &keywords);

    tokens
}

fn push_token(tokens: &mut Vec<Token>, current_token: &mut String, keywords: &HashSet<&str>) {
    if !current_token.is_empty() {
        tokens.push(parse_token(&current_token, &keywords));
        current_token.clear();
    }
}

pub fn parse_token(token: &str, keywords: &HashSet<&str>) -> Token {
    match token {
        "true" => Token {
            token_type: TokenType::Boolean,
            value: TokenValue::BoolValue(true),
        },
        "false" => Token {
            token_type: TokenType::Boolean,
            value: TokenValue::BoolValue(false),
        },
        _ => {
            if keywords.contains(&token) {
                Token {
                    token_type: TokenType::Keyword,
                    value: TokenValue::Keyword(token.to_string()),
                }
            } else {
                match token.parse::<i32>() {
                    Ok(value) => Token {
                        token_type: TokenType::Integer,
                        value: TokenValue::IntValue(value),
                    },
                    Err(_) => match token.parse::<f32>() {
                        Ok(value) => Token {
                            token_type: TokenType::Float,
                            value: TokenValue::FloatValue(value),
                        },
                        Err(_) => {
                            if token.contains("!") {
                                return Token {
                                    token_type: TokenType::Function,
                                    value: TokenValue::Identifier(token.to_string()),
                                };
                            }
                            Token {
                                token_type: TokenType::Identifier,
                                value: TokenValue::Identifier(token.to_string()),
                            }
                        }
                    },
                }
            }
        }
    }
}
