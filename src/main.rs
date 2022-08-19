use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_symbol(token: char) -> bool {
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

fn is_keyword(token: String) -> bool {
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

fn tokenize(contents: Vec<char>) -> Vec<String> {
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
                    break;
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
                panic!("Invalid character!");
            }
        }
        token.clear();
    }
    tokens
}

fn main() {
    let filename = String::from("examples/prog.jack");
    let mut buf = Vec::<u8>::new();
    let mut contents: Vec<char> = Vec::new();
    let mut file =
        BufReader::new(File::open(filename).expect("Something went wrong reading the file"));

    while file.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
        let s = String::from_utf8(buf).expect("from_utf8 failed");
        for c in s.chars() {
            contents.push(c);
            // println!("Character: {}", c);
        }

        buf = s.into_bytes();
        buf.clear();
    }

    // print!("{:?}", contents);
    let tokens = tokenize(contents);
    for token in tokens {
        println!("{}", token);
    }
}
