extern crate rustyline;
extern crate nuid;

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

use evaluator::Object;
use stack::Stack;
use objsys::ClassList;
use objsys::InstanceList;


static TESTPATH: &str = "/usr/home/kt/devel/redart/test/";



fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Argument expected.");
    }

    let a1 = &args[1];

    match a1.as_str() {
        "lex" => {
            if args.len() < 3 {
                println!("Please specify file ...");
                return;
            }

            let input = read_inputfile(&args[2]);
            let tokens = lexer::lex(&input);
            for t in tokens {
                print!("{} ", t);
            }
            println!();
        }
        "parse" => {
            if args.len() < 3 {
                println!("Please specify file...");
                return;
            }

            let input = read_inputfile(&args[2]);
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

            let input = read_inputfile(nextarg);


            if action == "lex" {
                let tokens = lexer::lex(&input);
                for t in tokens {
                    print!("{} ", t);
                }
                println!();
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

        let mut store = Stack::new();
        let mut classlist = ClassList::new();
        let mut instlist = InstanceList::new();

        evaluator::preval(&tree, &mut store, &mut classlist, &mut instlist);


        if store.has("main") {
            let mainfunc = &store.get("main").clone();

            match mainfunc {
                Object::Function(_, n, _) => {
                    utils::dprint(" ");
                    utils::dprint("EVALUATE");
                    utils::dprint(" ");
                    evaluator::eval(n, &mut store, &mut classlist, &mut instlist);
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


    fn read_inputfile(s: &str) -> String {

        let filename = match s {
            "1" => "1.hello.dart",
            "2" => "2.variable.dart",
            "3" => "3.addition.dart",
            "4" => "4.subtraction.dart",
            "5" => "5.multiplication.dart",
            "6" => "6.division.dart",
            "7" => "7.funcall.dart",
            "8" => "8.argpass.dart",
            "9" => "9.evaled_argpass.dart",
            "10" => "10.arithmetic.dart",
            "11" => "11.conditional.dart",
            "12" => "12.conditional2.dart",
            "13" => "13.conditional3.dart",
            "14" => "14.conditional4.dart",
            "15" => "15.conditional5.dart",
            "16" => "16.mutate.dart",
            "17" => "17.mutate_self.dart",
            "18" => "18.post_increment.dart",
            "19" => "19.post_decrement.dart",
            "20" => "20.pre_increment.dart",
            "21" => "21.pre_decrement.dart",
            "22" => "22.returnvalue.dart",
            "23" => "23.logical_or.dart",
            "24" => "24.logical_and.dart",
            "25" => "25.logical_expr.dart",
            "26" => "26.less_than.dart",
            "27" => "27.greater_than.dart",
            "28" => "28.less_or_equal.dart",
            "29" => "29.greater_or_equal.dart",
            "30" => "30.equality.dart",
            "31" => "31.equality2.dart",
            "32" => "32.equality3.dart",
            "33" => "33.recursion.dart",
            "34" => "34.unary_minus.dart",
            "35" => "35.arg_expression.dart",
            "36" => "36.arg_expression2.dart",
            "37" => "37.arg_expression3.dart",
            "38" => "38.arg_expression4.dart",
            "39" => "39.not.dart",
            "40" => "40.fibonacci.dart",
            "41" => "41.difficult_return.dart",

            "50" => "50.classdef.dart",

            "70" => "70.list_replace.dart",

            s => s
        };

        let mut input = String::new();
        let mut f = File::open(format!("{}{}", TESTPATH, filename)).expect("File not found!");
        f.read_to_string(&mut input).expect("Error when reading input file.");
        return input;
    }

}
