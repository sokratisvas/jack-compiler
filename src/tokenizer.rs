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

pub struct Tokenizer {
    contents: Vec<char>,
    tokens: Vec<String>
}

impl Tokenizer {
    pub fn new(contents: Vec<char>) -> Self {
        Tokenizer {
            contents: contents,
            tokens: Vec::new()
        }
    }

    pub fn tokenize(&mut self) {
        let mut position = 0;
        let mut token = String::new();

        while position != self.contents.len() {
            match self.contents[position] {
                _ if self.contents[position].clone().is_alphabetic() => {
                    token.push(self.contents[position].clone());
                    position += 1;
                    while position != self.contents.len()
                        && (self.contents[position].is_alphanumeric() || self.contents[position] == '_')
                    {
                        token.push(self.contents[position].clone());
                        position += 1;
                    }
                    self.tokens.push(token.clone());
                }
                _ if self.contents[position].clone().is_numeric() => {
                    token.push(self.contents[position].clone());
                    position += 1;
                    while position != self.contents.len() && self.contents[position].clone().is_numeric() {
                        token.push(self.contents[position].clone());
                        position += 1;
                    }
                    self.tokens.push(token.clone());
                }
                _ if is_symbol(self.contents[position].clone()) && self.contents[position].clone() != '/' => {
                    token.push(self.contents[position].clone());
                    self.tokens.push(token.clone());
                    position += 1;
                }
                '/' => {
                    position += 1;

                    if position == self.contents.len() {
                        break;
                    }

                    if self.contents[position].clone() == '/' {
                        while position != self.contents.len() && self.contents[position].clone() != '\n' {
                            position += 1;
                        }
                    } else if self.contents[position].clone() == '*' {
                        position += 1;
                        while self.contents[position].clone() != '*' || self.contents[position + 1].clone() != '/'
                        {
                            position += 1;
                        }
                        position += 2;
                    } else {
                        self.tokens.push('/'.to_string());
                    }
                }
                '\n' | ' ' | '\t' => {
                    position += 1;
                }
                '"' => {
                    position += 1;
                    token.push('"');
                    while position != self.contents.len() && self.contents[position] != '"' {
                        token.push(self.contents[position].clone());
                        position += 1;
                    }
                    position += 1;
                    token.push('"');
                    self.tokens.push(token.clone());
                }
                _ => {
                    position += 1;
                }
            }
            token.clear();
        }
    }

    pub fn write_file(&self, filename: String) -> std::io::Result<()> {
        let outpath: String = filename.replace(".jack", "T-actual.xml");
        let mut output = File::create(outpath)?;
        write!(output, "<tokens>\n")
            .map_err(|err| println!("{:?}", err))
            .ok();
        for token in &self.tokens {
            let token_xml = format!(
                "<{0}> {1} </{0}>\n",
                classify_token(token.clone()),
                markup_token(token.clone())
            );
            write!(output, "{}", token_xml)
                .map_err(|err| println!("{:?}", err))
                .ok();
        }
        write!(output, "</tokens>\n")
            .map_err(|err| println!("{:?}", err))
            .ok();
        Ok(())
    }
}
