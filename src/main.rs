extern crate rustyline;
extern crate nuid;
extern crate queues;

use std::io::prelude::*;
use std::env;
use std::fs::File;

mod parser;
mod lexer;
mod evaluator;
mod builtin;
mod utils;
mod stack;
mod objsys;
mod expression;
mod token;
mod node;
mod object;
mod testlist;
mod context;
mod reader;

use context::Ctx;
use stack::Stack;
use objsys::ObjSys;
use std::collections::HashMap;
use node::{Node, NodeType};


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Argument expected.");
    }

    let mut ctx = Ctx{
        filepath: String::from(""),
        debug: true
    };

    let a1 = &args[1];

    match a1.as_str() {
        "lex" => {
            if args.len() < 3 {
                println!("Please specify file ...");
                return;
            }
            ctx.filepath = String::from(&args[2]);
            do_task("lex", read_file(&args[2]), &ctx);
        }
        "parse" => {
            if args.len() < 3 {
                println!("Please specify file...");
                return;
            }
            ctx.filepath = String::from(&args[2]);
            do_task("parse", read_file(&args[2]), &ctx);
        }
        "test" => {
            if args.len() < 3 {
                println!("Running all tests:");
                for s in testlist::TESTS {
                    println!("Running test: {}", s);
                    let path = format!("{}/{}", testlist::TESTPATH, s);
                    do_task("eval", read_file(path.as_str()), &ctx);
                }
                return;
            }

            let a2 : &String =  &args[2];
            let mut task = "eval";
            let nextarg: &String;

            match a2.as_str() {
                "lex" => {
                    task = "lex";
                    nextarg = &args[3];
                }
                "parse" => {
                    task = "parse";
                    nextarg = &args[3];
                }
                "eval" => {
                    nextarg = &args[3];
                }
                _ => {
                    nextarg = &args[2];
                }
            }

            let filepath = testlist::get_filepath(nextarg.clone());
            ctx.filepath = filepath.clone();
            do_task(task, read_file(filepath.as_str()), &ctx);
        }
        "testfail" => {
            if args.len() < 3 {
                println!("Running all fail tests:");
                for s in testlist::FAILTESTS {
                    let path = format!("{}/{}", testlist::FAILTESTPATH, s);
                    do_task("eval", read_file(path.as_str()), &ctx);
                }
                return;
            }

            let a2 : &String =  &args[2];
            let mut task = "eval";
            let nextarg: &String;

            match a2.as_str() {
                "lex" => {
                    task = "lex";
                    nextarg = &args[3];
                }
                "parse" => {
                    task = "parse";
                    nextarg = &args[3];
                }
                "eval" => {
                    nextarg = &args[3];
                }
                _ => {
                    nextarg = &args[2];
                }
            }

            let filepath = testlist::get_failfilepath(nextarg.clone());
            ctx.filepath = filepath.clone();
            do_task(task, read_file(filepath.as_str()), &ctx);
        }
        _ => {
            println!("Illegal argument: {}", a1);
        }
    }
}


fn do_task(action: &str, input: String, ctx: &Ctx) {

    match action {
        "lex" => {
            let reader = lexer::lex(&input);
            for t in reader.tokens() {
                print!("{} ", t);
            }
            println!();
        }
        "parse" => {
            let mut tokens = lexer::lex(&input);
            let tree = parser::parse(&mut tokens, ctx).unwrap();
            println!("\n{}\n", tree);
        }
        "eval" => {
            evaluate(&input, ctx);
        }
        x => {
            println!("Unknown action: {}", x);
        }
    }
}


fn evaluate(input: &String, ctx: &Ctx) {

    let mut tokens = lexer::lex(&input);
    let tree = parser::parse(&mut tokens, ctx).unwrap();

    let mut store = Stack::new();
    let mut objsys = ObjSys::new();
    // let mut globals : HashMap<String, Object> = HashMap::new();
    let mut globals : HashMap<String, Node> = HashMap::new();

    evaluator::preval(&tree, &mut globals, &mut objsys, ctx);

    if !globals.contains_key("main") {
        // As Dart.
        panic!("Error: No 'main' method found.");
    }

    let mainfunc = &globals.get("main").unwrap().clone();

    match &mainfunc.nodetype {
        NodeType::FunDef(_) => {
            utils::dprint(" ");
            utils::dprint("EVALUATE");
            utils::dprint(" ");

            let mainbody = &mainfunc.children[1];

            store.push_call();
            evaluator::eval(mainbody, &mut globals, &mut store, &mut objsys, ctx);
            store.pop_call();
        }
        x => {
            panic!("Unexpected type of 'main': {:?}", x)
        }
    }
}


fn read_file(filepath: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(filepath).expect(format!("File not found: {}.", filepath).as_str());
    f.read_to_string(&mut input).expect("Error when reading input file.");
    return input;
}

