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


pub fn call(name: &str, args: &Vec<Object>, symtable: &HashMap<String, Object>) -> Object {
    match name {
        "print" => {
            if args.len() < 1 {
                panic!("Argument expected by print().");
            }

            let a0= &args[0];

            match a0 {

                Object::STRING(s) => {
                    println!("{}", s);
                }
                Object::NUM(x) => {
                    println!("{}", x);
                }
                _ => {
                    // println!("Illegal arg for print: {}", a0);
                    panic!("Illegal argument for print")
                }
            }
        }

        _ => panic!("Unknown command: {}", name)
    }
    Object::VOID
}

