use std::io::Read;
use rand::Rng;
use crate::state::State;
use crate::object::Object;
use crate::evalhelp::{argnodes_to_argobjs, call_function, MaybeRef};
use crate::NodeType;
use crate::node::Node;
use crate::heapobjs::InternalFile;
use crate::error::evalerror;


pub fn has_function(name: &str) -> bool {
    match name {
        "assert" |
        "print" |
        "__IO_FILE_CREATE" |
        "__IO_FILE_READ_AS_STRING" |
        "__LIST_ADD" |
        "__LIST_CLEAR" |
        "__LIST_REMOVEAT" |
        "__LIST_REMOVELAST" |
        "__LIST_REMOVERANGE" |
        "__LIST_SHUFFLE" |
        "__LIST_TOSTRING" |
        "__MATH_ACOS" |
        "__MATH_ASIN" |
        "__MATH_ATAN" |
        "__MATH_ATAN2" |
        "__MATH_COS" |
        "__MATH_EXP" |
        "__MATH_LOG" |
        "__MATH_MAX" |
        "__MATH_MIN" |
        "__MATH_POW" |
        "__MATH_SIN" |
        "__MATH_SQRT" |
        "__MATH_TAN" |
        "__MATH_NEXT_BOOL" |
        "__MATH_NEXT_DOUBLE" |
        "__MATH_NEXT_INT"
        => true,
        _ => false
    }
}


