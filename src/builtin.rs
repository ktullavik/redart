use state::State;
use object::Object;
use std::process;
use std::fs::File;
use std::io::Read;


pub fn has_function(name: &str) -> bool {
    match name {
        "assert" |
        "__IO_FILE_READ_AS_STRING" |
        "print" => true,
        _ => false
    }
}


pub fn call(name: &str, args: &Vec<Object>, state: &State) -> Object {
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

                        let filepath = &state.filepath;
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

                    let filepath = &state.filepath;
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

        "__IO_FILE_READ_AS_STRING" => {
            if args.len() < 1 {
                panic!("Argument expected by __IO_FILE_READ_AS_STRING.");
            }

            match &args[0] {

                Object::String(s) => {
                    let mut file = File::open(s).unwrap();
                    let mut content = String::new();
                    file.read_to_string(&mut content).unwrap();
                    return Object::String(content);
                }

                _ => {
                    panic!("Unexpected argument for __IO_READ_AS_STRING")
                }
            }
        }

        "print" => {
            if args.len() < 1 {
                panic!("Argument expected by print.");
            }
            println!("{}", &args[0]);
        }

        _ => panic!("Unknown command: {}", name)
    }
    Object::Null
}

