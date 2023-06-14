use std::process;
use object::Object;


pub fn has_function(name: &str) -> bool {
    match name {
        "assert" => true,
        "print" => true,
        _ => false
    }
}


pub fn call(name: &str, args: &Vec<Object>) -> Object {
    match name {

        "assert" => {
            if args.len() < 1 {
                panic!("Argument expected by assert().");
            }

            let a0 = &args[0];

            match a0 {
                Object::Bool(b) => {
                    if !b {

                        let mut msg = String::from("is not true.");

                        if args.len() > 1 {
                            msg = format!("{}", &args[1]);
                        }

                        let linenum = 0;
                        let sympos = 0;
                        let filename = "/home/kt/devel/project/filename.dart";

                        println!("'file://{}': Failed assertion: line {} pos {}: argument: {}", filename, linenum, sympos, msg);
                        // TODO: Dart manages to get the variable name in here.
                        // println!("'file://{}': Failed assertion: line {} pos {}: 'argname': {}.", filename, linenum, sympos, msg);

                        process::exit(1);
                    }
                }
                _ => {
                    // Should be caught generally, by type system. For now, msg like dart.
                    // TODO, get filename, line number and symbol number.
                    println!(
                        "{}:{}:{}: Error: A value of type 'int' can't be assigned to a variable of type 'bool'.",
                        "filename",
                        0,
                        0
                    );
                    process::exit(1);
                }
            }
        }

        "print" => {
            if args.len() < 1 {
                panic!("Argument expected by print().");
            }

            let a0= &args[0];

            match a0 {

                Object::String(s) => {
                    println!("{}", s);
                }
                Object::Int(n) => {
                    println!("{}", n);
                }
                Object::Double(x) => {
                    println!("{}", x);
                }
                Object::Bool(b) => {
                    println!("{}", b);
                }
                _ => {
                    panic!("Illegal argument for print: {:?}", a0)
                }
            }
        }

        _ => panic!("Unknown command: {}", name)
    }
    Object::Null
}

