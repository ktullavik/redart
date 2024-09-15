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
mod heapobjs;

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::time::Instant;
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
    let a1 = &args[1];

    match a1.as_str() {
        "lex" => {
            if args.len() < 3 {
                println!("Error: File argument expected.");
                return;
            }
            do_task("lex", args[2].clone(), &dirs);
        }
        "parse" => {
            if args.len() < 3 {
                println!("Error: File argument expected.");
                return;
            }
            do_task("parse", args[2].clone(), &dirs);
        }
        "test" => {
            if args.len() < 3 {
                println!("Running all tests:");
                let start = Instant::now();
                for s in testlist::TESTS {
                    println!("____________________________________________________");
                    println!("Running test: {}", s);
                    println!("----------------------------------------------------");
                    let path = format!("{}/{}", dirs.testdir(), s);
                    do_task("eval", String::from(path.as_str()), &dirs);
                }
                let end = Instant::now();
                println!("____________________________________________________");
                println!("Ran {} tests in {}ms.", testlist::TESTS.len(), (end - start).as_millis());
                return;
            }

            let mut task = "eval";
            let nextarg: &String;

            match args[2].as_str() {
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
            do_task(task, filepath, &dirs);
        }
        "testfail" => {
            if args.len() < 3 {
                println!("Error: Argument expected.");
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
            do_task(task, filepath, &dirs);
        }
        _ => {
            println!("Illegal argument: {}", a1);
        }
    }
}


fn do_task(action: &str, filepath: String, dirs: &Dirs) {

    let mut state = State::new();
    state.filepath = filepath.clone();

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
            parser::parse(&mut tokens, &mut state);

            for f in &state.globals {
                println!("\n{}\n", f);
            }
        }
        "eval" => {
            evaluate(filepath, &mut state, dirs);
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
        if filepath.starts_with("auto:") {
            // Auto included library.
            let libname = filepath.clone().split_off(5);
            format!("{}/auto/{}", dirs.libdir(), libname)
        }
        else if filepath.starts_with("dart:") {
            // Built-in library.
            let libname = filepath.clone().split_off(5);
            format!("{}/core/{}.dart", dirs.libdir(), libname)
        }
        else {
            // User provided library.
            format!("{}/{}", basepath, filepath)
        };

    state.filepath = filepath.clone();
    let input = read_file(fullpath.as_str());
    let mut tokens = lexer::lex(&input);

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
