use evaluator::Object;


pub fn has_function(name: &str) -> bool {
    match name {
        "print" => true,
        _ => false
    }
}


pub fn call(name: &str, args: &Vec<Object>) -> Object {
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
                Object::INT(n) => {
                    println!("{}", n);
                }
                Object::DOUBLE(x) => {
                    println!("{}", x);
                }
                Object::BOOL(b) => {
                    println!("{}", b);
                }
                _ => {
                    panic!("Illegal argument for print: {:?}", a0)
                }
            }
        }

        _ => panic!("Unknown command: {}", name)
    }
    Object::VOID
}

