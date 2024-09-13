use std::process;
use std::io::Read;
use state::State;
use object::Object;
use evaluator::call_function;
use evaluator::MaybeRef;
use NodeType;
use node::Node;
use crate::heapobjs::InternalFile;



pub fn has_function(name: &str) -> bool {
    match name {
        "assert" |
        "print" |
        "__IO_FILE_CREATE" |
        "__IO_FILE_READ_AS_STRING" |
        "__LIST_ADD" |
        "__LIST_CLEAR" |
        "__LIST_REMOVELAST" |
        "__LIST_TOSTRING"
        => true,
        _ => false
    }
}


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
                let c = state.objsys.get_class(inst.classname.as_str());
                let m = c.get_method("toString");

                match &m {
                    Object::Function(_, _, _, _) => {
                        let tostring_args = Node::new(NodeType::ArgList);
                        let strobj = call_function(MaybeRef::Ref(inst.id.clone()), &m, &tostring_args, state);
                        println!("{}", strobj);
                    }
                    x => panic!("Error: toString was not a function: {}", x)
                }
            }
            else {
                println!("{}", &args[0]);
            }
        }

        "__IO_FILE_CREATE" => {
            if args.len() != 2 {
                panic!("Wrong number of arguments for __IO_FILE_CREATE.");
            }

            match &args[0] {

                Object::Reference(rk) => {

                    match &args[1] {

                        Object::String(s) => {
                            let ifile = InternalFile::new(s.to_string());
                            let internal_rk = state.objsys.register_file(ifile);

                            let dfile = state.objsys.get_instance_mut(rk);
                            dfile.set_field(String::from("_internalFile"), internal_rk);
                            return Object::Reference(rk.clone());
                        }
                        _ => panic!("Unexpected second arg for __IO_FILE_CREATE")
                    }
                }
                _ => panic!("Unexpected first arg for __IO_FILE_CREATE")
            }
        }

        "__IO_FILE_READ_AS_STRING" => {
            if args.len() < 1 {
                panic!("Argument expected by __IO_FILE_READ_AS_STRING.");
            }

            match &args[0] {

                Object::Reference(rk) => {
                    let ifile = state.objsys.get_file_mut(rk);
                    let mut content = String::new();
                    ifile.file.read_to_string(&mut content).unwrap();
                    return Object::String(content);
                }
                _ => panic!("Unexpected argument for __IO_READ_AS_STRING")
            }
        }

        "__LIST_ADD" => {
            if args.len() != 2 {
                panic!("Two arguments expected by __LIST_ADD.");
            }

            match args.get(0).unwrap() {

                Object::Reference(rk) => {
                    let ilist = state.objsys.get_list_mut(rk);
                    ilist.add(args[1].clone());
                }
                x => panic!("Unexpected argument for __LIST_ADD: {}", x)
            }
            return Object::Null;
        }

        "__LIST_CLEAR" => {
            if args.len() != 1 {
                panic!("Argument expected by __LIST_CLEAR.");
            }

            match args.get(0).unwrap() {

                Object::Reference(rk) => {
                    let ilist = state.objsys.get_list_mut(rk);
                    ilist.set_elements(Vec::new());
                    // state.liststore.set_elements(rk.clone(), Vec::new())
                }
                x => panic!("Unexepcted argument for __LIST_CLEAR: {}", x)
            }
            return Object::Null;
        }

        "__LIST_REMOVELAST" => {
            if args.len() < 1 {
                panic!("Argument expected by __LIST_REMOVELAST");
            }

            match args.get(0).unwrap() {

                Object::Reference(rk) => {
                    let ilist = state.objsys.get_list_mut(rk);
                    return ilist.remove_last();
                }
                x => panic!("Unexepected argument for __LIST_REMOVELAST: {}", x)
            }
        }

        "__LIST_TOSTRING" => {
            if args.len() < 1 {
                panic!("Argument expected by __LIST_TOSTRING");
            }

            if let Object::Reference(rk) = &args[0]  {
                let ilist = state.objsys.get_list(rk);
                return Object::String(ilist.to_string());
            }
            return Object::String(format!("{}", args[0]));
        }

        _ => panic!("Unknown command: {}", name)
    }
    Object::Null
}

