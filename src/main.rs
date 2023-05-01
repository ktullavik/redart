extern crate rustyline;

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::collections::HashMap;

mod parser;
mod lexer;
mod evaluator;
mod builtin;
mod utils;

use parser::Node;
use parser::NodeType;
use evaluator::Object;


static TESTPATH: &str = "/usr/home/kt/devel/redart/test/";



fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Argument expected.");
    }

    let a1 = &args[1];
    let mut input = String::new();

    match a1.as_str() {
        "lex" => {
            if args.len() < 3 {
                println!("Please specify file ...");
                return;
            }

            input = read_inputfile(&args[2]);
            let tokens = lexer::lex(&input);
            println!("\n{:?}\n", tokens);
        }
        "parse" => {
            if args.len() < 3 {
                println!("Please specify file...");
                return;
            }

            input = read_inputfile(&args[2]);
            let tokens = lexer::lex(&input);
            let tree = parser::parse(&tokens).unwrap();
            println!("\n{}\n", tree);
        }
        "test" => {

            if args.len() < 3 {
                println!("Please specify filename...");
                return;
            }
            let a2 : &String =  &args[2];
            let mut action = "eval";
            let testfile: &str;
            let nextarg: &String;

            match a2.as_str() {
                "lex" => {
                    action = "lex";
                    nextarg = &args[3];
                }
                "parse" => {
                    action = "parse";
                    nextarg = &args[3];
                }
                "eval" => {
                    nextarg = &args[3];
                }
                _ => {
                    nextarg = &args[2];
                }
            }

            input = read_inputfile(nextarg);


            if action == "lex" {
                let tokens = lexer::lex(&input);
                println!("tokens: \n{:?}\n", tokens);
            }
            else if action == "parse" {
                let tokens = lexer::lex(&input);
                let tree = parser::parse(&tokens).unwrap();
                println!("\n{}\n", tree);
            }
            else if action == "eval" {
                evaluate(&input);
            }
            else {
                println!("Unknown action: {}", action);
            }
        }
        _ => {
            println!("Illegal argument: {}", a1);
        }
    }


    fn evaluate(input: &String) {

        utils::dprint(String::from(" "));
        utils::dprint(String::from("EVALUATE"));
        utils::dprint(String::from(" "));

        let tokens = lexer::lex(&input);
        let tree = parser::parse(&tokens).unwrap();

        let mut symtable :HashMap<String, evaluator::Object> = HashMap::new();

        evaluator::preval(&tree, &mut symtable);

        let mainfunc = &symtable.remove("main").unwrap();

        match mainfunc {
            Object::FUNCTION(s, n, v) => {
                evaluator::eval(n, &mut symtable);
            }
            x => {
                panic!("Unexpected type of 'main': {:?}", x)
            }
        }
    }


    fn read_inputfile(s: &str) -> String {
        let testfile: &str;

        match s {
            "1" => {
                testfile = "1.hello.dart";
            }
            "2" => {
                testfile = "2.variable.dart";
            }
            "3" => {
                testfile = "3.addition.dart";
            }
            "4" => {
                testfile = "4.subtraction.dart";
            }
            "5" => {
                testfile = "5.multiplication.dart";
            }
            "6" => {
                testfile = "6.division.dart";
            }
            "7" => {
                testfile = "7.funcall.dart";
            }
            "8" => {
                testfile = "8.argpass.dart";
            }
            "14" => {
                testfile = "14.list_replace.dart";
            }
            s => {
                testfile = s;
            }
        }

        let mut input = String::new();
        let mut f = File::open(format!("{}{}", TESTPATH, testfile)).expect("file not found");
        f.read_to_string(&mut input).expect("Test file not found.");
        return input;
    }

}
