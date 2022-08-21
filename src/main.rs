use std::fs::File;
use std::io::{BufRead, BufReader};
mod tokenizer;

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
    let tokens = tokenizer::tokenize(contents);
    for token in tokens {
        println!(
            "{0} \t <{1}> {2} </{1}>",
            token,
            tokenizer::classify_token(token.clone()),
            tokenizer::markup_token(token.clone())
        );
    }
}
