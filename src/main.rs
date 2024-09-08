extern crate rustyline;
extern crate nuid;
extern crate queues;

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
mod state;
mod reader;
mod dirs;

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::collections::HashMap;
use state::State;
use dirs::Dirs;
use node::NodeType;


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Argument expected.");
        return;
    }

    let dirs = Dirs::new();
    let mut state = State::new();

    let a1 = &args[1];

    match a1.as_str() {
        "lex" => {
            if args.len() < 3 {
                println!("Please specify file ...");
                return;
            }
            state.filepath = String::from(&args[2]);
            do_task("lex",args[2].clone(), &mut state, &dirs);
        }
        "parse" => {
            if args.len() < 3 {
                println!("Please specify file...");
                return;
            }
            state.filepath = String::from(&args[2]);
            do_task("parse",args[2].clone(), &mut state, &dirs);
        }
        "test" => {
            if args.len() < 3 {
                println!("Running all tests:");
                for s in testlist::TESTS {
                    println!("____________________________________________________");
                    println!("Running test: {}", s);
                    println!("----------------------------------------------------");
                    let path = format!("{}/{}", dirs.testdir(), s);
                    state.filepath = String::from(path.as_str());
                    do_task("eval", String::from(path.as_str()), &mut state, &dirs);
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

            let filepath = testlist::get_filepath(nextarg.clone(), &dirs);
            state.filepath = filepath.clone();
            do_task(task, filepath, &mut state, &dirs);
        }
        "testfail" => {
            if args.len() < 3 {
                println!("Running all fail tests:");
                for s in testlist::FAILTESTS {
                    let path = format!("{}/{}", dirs.failtestdir(), s);
                    state.filepath = String::from(path.as_str());
                    do_task("eval", String::from(path.as_str()), &mut state, &dirs);
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

            let filepath = testlist::get_failfilepath(nextarg.clone(), &dirs);
            state.filepath = filepath.clone();
            do_task(task, filepath, &mut state, &dirs);
        }
        _ => {
            println!("Illegal argument: {}", a1);
        }
    }
}


fn do_task(action: &str, filepath: String, state: &mut State, dirs: &Dirs) {


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
            // let mut globals: Vec<Node> = Vec::new();
            parser::parse(&mut tokens, state);

            for f in &state.globals {
                println!("\n{}\n", f);
            }
        }
        "eval" => {
            evaluate(filepath, state, dirs);
        }
        x => {
            println!("Unknown action: {}", x);
        }
    }
}


fn filecurse(
    basepath: String,
    filepath: String,
    memo: &mut HashMap<String, (usize, usize)>,
    state: &mut State,
    dirs: &Dirs) {

    let fullpath =
        if filepath.starts_with("dart:") {
            // Built-in library
            let libname = filepath.clone().split_off(5);
            format!("{}/core/{}.dart", dirs.libdir(), libname)
        }
        else {
            // User provided library.
            format!("{}/{}", basepath, filepath)
        };

    let input = read_file(fullpath.as_str());
    let mut tokens = lexer::lex(&input);

    state.filepath = filepath.clone();

    let oldlen = state.globals.len();

    let imports = parser::parse(&mut tokens, state);

    memo.insert(filepath.clone(), (oldlen, state.globals.len()));

    let mut looktable: HashMap<String, usize> = HashMap::new();

    for i in oldlen..state.globals.len() {
        let f = &state.globals[i];

        match &f.nodetype {
            NodeType::FunDef(funcname, _) => {
                looktable.insert(funcname.clone(), i);

            }
            NodeType::Constructor(name, _) => {
                looktable.insert(name.clone(), i);
            }
            _ => {
                panic!("Unexpected node type in globals");
            }
        }
    }

    for s in imports {

        if memo.contains_key(&s) {
            continue;
        }
        filecurse(basepath.clone(), s.clone(), memo, state, dirs);

        // For every import, merge its functions into this files looktable.

        let (childstart, childend) = memo[&s];

        for i in childstart..childend {

            let f = &state.globals[i];
            match &f.nodetype {
                NodeType::FunDef(funcname, _) => {
                    looktable.insert(funcname.clone(), i);
                }
                NodeType::Constructor(name, _) => {
                    looktable.insert(name.clone(), i);
                }
                _ => {
                    panic!("Unexpected node type in globals");
                }
            }
        }
    }
    state.looktables.insert(filepath.clone(), looktable);
}


fn evaluate(filepath: String, state: &mut State, dirs: &Dirs) {

    let mut memo: HashMap<String, (usize, usize)> = HashMap::new();

    let mut parts: Vec<&str> = filepath.split('/').collect();

    let filename = parts.remove(parts.len() - 1);
    let basepath = parts.join("/");

    filecurse(basepath.clone(), String::from(filename), &mut memo, state, dirs);

    let toptable = &state.looktables[filename];
    if !toptable.contains_key("main") {
        // As Dart.
        panic!("Error: No 'main' method found.");
    }

    let mainindex: &usize = toptable.get("main").unwrap();
    let mainfunc = &state.globals[*mainindex].clone();
    state.filepath = filename.to_string();

    match &mainfunc.nodetype {
        NodeType::FunDef(_, _) => {
            utils::dprint(" ");
            utils::dprint("EVALUATE");
            utils::dprint(" ");

            let mainbody = &mainfunc.children[1];

            state.stack.push_call();
            evaluator::eval(mainbody, state, true);
            state.stack.pop_call();
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