pub fn call(fnode: &Node, name: &str, state: &mut State) -> Object {

    let argnodes = &fnode.children[0].children;
    let args = argnodes_to_argobjs(argnodes, state);

    match name {

        "assert" => {
            if args.len() < 1 {
                panic!("Argument expected by assert().");
            }

            let a0 = &args[0];

            match a0 {
                Object::Bool(b) => {
                    if !b {

                        let mut msg = String::from("");

                        if args.len() > 1 {
                            // Dart accepts ints and bools and whatnot as second param.
                            msg = format!(": {}", &args[1]);
                        }
                        evalerror(
                            format!("Failed assertion{}", msg),
                            state,
                            fnode
                        )
                    }
                }
                _ => {
                    evalerror(
                        format!("Expected bool. Got: {}", args[0]),
                        state,
                        &argnodes[0]
                    );
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

                let fakenode = Node::new(
                    NodeType::MethodCall(
                        "toString".to_string(),
                        Box::new(argnodes[0].clone()),
                        state.filepath.clone(),
                        argnodes[0].find_node_position().0,
                        argnodes[0].find_node_position().1
                ));
                let m = c.get_method("toString", state, &fakenode);

                match &m {
                    Object::Function(_, _, _, _) => {
                        let tostring_args = Node::new(
                            NodeType::ArgList(
                                fnode.children[0].find_node_position().0,
                                fnode.children[0].find_node_position().1)
                        );
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

        "__LIST_REMOVEAT" => {
            if args.len() != 2 {
                panic!("Two arguments expected by __LIST_REMOVEAT.");
            }

            match &args[0] {

                Object::Reference(rk) => {
                    
                    match &args[1] {

                        Object::Int(i) => {

                            let ilist = state.objsys.get_list_mut(rk);

                            if *i < 0 {
                                panic!("Second argument of __LIST_REMOVEAT must be positive or 0. Got: {}", i)
                            }
                            if *i >= (ilist.els.len() as i64) {
                                panic!("Second argument of __LIST_REMOVEAT is out of bounds: {} (list length is {})", i, ilist.els.len())
                            }
                            return ilist.remove_at(*i as usize)
                        }
                        x => panic!("Unexpected second argument for __LIST_REMOVEAT: {}", x)
                    }
                }
                x => panic!("Unexepected first argument for __LIST_REMOVEAT: {}", x)
            }
        }

        "__LIST_REMOVELAST" => {
            if args.len() < 1 {
                panic!("Argument expected by __LIST_REMOVELAST.");
            }

            match args.get(0).unwrap() {

                Object::Reference(rk) => {
                    let ilist = state.objsys.get_list_mut(rk);
                    return ilist.remove_last();
                }
                x => panic!("Unexepected argument for __LIST_REMOVELAST: {}", x)
            }
        }

        "__LIST_REMOVERANGE" => {
            if args.len() != 3 {
                panic!("Expected 3 arguments for __LIST_REMOVERANGE. Got: {}", args.len());
            }

            if let Object::Reference(rk) = &args[0]  {
                let ilist = state.objsys.get_list_mut(rk);

                match &args[1] {
                    Object::Int(n1) => {

                        if *n1 < 0 {
                            panic!("Second arg of __LIST_REMOVERANGE must be positive or 0. Got: {}", n1)
                        }

                        match &args[2] {

                            Object::Int(n2) => {

                                if *n2 < 0 {
                                    panic!("Third arg of __LIST_REMOVERANGE must be positive or 0. Got: {}", n2)
                                }
                                if *n2 < *n1 {
                                    panic!("Third arg of __LIST_REMOVERANGE must be greater than first arg. {} {}", n2, n1)
                                }
                                if *n2 as usize > ilist.els.len() {
                                    panic!("Third arg of __LIST_REMOVERANGE must be less than list length.")
                                }

                                ilist.remove_range(*n1 as usize, *n2 as usize);
                            }

                            x => panic!("Unexpected second argument for __LIST_REMOVERANGE: {}", x)
                        }
                    }
                    x => panic!("Unexpected first argument for __LIST_REMOVERANGE: {}", x)
                }

            }
            return Object::Null;
        }

        "__LIST_SHUFFLE" => {
            if args.len() < 1 {
                panic!("Argument expected by __LIST_SHUFFLE.");
            }

            if let Object::Reference(rk) = &args[0]  {
                let ilist = state.objsys.get_list_mut(rk);
                ilist.shuffle();
                return Object::Null;
            }
            panic!("__LIST_SHUFFLE: ilist reference expected, got: {}", &args[0])
        }

        "__LIST_TOSTRING" => {
            if args.len() < 1 {
                panic!("Argument expected by __LIST_TOSTRING.");
            }

            if let Object::Reference(rk) = &args[0]  {
                let ilist = state.objsys.get_list(rk);
                return Object::String(ilist.to_string());
            }
            // XXX I think panic here
            return Object::String(format!("{}", args[0]));
        }

        "__MATH_ACOS" => {
            if args.len() != 1 {
                panic!("Expected 1 argument for __MATH_ACOS. Got: {}", args.len());
            }

            match &args[0] {
                Object::Double(x) => {
                    return Object::Double(x.acos());
                }
                x => panic!("Unexpected argument for __MATH_ACOS: {}", x)
            }
        }

        "__MATH_ASIN" => {
            if args.len() != 1 {
                panic!("Expected 1 argument for __MATH_ASIN. Got: {}", args.len());
            }

            match &args[0] {
                Object::Double(x) => {
                    return Object::Double(x.asin());
                }
                x => panic!("Unexpected argument for __MATH_ASIN: {}", x)
            }
        }

        "__MATH_ATAN" => {
            if args.len() != 1 {
                panic!("Expected 1 argument for __MATH_ATAN. Got: {}", args.len());
            }

            match &args[0] {
                Object::Double(x) => {
                    return Object::Double(x.atan());
                }
                x => panic!("Unexpected argument for __MATH_ATAN: {}", x)
            }
        }

        "__MATH_ATAN2" => {
            if args.len() != 2 {
                panic!("Expected 2 arguments for __MATH_ATAN2. Got: {}", args.len());
            }

            match &args[0] {
                Object::Double(x) => {
                    return Object::Double(x.atan2(*x));
                }
                x => panic!("Unexpected argument for __MATH_ATAN2: {}", x)
            }
        }

        "__MATH_COS" => {
            if args.len() != 1 {
                panic!("Expected 1 argument for __MATH_COS. Got: {}", args.len());
            }

            match &args[0] {
                Object::Double(x) => {
                    return Object::Double(x.cos());
                }
                x => panic!("Unexpected argument for __MATH_COS: {}", x)
            }
        }

        "__MATH_EXP" => {
            if args.len() != 1 {
                panic!("Expected 1 argument for __MATH_EXP. Got: {}", args.len());
            }

            match &args[0] {
                Object::Double(x) => {
                    return Object::Double(x.exp());
                }
                x => panic!("Unexpected argument for __MATH_EXP: {}", x)
            }
        }

        "__MATH_LOG" => {
            if args.len() != 1 {
                panic!("Expected 1 argument for __MATH_LOG. Got: {}", args.len());
            }

            match &args[0] {
                Object::Double(x) => {
                    return Object::Double(x.ln());
                }
                x => panic!("Unexpected argument for __MATH_LOG: {}", x)
            }
        }

        "__MATH_MAX" => {
            if args.len() != 1 {
                panic!("Expected 2 arguments for __MATH_MAX. Got: {}", args.len());
            }

            match &args[0] {
                Object::Double(x1) => {
                    match &args[1] {
                        Object::Double(x2) => {
                            if *x1 >= *x2 {
                                return Object::Double(x1.clone());
                            }
                            return Object::Double(x2.clone());
                        }
                        Object::Int(n2) => {
                            if *x1 >= (*n2 as f64) {
                                return Object::Double(x1.clone());
                            }
                            return Object::Int(n2.clone())
                        }
                        x => panic!("Unexpected second argument for __MATH_MAX: {}", x)
                    }
                }
                Object::Int(n1) => {
                    match &args[1] {
                        Object::Double(x2) => {
                            if (*n1 as f64) > *x2 {
                                return Object::Int(n1.clone());
                            }
                            return Object::Double(x2.clone());
                        }
                        Object::Int(n2) => {
                            if *n1 >= *n2 {
                                return Object::Int(n1.clone());
                            }
                            return Object::Int(n2.clone())
                        }
                        x => panic!("Unexpected second argument for __MATH_MAX: {}", x)
                    }
                }
                x => panic!("Unexpected first argument for __MATH_MAX: {}", x)
            }
        }

        "__MATH_MIN" => {
            if args.len() != 1 {
                panic!("Expected 2 arguments for __MATH_MIN. Got: {}", args.len());
            }

            match &args[0] {
                Object::Double(x1) => {
                    match &args[1] {
                        Object::Double(x2) => {
                            if *x1 <= *x2 {
                                return Object::Double(x1.clone());
                            }
                            return Object::Double(x2.clone());
                        }
                        Object::Int(n2) => {
                            if *x1 <= (*n2 as f64) {
                                return Object::Double(x1.clone());
                            }
                            return Object::Int(n2.clone())
                        }
                        x => panic!("Unexpected second argument for __MATH_MIN: {}", x)
                    }
                }
                Object::Int(n1) => {
                    match &args[1] {
                        Object::Double(x2) => {
                            if (*n1 as f64) < *x2 {
                                return Object::Int(n1.clone());
                            }
                            return Object::Double(x2.clone());
                        }
                        Object::Int(n2) => {
                            if *n1 <= *n2 {
                                return Object::Int(n1.clone());
                            }
                            return Object::Int(n2.clone())
                        }
                        x => panic!("Unexpected second argument for __MATH_MIN: {}", x)
                    }
                }
                x => panic!("Unexpected first argument for __MATH_MIN: {}", x)
            }
        }

        "__MATH_POW" => {
            if args.len() != 2 {
                panic!("Expected 2 arguments for __MATH_POW. Got: {}", args.len());
            }

            match &args[0] {

                Object::Double(x1) => {

                    match &args[1] {
                        Object::Double(x2) => {
                            return Object::Double(x1.powf(*x2));
                        }
                        Object::Int(n2) => {
                            return Object::Double(x1.powi(*n2 as i32));
                        }
                        x => panic!("Unexpected second argument for __MATH_POW: {}", x)
                    }
                }
                Object::Int(n1) => {

                    match &args[1] {
                        Object::Double(x2) => {
                            return Object::Double((*n1 as f64).powf(*x2));
                        }
                        Object::Int(n2) => {
                            return Object::Double((*n1 as f64).powi(*n2 as i32));
                        }
                        x => panic!("Unexpected second argument for __MATH_POW: {}", x)
                    }
                    
                }
                x => panic!("Unexpected first argument for __MATH_POW: {}", x)
            }
        }

        "__MATH_SIN" => {
            if args.len() != 1 {
                panic!("Expected 1 argument for __MATH_SIN. Got: {}", args.len());
            }

            match &args[0] {
                Object::Double(x) => {
                    return Object::Double(x.sin());
                }
                Object::Int(n) => {
                    return Object::Double((*n as f64).sin())
                }
                x => panic!("Unexpected argument for __MATH_SIN: {}", x)
            }
        }

        "__MATH_SQRT" => {
            if args.len() != 1 {
                panic!("Expected 1 argument for __MATH_SQRT. Got: {}", args.len());
            }

            match &args[0] {
                Object::Int(n) => {
                    return Object::Double((*n as f64).sqrt());
                }
                Object::Double(x) => {
                    return Object::Double(x.sqrt());
                }
                x => panic!("Unexpected argument for __MATH_SQRT: {}", x)
            }
        }

        "__MATH_TAN" => {
            if args.len() != 1 {
                panic!("Expected 1 argument for __MATH_TAN. Got: {}", args.len());
            }

            match &args[0] {
                Object::Int(n) => {
                    return Object::Double((*n as f64).tan());
                }
                Object::Double(x) => {
                    return Object::Double(x.tan());
                }
                x => panic!("Unexpected argument for __MATH_TAN: {}", x)
            }
        }

        "__MATH_NEXT_BOOL" => {
            if args.len() != 0 {
                panic!("Expected 0 arguments for __MATH_NEXT_BOOL. Got: {}", args.len());
            }
            
            let mut rng = rand::thread_rng();
            let r = rng.gen_range(0 .. 2);
            return Object::Bool(r == 1);
        }

        "__MATH_NEXT_DOUBLE" => {
            if args.len() != 0 {
                panic!("Expected 0 arguments for __MATH_NEXT_DOUBLE. Got: {}", args.len());
            }

            let mut rng = rand::thread_rng();
            let r = rng.gen::<f64>();
            return Object::Double(r);
        }

        "__MATH_NEXT_INT" => {
            if args.len() != 1 {
                panic!("Expected 1 argument for __MATH_NEXT_INT. Got: {}", args.len());
            }

            match &args[0] {

                Object::Int(n) => {
                    let mut rng = rand::thread_rng();
                    let r = rng.gen_range(0 .. *n);
                    return Object::Int(r);
                }
                x => evalerror(format!("Expected int. Got {}", x), state, &argnodes[0])
            }
        }


        _ => panic!("Unknown command: {}", name)
    }
    Object::Null
}

