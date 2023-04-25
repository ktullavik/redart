use std::collections::HashMap;
use evaluator::Object;
use parser::Node;
use parser::NodeType;


pub fn has_function(name: &str) -> bool {
    match name {
        "print" => true,
        _ => false
    }
}


pub fn call(name: &str, args: &Vec<Node>, symtable: &HashMap<String, Object>) -> Object {
    match name {
        "print" => {
            if args.len() < 1 {
                panic!("Argument expected by print().");
            }

            let paramlist = &args[0];
            let a1 = &paramlist.children[0];

            let t: &NodeType = &a1.nodetype;
//            let unpacked :Vec<String> = Vec::new();
//
//            for i in 0 .. args.len() {
//                unpacked.push();
//            }
            match t {
                NodeType::STRING(s) => {
                    println!("{}", s);
                }
                NodeType::NUM(s) => {
                    println!("{}", s);
                }
                NodeType::NAME(s) => {
                    let o = &symtable[s];
                    match o {
                        Object::NUM(v) => {
                            println!("{}", v);
                        }
                        Object::STRING(s) => {
                            println!("{}", s);
                        }
                        _ => {
                            println!("Illegal name arg for print: {}", t);
                            panic!("Illegal name argument for print")
                        }
                    }
                }
                // NodeType::TYPEDVAR(t, s) => {
                //     let o = &symtable[s];
                //     match o {
                //         Object::NUM(v) => {
                //             println!("{}", v);
                //         }
                //         Object::STRING(s) => {
                //             println!("{}", s);
                //         }
                //         _ => {
                //             println!("Illegal typedvar arg for print: {}", t);
                //             panic!("Illegal typedvar argument for print")
                //         }
                //     }
                // }
                _ => {
                    println!("Illegal arg for print: {}", t);
                    panic!("Illegal argument for print")
                }
            }

            // println!("{}", args[0]);
            // println!("{}", a);

        }

        _ => panic!("Unknown command: {}", name)
    }
    Object::VOID
}


//pub fn ls(dir: &Path) {
//
//    if dir.is_dir() {
//        for entry in fs::read_dir(dir).unwrap() {
//            let entry = entry.unwrap();
//            let path = entry.path();
//            if path.is_file() {
//                println!("{}", path.file_name().unwrap().to_str().unwrap());
//            }
//        }
//    } else {
//        eprintln!("Not a directory.");
//    }
//}
//
//
//pub fn ospace(cmd: &str, args: &Vec<Node>) {
//    use std::process::Command;
//
//
//    Command::new(cmd)
//        .args(args)
//        .spawn()
//        .expect("Command failed.");
//}

