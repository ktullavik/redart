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
            do_task("lex",args[2].clone(), &ctx);
        }
        "parse" => {
            if args.len() < 3 {
                println!("Please specify file...");
                return;
            }
            ctx.filepath = String::from(&args[2]);
            do_task("parse",args[2].clone(), &ctx);
        }
        "test" => {
            if args.len() < 3 {
                println!("Running all tests:");
                for s in testlist::TESTS {
                    println!("Running test: {}", s);
                    let path = format!("{}/{}", testlist::TESTPATH, s);
                    do_task("eval", String::from(path.as_str()), &ctx);
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
            do_task(task, filepath, &ctx);
        }
        "testfail" => {
            if args.len() < 3 {
                println!("Running all fail tests:");
                for s in testlist::FAILTESTS {
                    let path = format!("{}/{}", testlist::FAILTESTPATH, s);
                    do_task("eval", String::from(path.as_str()), &ctx);
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
            do_task(task, filepath, &ctx);
        }
        _ => {
            println!("Illegal argument: {}", a1);
        }
    }
}


fn do_task(action: &str, filepath: String, ctx: &Ctx) {

    match action {
        "lex" => {
            let input = read_file(filepath.as_str());
            let reader = lexer::lex(&input);
            for t in reader.tokens() {
                print!("{} ", t);
            }
            println!();
        }
        "parse" => {
            let input = read_file(filepath.as_str());
            let mut tokens = lexer::lex(&input);
            let mut globals = HashMap::new();
            let mut objsys = ObjSys::new();
            parser::parse(&mut tokens, &mut globals, &mut objsys, ctx);

            for k in globals.keys() {
                let tree = globals.get(k).unwrap();
                println!("\n{}\n", tree);
            }
        }
        "eval" => {
            evaluate(filepath, ctx);
        }
        x => {
            println!("Unknown action: {}", x);
        }
    }
}



fn filecurse(basepath: String, filepath: String, globals: &mut HashMap<String, Node>, store: &mut Stack, objsys: &mut ObjSys, ctx: &Ctx) {

    let mut fpath = basepath.clone();
    fpath.push_str(filepath.as_str());

    println!("basepath: {}, filepath: {}", basepath, filepath);

    let input = read_file(fpath.as_str());
    let mut tokens = lexer::lex(&input);


    let imports = parser::parse(&mut tokens, globals, objsys, ctx);

    for s in imports {
        filecurse(basepath.clone(), s, globals, store, objsys, ctx);
    }
}



fn evaluate(filepath: String, ctx: &Ctx) {

    let mut globals: HashMap<String, Node> = HashMap::new();
    let mut store = Stack::new();
    let mut objsys = ObjSys::new();

    let parts = filepath.split("/");
    let mut vecparts: Vec<&str> = parts.collect();

    let filename = vecparts.remove(vecparts.len() - 1);
    let mut basepath: String = String::new();

    for s in vecparts {
        basepath.push_str(s);
        basepath.push_str("/");
    }


    filecurse(basepath, String::from(filename), &mut globals, &mut store, &mut objsys, ctx);


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

