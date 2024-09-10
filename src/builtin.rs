use state::State;
use object::Object;
use std::process;
use std::fs::File;
use std::io::Read;
use evaluator::eval;


pub fn has_function(name: &str) -> bool {
    match name {
        "assert" |
        "print" |
        "__IO_FILE_READ_AS_STRING" |
        "__LIST_ADD" |
        "__LIST_CLEAR" |
        "__LIST_TOSTRING"
        => true,
        _ => false
    }
}


// Warning: args can not only be changed internally. This function takes
// ownership of the arg objects by removing them from the list.
pub fn call(name: &str, args: &mut Vec<Object>, state: &mut State) -> Object {
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

        "print" => {
            if args.len() < 1 {
                panic!("Argument expected by print.");
            }
            if let Object::Reference(k) = &args[0] {
                let inst = state.objsys.get_instance(k);
                
                let c = state.objsys.get_class(&inst.classname);
                let m = c.get_method("toString");

                match m {

                    Object::Function(_, filename, node, _) => {

                        state.stack.push_call();
                        let oldfilename = state.filepath.clone();
                        state.filepath = filename.clone();
                        let oldthis = state.objsys.get_this();
                        state.objsys.set_this(inst.id.clone());

                        eval(&node, state, true);

                        state.objsys.set_this(oldthis);
                        state.filepath = oldfilename;
                        state.stack.pop_call();
                    }

                    x => panic!("Error: toString was not a function: {}", x)
                }

            }
            else {
                println!("{}", &args[0]);
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

        "__LIST_ADD" => {
            if args.len() != 2 {
                panic!("Two arguments expected by __LIST_ADD.");
            }

            let new_el = args[1].clone();

            match args.get_mut(0).unwrap() {

                Object::__InternalList(vec) => {
                    vec.push(new_el);
                }

                x => panic!("Unexpected argument for __LIST_ADD: {}", x)
            }

            return args.remove(0);
        }

        "__LIST_CLEAR" => {
            if args.len() != 1 {
                panic!("Argument expected by __LIST_CLEAR.");
            }

            match args.get_mut(0).unwrap() {

                Object::__InternalList(vec) => {
                    vec.clear();
                }

                x => panic!("Unexepcted argument for __LIST_CLEAR: {}", x)
            }
            return args.remove(0);
        }

        "__LIST_TOSTRING" => {
            if args.len() < 1 {
                panic!("Argument expected by __LIST_TOSTRING");
            }

            println!("{}", args[0]);
            
        }


        _ => panic!("Unknown command: {}", name)
    }
    Object::Null
}

