use crate::lexer::lexer::TokenValue::{Identifier, Keyword, StrValue};
use crate::lexer::lexer::{Token, TokenType, TokenValue};

pub fn translate(tokens: &Vec<Token>) -> String {
    let (mut in_string, mut in_function) = (false, false);
    let (mut doing_function) = (false);

    let mut end_result: String = String::new();

    for (pos, token) in tokens.iter().enumerate() {
        match token.token_type {
            // ! KEYWORD
            TokenType::Keyword => {
                if let TokenValue::Keyword(keyword_value) = &token.value {
                    if keyword_value == "fn" {
                        end_result += "function ";
                    }
                }
            }
            // ! IDENTIFIER
            TokenType::Identifier => {
                if let TokenValue::Identifier(indentifier_value) = &token.value {
                    end_result += format!("{indentifier_value} ").as_str();
                }
            }
            // ! PARENS / BRACES
            TokenType::LParen => {
                end_result += "(";
            }
            TokenType::RParen => {
                if doing_function {
                    end_result += ")";
                    doing_function = false;
                    continue;
                }
                end_result += ") ";
            }
            TokenType::LBrace => {
                end_result += "{\n";
            }
            TokenType::RBrace => {
                end_result += "\n}\n";
            }
            // ! FUNCTION
            TokenType::Function => {
                doing_function = true;
                if let Identifier(function_name) = &token.value {
                    if function_name == "println!" {
                        end_result += "print";
                    }
                }
            }
            // ! STRING
            TokenType::String => {
                if let StrValue(string) = &token.value {
                    end_result += format!("\"{string}\"").as_str();
                }
            }
            // ! SEMICOLON
            TokenType::Semicolon => {
                if tokens[pos + 1].token_type == TokenType::RBrace {
                    end_result += ";"; continue;
                }
                end_result += ";\n"
            }

            _ => {}
        }
    }

    // run main function
    end_result += "main()";

    end_result
}
