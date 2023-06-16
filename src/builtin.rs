use context::Ctx;
use std::process;
use object::Object;


pub fn has_function(name: &str) -> bool {
    match name {
        "assert" |
        "print" => true,
        _ => false
    }
}


pub fn call(name: &str, args: &Vec<Object>, ctx: &Ctx) -> Object {
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
                            // Dart accepts ints and bools and whatnot as second param.
                            msg = format!("{}", &args[1]);
                        }

                        let filepath = &ctx.filepath;
                        let linenum = 0;
                        let sympos = 0;
                        println!("'file://{}': Failed assertion: line {} pos {}: argument: {}", filepath, linenum, sympos, msg);
                        // TODO: Dart manages to get the variable name in here.
                        // println!("'file://{}': Failed assertion: line {} pos {}: 'argname': {}.", filename, linenum, sympos, msg);

                        process::exit(1);
                    }
                }
                _ => {
                    // Should be caught generally, by type system. For now, msg like dart.
                    // TODO, get line number, symbol number and object type.

                    let filepath = &ctx.filepath;
                    let linenum = 0;
                    let sympos = 0;
                    let objtype = "unknown";

                    println!(
                        "{}:{}:{}: Error: A value of type '{}' can't be assigned to a variable of type 'bool'.",
                        filepath,
                        linenum,
                        sympos,
                        objtype
                    );
                    process::exit(1);
                }
            }
        }

        "print" => {
            if args.len() < 1 {
                panic!("Argument expected by print().");
            }
            println!("{}", &args[0]);
        }

        _ => panic!("Unknown command: {}", name)
    }
    Object::Null
}

