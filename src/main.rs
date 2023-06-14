extern crate rustyline;
extern crate nuid;
extern crate queues;
extern crate core;

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

use object::Object;
use stack::Stack;
use objsys::ClassMap;
use objsys::InstanceMap;
use std::collections::HashMap;


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
            do_task("lex", read_inputfile(&args[2]));
        }
        "parse" => {
            if args.len() < 3 {
                println!("Please specify file...");
                return;
            }
            do_task("parse", read_inputfile(&args[2]));
        }
        "test" => {
            if args.len() < 3 {
                println!("Running all tests:");
                for testindex in 1 .. 49 {
                    println!("Running test: {}", testindex);
                    do_task("eval", read_testfile(testindex.to_string().as_str()));
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

            do_task(task, read_testfile(nextarg));
        }
        _ => {
            println!("Illegal argument: {}", a1);
        }
    }


    fn do_task(action: &str, input: String) {

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
                let tree = parser::parse(&tokens).unwrap();
                println!("\n{}\n", tree);
            }
            "eval" => {
                evaluate(&input);
            }
            x => {
                println!("Unknown action: {}", x);
            }

        }
    }


    fn evaluate(input: &String) {

        let tokens = lexer::lex(&input);
        let tree = parser::parse(&tokens).unwrap();

        let mut store = Stack::new();
        let mut classlist = ClassMap::new();
        let mut instlist = InstanceMap::new();
        let mut globals : HashMap<String, Object> = HashMap::new();

        evaluator::preval(&tree, &mut globals, &mut store, &mut classlist, &mut instlist);

        if globals.contains_key("main") {
            let mainfunc = &globals.get("main").unwrap().clone();

            match mainfunc {
                Object::Function(_, n, _) => {
                    utils::dprint(" ");
                    utils::dprint("EVALUATE");
                    utils::dprint(" ");

                    store.push_call();
                    evaluator::eval(n, &mut globals, &mut store, &mut classlist, &mut instlist);
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


    fn read_inputfile(filename: &str) -> String {
        let mut input = String::new();
        let mut f = File::open(format!("{}{}", TESTPATH, filename)).expect("File not found!");
        f.read_to_string(&mut input).expect("Error when reading input file.");
        return input;
    }


    fn read_testfile(s: &str) -> String {

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
            "42" => "42.bitand.dart",
            "43" => "43.bitor.dart",
            "44" => "44.bitxor.dart",
            "45" => "45.left_associative_sum.dart",
            "46" => "46.hard_expression.dart",
            "47" => "47.left_associative_product.dart",
            "48" => "48.string_concat.dart",

            "50" => "50.method_reading_field.dart",
            "51" => "51.constructor_setting_field.dart",
            "52" => "52.constructor_setting_field_from_arg.dart",
            "53" => "53.method_postincrementing_field.dart",
            "54" => "54.method_postdecrementing_field.dart",
            "55" => "55.method_preincrementing_field.dart",
            "56" => "56.method_predecrementing_field.dart",

            "60" => "60.cross_function_leak.dart",
            "61" => "61.double_declaration.dart",
            "62" => "62.forgotten_paramlist.dart",
            "63" => "63.lexical_scope.dart",
            "64" => "64.plus_is_not_prefix.dart",

            "70" => "70.string_interpolation.dart",
            "71" => "71.string_interpolation2.dart",
            "72" => "72.string_interpolation3.dart",
            "73" => "73.string_interpolation4.dart",

            "80" => "80.semicolon_king.dart",

            "100" => "100.list_replace.dart",

            s => s
        };

        let mut input = String::new();
        let mut f = File::open(format!("{}{}", TESTPATH, filename)).expect(format!("File not found: {}.", filename).as_str());
        f.read_to_string(&mut input).expect("Error when reading input file.");
        return input;
    }
}
