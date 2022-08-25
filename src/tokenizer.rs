use std::fs::File;
use std::io::prelude::*;

pub fn is_symbol(token: char) -> bool {
    token == '{'
        || token == '}'
        || token == '['
        || token == ']'
        || token == '('
        || token == ')'
        || token == '.'
        || token == ','
        || token == ';'
        || token == '+'
        || token == '-'
        || token == '*'
        || token == '/'
        || token == '&'
        || token == '|'
        || token == '<'
        || token == '>'
        || token == '='
        || token == '~'
}

pub fn is_keyword(token: String) -> bool {
    token == "class".to_string()
        || token == "constructor".to_string()
        || token == "function".to_string()
        || token == "method".to_string()
        || token == "field".to_string()
        || token == "static".to_string()
        || token == "var".to_string()
        || token == "int".to_string()
        || token == "char".to_string()
        || token == "boolean".to_string()
        || token == "void".to_string()
        || token == "true".to_string()
        || token == "false".to_string()
        || token == "null".to_string()
        || token == "this".to_string()
        || token == "let".to_string()
        || token == "do".to_string()
        || token == "if".to_string()
        || token == "else".to_string()
        || token == "while".to_string()
        || token == "return".to_string()
}

pub fn tokenize(contents: Vec<char>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut position = 0;
    let mut token = String::new();

    while position != contents.len() {
        match contents[position] {
            _ if contents[position].clone().is_alphabetic() => {
                token.push(contents[position].clone());
                position += 1;
                while position != contents.len()
                    && (contents[position].is_alphanumeric() || contents[position] == '_')
                {
                    token.push(contents[position].clone());
                    position += 1;
                }
                tokens.push(token.clone());
            }
            _ if contents[position].clone().is_numeric() => {
                token.push(contents[position].clone());
                position += 1;
                while position != contents.len() && contents[position].clone().is_numeric() {
                    token.push(contents[position].clone());
                    position += 1;
                }
                tokens.push(token.clone());
            }
            _ if is_symbol(contents[position].clone()) && contents[position].clone() != '/' => {
                token.push(contents[position].clone());
                tokens.push(token.clone());
                position += 1;
            }
            '/' => {
                position += 1;

                if position == contents.len() {
                    break;
                }

                if contents[position].clone() == '/' {
                    while position != contents.len() && contents[position].clone() != '\n' {
                        position += 1;
                    }
                } else if contents[position].clone() == '*' {
                    position += 1;
                    while contents[position].clone() != '*' || contents[position + 1].clone() != '/'
                    {
                        position += 1;
                    }
                    position += 2;
                } else {
                    tokens.push('/'.to_string());
                }
            }
            '\n' | ' ' | '\t' => {
                position += 1;
            }
            '"' => {
                position += 1;
                token.push('"');
                while position != contents.len() && contents[position] != '"' {
                    token.push(contents[position].clone());
                    position += 1;
                }
                position += 1;
                token.push('"');
                tokens.push(token.clone());
            }
            _ => {
                position += 1;
            }
        }
        token.clear();
    }
    tokens
}

pub fn classify_token(token: String) -> String {
    match token {
        _ if is_keyword(token.clone()) => "keyword".to_string(),
        _ if is_symbol(token.clone().chars().next().unwrap()) => "symbol".to_string(),
        _ if token.clone().starts_with('"') && token.clone().ends_with('"') => {
            "stringConstant".to_string()
        }
        _ if token.clone().parse::<u16>().is_ok() => {
            // Integers 0 upto 2^16 - 1
            "integerConstant".to_string()
        }
        _ => "identifier".to_string(),
    }
}

pub fn markup_token(token: String) -> String {
    match token.as_str() {
        _ if token.clone().starts_with('"') && token.clone().ends_with('"') => {
            let mut chars = token.chars();
            chars.next();
            chars.next_back();
            chars.as_str().to_string()
        }
        ">" => "&gt;".to_string(),
        "<" => "&lt;".to_string(),
        "&" => "&amp;".to_string(),
        "\"" => "&quot;".to_string(),
        _ => token.clone(),
    }
}

pub fn tokenizer_output(tokens: Vec<String>, filename: String) -> std::io::Result<()> {
    let outpath: String = filename.replace(".jack", "T-actual.xml");
    let mut output = File::create(outpath)?;
    write!(output, "<tokens>\n")
        .map_err(|err| println!("{:?}", err))
        .ok();
    for token in tokens {
        write!(output, "{}", token)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }
    write!(output, "</tokens>\n")
        .map_err(|err| println!("{:?}", err))
        .ok();
    Ok(())
}
