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
                println!("Please specify test case...");
                return;
            }
            let a2 : &String =  &args[2];
            match a2.as_str() {
                "1" => {
                    let mut f = File::open(format!("{}1.hello.dart", TESTPATH,)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                "2" => {
                    let mut f = File::open(format!("{}2.variable.dart", TESTPATH)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                "3" => {
                    let mut f = File::open(format!("{}3.addition.dart", TESTPATH)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                "14" => {
                    let mut f = File::open(format!("{}14.list_replace.dart", TESTPATH)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                _ => panic!("Test not found!")
            }

            let tokens = lexer::lex(&input);
            println!("tokens: \n{:?}\n", tokens);
        }
        "parse" => {
            if args.len() < 3 {
                println!("Please specify test case...");
                return;
            }
            let a2 : &String =  &args[2];

            match a2.as_str() {
                "1" => {
                    let mut f = File::open(format!("{}1.hello.dart", TESTPATH,)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                "2" => {
                    let mut f = File::open(format!("{}2.variable.dart", TESTPATH)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                "3" => {
                    let mut f = File::open(format!("{}3.addition.dart", TESTPATH)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                "14" => {
                    let mut f = File::open(format!("{}14.list_replace.dart", TESTPATH)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                _ => panic!("Test not found!")
            }
            let tokens = lexer::lex(&input);
            let tree = parser::parse(&tokens).unwrap();
            println!("\n{}\n", tree);
        }
        "test" => {
            if args.len() < 3 {
                println!("Please specify test case...");
                return;
            }
            let a2 : &String =  &args[2];

            match a2.as_str() {
                "1" => {
                    let mut f = File::open(format!("{}1.hello.dart", TESTPATH,)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                "2" => {
                    let mut f = File::open(format!("{}2.variable.dart", TESTPATH)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                "3" => {
                    let mut f = File::open(format!("{}3.addition.dart", TESTPATH)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                "14" => {
                    let mut f = File::open(format!("{}14.list_replace.dart", TESTPATH)).expect("file not found");
                    f.read_to_string(&mut input).expect("Test file not found.");
                }
                _ => panic!("Test not found!")
            }

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
        _ => {
            println!("Illegal argument: {}", a1);
        }
    }

}
