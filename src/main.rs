extern crate rustyline;

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::collections::HashMap;
use rustyline::error::ReadlineError;
use rustyline::Editor;
mod parser;
mod lexer;
mod evaluator;
mod builtin;


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        let a = &args[1];
        let mut f = File::open("/usr/home/kt/dev/redart/dartprogs/main.dart").expect("file not found");
        let mut input = String::new();
        f.read_to_string(&mut input)
            .expect("something went wrong reading the file");

        match a.as_str() {
            "lex" => {
                let tokens = lexer::tokenize(&input);
                println!("tokens: \n{:?}\n", tokens);
            }
            "parse" => {
                let tokens = lexer::tokenize(&input);
                let tree = parser::parse(&tokens).unwrap();
                println!("\n{}\n", tree);
            }
            url => {

                let mut symtable :HashMap<String, evaluator::Object> = HashMap::new();

                let tokens = lexer::tokenize(&input);
                let tree = parser::parse(&tokens).unwrap();
                let result = evaluator::eval(&tree, &mut symtable);
                println!("result: {:?}", result);
            }
        }
//        run(&input, &mut symtable);
    }
    else {
        eprintln!("Argument expected.");
    }
}
