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
            let mut f = File::open(&args[2]).expect("file not found");
            f.read_to_string(&mut input).expect("Test file not found.");

            let tokens = lexer::lex(&input);
            println!("tokens: \n{:?}\n", tokens);
        }
        "parse" => {
            if args.len() < 3 {
                println!("Please specify file...");
                return;
            }
            let mut f = File::open(&args[2]).expect("file not found");
            f.read_to_string(&mut input).expect("Test file not found.");

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

            match a2.as_str() {
                "1" => {
                    testfile = "1.hello.dart";
                }
                "2" => {
                    testfile = "2.variable.dart";
                }
                "3" => {
                    testfile = "3.addition.dart";
                }
                "14" => {
                    testfile = "14.list_replace.dart";
                }
                "lex" => {
                    action = "lex";
                    if args.len() < 4 {
                        println!("Please specify filename...");
                    }
                    testfile = &args[4];
                }
                "parse" => {
                    action = "parse";
                    if args.len() < 4 {
                        println!("Please specify filename...");
                    }
                    testfile = &args[4];
                }
                "eval" => {
                    if args.len() < 4 {
                        println!("Please specify filename...");
                    }
                    testfile = &args[4];
                }
                x => {
                    testfile = x;
                }
            }

            let mut f = File::open(format!("{}{}", TESTPATH, testfile)).expect("file not found");
            f.read_to_string(&mut input).expect("Test file not found.");

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
        let tokens = lexer::lex(&input);
        let tree = parser::parse(&tokens).unwrap();

        let mut symtable :HashMap<String, evaluator::Object> = HashMap::new();

        for node in tree.children {
            match node.nodetype {
                NodeType::FUNDEF(ref fname) => {
                    if fname == "main" {
                        let maincall: Node = Node { nodetype: NodeType::FUNCALL(String::from("main")), children: node.children };
                        evaluator::eval(&maincall, &mut symtable);
                    }
                }
                _ => {
                    println!("not main: {}", node);
                }
            }
        }
    }

}
