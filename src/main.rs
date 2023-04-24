extern crate rustyline;

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::collections::HashMap;

mod parser;
mod lexer;
mod evaluator;
mod builtin;

use parser::Node;
use parser::NodeType;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        let a = &args[1];
        let mut f = File::open("/usr/home/kt/devel/redart/dartprogs/hello.dart").expect("file not found");
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

                println!(" ");
                println!("LEX");
                println!(" ");
                let tokens = lexer::tokenize(&input);
                println!(" ");
                println!("PARSE");
                println!(" ");
                let tree = parser::parse(&tokens).unwrap();
                println!(" ");
                println!("EVALUATE");
                println!(" ");
                //let result = evaluator::eval(&tree, &mut symtable);
                // println!("result: {:?}", result);

                // println!("main: {:?}", symtable["main"]);

                for node in tree.children {
                    // println!("child: {:?}", node);
                    match node.nodetype {
                        NodeType::FUNDEF(ref fname) => {
                            // println!("main");
                            // let maincall = Object::FUNCTION(String.from(fname), body);
                            // Pre-eval creates a fundef. Now we need to create a funcall
                            // and call it.
                            // let maincall : Node = Node(NodeType::FUNCALL(String::from("main")));
                            let maincall : Node = Node { nodetype: NodeType::FUNCALL(String::from("main")), children: node.children };
                            // let maincall = Node(NodeType::FUNCALL("main"));
                            // let params = node.children[0].clone();
                            // let body = node.children[1].clone();
                            // maincall.children.push(params);
                            // maincall.children.push(body);
                            evaluator::eval(&maincall, &mut symtable);
                        }
                        _ => {
                            println!("not main");
                        }
                    }
                }



                // let dartmain = symtable["main"].clone();
                // match dartmain {
                //     Object::FUNCTION(ref fname , ref node) => {
                //         evaluator::eval(node, &mut symtable);
                //     }
                //     _ => panic!("Main was not a function: {:?}", dartmain)
                // }

                // evaluator::eval(Object::FUNCTION(), &mut symtable);


            }
        }
//        run(&input, &mut symtable);
    }
    else {
        eprintln!("Argument expected.");
    }
}
