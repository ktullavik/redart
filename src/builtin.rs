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
    Object::Void
}

