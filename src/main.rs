use std::fs::File;
use std::io::{BufRead, BufReader};
mod tokenizer;

fn main() {
    let files = vec![
        "examples/ArrayTest/Main.jack", 
        "examples/ExpressionLessSquare/Main.jack",
        "examples/ExpressionLessSquare/Square.jack",
        "examples/ExpressionLessSquare/SquareGame.jack",
        "examples/Square/Main.jack",
        "examples/Square/Square.jack",
        "examples/Square/SquareGame.jack",
    ];

    for filename in files {
        let mut buf: Vec<u8> = Vec::new();
        let mut contents: Vec<char> = Vec::new();
        let mut file = BufReader::new(
            File::open(filename.clone()).expect("Something went wrong reading the file"),
        );

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
        let mut tokenizer = tokenizer::Tokenizer::new(contents);
        tokenizer.tokenize();
        tokenizer.write_file(filename.to_string().clone())
            .map_err(|err| println!("{:?}", err))
            .ok();
    }
}
