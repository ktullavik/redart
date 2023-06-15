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

use object::Object;
use stack::Stack;
use objsys::ClassMap;
use objsys::InstanceMap;
use std::collections::HashMap;


static TESTPATH: &str = "/usr/home/kt/devel/redart/test";
static FAILTESTPATH: &str = "/usr/home/kt/devel/redart/testfail";



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
                    let filepath = get_testfilepath(testindex.to_string());
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

            let filepath = get_testfilepath(nextarg.clone());
            ctx.insert("filepath", filepath.clone());
            do_task(task, read_file(filepath.as_str()), &ctx);
        }
        "testfail" => {
            if args.len() < 3 {
                println!("Running all fail tests:");
                for testindex in 1 .. 5 {
                    println!("Running test: {}", testindex);
                    let filepath = get_failtestfilepath(testindex.to_string());
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

            let filepath = get_failtestfilepath(nextarg.clone());
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


    fn get_failtestfilepath(s: String) -> String {

        let filename = match s.as_str() {
            "1" => "1.cross_function_leak.dart",
            "2" => "2.double_declaration.dart",
            "3" => "3.forgotten_paramlist.dart",
            "4" => "4.plus_is_not_prefix.dart",
            x => panic!("Unknown failtest: {}", x)
        };
        return format!("{}/{}", FAILTESTPATH, filename);
    }


    fn get_testfilepath(s: String) -> String {

        let filename = match s.as_str() {
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
            "49" => "49.lexical_scope.dart",
            "50" => "50.method_reading_field.dart",
            "51" => "51.constructor_setting_field.dart",
            "52" => "52.constructor_setting_field_from_arg.dart",
            "53" => "53.method_postincrementing_field.dart",
            "54" => "54.method_postdecrementing_field.dart",
            "55" => "55.method_preincrementing_field.dart",
            "56" => "56.method_predecrementing_field.dart",
            "57" => "57.string_interpolation.dart",
            "58" => "58.string_interpolation2.dart",
            "59" => "59.string_interpolation3.dart",
            "60" => "60.string_interpolation4.dart",
            "61" => "61.semicolon_king.dart",

            s => s
        };

        return format!("{}/{}", TESTPATH, filename);
    }
}
