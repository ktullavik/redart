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

use object::Object;
use stack::Stack;
use objsys::ClassMap;
use objsys::InstanceMap;
use std::collections::HashMap;



fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Argument expected.");
    }

    let mut ctx = HashMap::new();
    ctx.insert("filename", String::from(""));

    let a1 = &args[1];

    match a1.as_str() {
        "lex" => {
            if args.len() < 3 {
                println!("Please specify file ...");
                return;
            }
            ctx.insert("filepath", String::from(&args[2]));
            do_task("lex", read_file(&args[2]), &ctx);
        }
        "parse" => {
            if args.len() < 3 {
                println!("Please specify file...");
                return;
            }
            ctx.insert("filepath", String::from(&args[2]));
            do_task("parse", read_file(&args[2]), &ctx);
        }
        "test" => {
            if args.len() < 3 {
                println!("Running all tests:");
                for testindex in 1 .. 62 {
                    println!("Running test: {}", testindex);
                    let filepath = testlist::get_filepath(testindex.to_string());
                    ctx.insert("filepath", filepath.clone());
                    do_task("eval", read_file(filepath.as_str()), &ctx);
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
            ctx.insert("filepath", filepath.clone());
            do_task(task, read_file(filepath.as_str()), &ctx);
        }
        "testfail" => {
            if args.len() < 3 {
                println!("Running all fail tests:");
                for testindex in 1 .. 5 {
                    println!("Running test: {}", testindex);
                    let filepath = testlist::get_failfilepath(testindex.to_string());
                    ctx.insert("filepath", filepath.clone());
                    do_task("eval", read_file(filepath.as_str()), &ctx);
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
            ctx.insert("filepath", filepath.clone());
            do_task(task, read_file(filepath.as_str()), &ctx);
        }
        _ => {
            println!("Illegal argument: {}", a1);
        }
    }


    fn do_task(action: &str, input: String, ctx: &HashMap<&str, String>) {

        match action {
            "lex" => {
                let tokens = lexer::lex(&input);
                for t in tokens {
                    print!("{} ", t);
                }
                println!();
            }
            "parse" => {
                let tokens = lexer::lex(&input);
                let tree = parser::parse(&tokens, ctx).unwrap();
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


    fn evaluate(input: &String, ctx: &HashMap<&str, String>) {

        let tokens = lexer::lex(&input);
        let tree = parser::parse(&tokens, ctx).unwrap();

        let mut store = Stack::new();
        let mut classlist = ClassMap::new();
        let mut instlist = InstanceMap::new();
        let mut globals : HashMap<String, Object> = HashMap::new();

        evaluator::preval(&tree, &mut globals, &mut store, &mut classlist, &mut instlist, ctx);

        if globals.contains_key("main") {
            let mainfunc = &globals.get("main").unwrap().clone();

            match mainfunc {
                Object::Function(_, n, _) => {
                    utils::dprint(" ");
                    utils::dprint("EVALUATE");
                    utils::dprint(" ");

                    store.push_call();
                    evaluator::eval(n, &mut globals, &mut store, &mut classlist, &mut instlist, ctx);
                    store.pop_call();
                }
                x => {
                    panic!("Unexpected type of 'main': {:?}", x)
                }
            }
        }
        else {
            // As Dart.
            panic!("Error: No 'main' method found.");
        }
    }


    fn read_file(filepath: &str) -> String {
        let mut input = String::new();
        let mut f = File::open(filepath).expect(format!("File not found: {}.", filepath).as_str());
        f.read_to_string(&mut input).expect("Error when reading input file.");
        return input;
    }

}
