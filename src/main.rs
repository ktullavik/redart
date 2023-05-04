extern crate rustyline;

use std::io::prelude::*;
use std::env;
use std::fs::File;

mod parser;
mod lexer;
mod evaluator;
mod builtin;
mod utils;
mod stack;

use evaluator::Object;
use stack::Stack;


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
            println!("\n{:?}\n", tokens);
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

        let mut store = Stack::new();

        evaluator::preval(&tree, &mut store);

        let mainfunc = &store.get("main").clone();

        match mainfunc {
            Object::FUNCTION(_, n, _) => {
                utils::dprint(" ");
                utils::dprint("EVALUATE");
                utils::dprint(" ");
                evaluator::eval(n, &mut store);
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
            "9" => {
                testfile = "9.evaled_argpass.dart";
            }
            "10" => {
                testfile = "10.arithmetic.dart";
            }
            "11" => {
                testfile = "11.conditional.dart";
            }
            "12" => {
                testfile = "12.conditional2.dart";
            }
            "13" => {
                testfile = "13.conditional3.dart";
            }
            "14" => {
                testfile = "14.conditional4.dart";
            }
            "15" => {
                testfile = "15.conditional5.dart";
            }
            "16" => {
                testfile = "16.mutate.dart";
            }
            "17" => {
                testfile = "17.mutate_self.dart";
            }
            "18" => {
                testfile = "18.post_increment.dart";
            }
            "19" => {
                testfile = "19.post_decrement.dart";
            }
            "20" => {
                testfile = "20.pre_increment.dart";
            }
            "21" => {
                testfile = "21.pre_decrement.dart";
            }
            "22" => {
                testfile = "22.returnvalue.dart";
            }
            "23" => {
                testfile = "23.logical_or.dart";
            }
            "24" => {
                testfile = "24.logical_and.dart";
            }
            "25" => {
                testfile = "25.logical_expr.dart";
            }
            "26" => {
                testfile = "26.less_than.dart";
            }
            "27" => {
                testfile = "27.greater_than.dart";
            }
            "28" => {
                testfile = "28.less_or_equal.dart";
            }
            "29" => {
                testfile = "29.greater_or_equal.dart";
            }
            "34" => {
                testfile = "34.list_replace.dart";
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
