use parser::Node;
use parser::NodeType;
use builtin;
use utils::dprint;
use stack::Stack;


#[derive(Debug)]
#[derive(Clone)]
pub enum Object {
    INT(i64),
    DOUBLE(f64),
    BOOL(bool),
    STRING(String),
    FUNCTION(String, Node, Vec<String>),
    VOID
}


pub fn preval(node: &Node, store: &mut Stack) {
    dprint(" ");
    dprint("PREVAL");
    dprint(" ");

    store.push();

    for n in &node.children {
        let t: &NodeType = &n.nodetype;

        match t {
            NodeType::FunDef(fname) => {
                dprint(format!("Preval: NodeType::FUNDEF '{}'", fname));

                let params = &n.children[0];
                dprint(format!("{}", params));

                let body = n.children[1].clone();

                if params.nodetype != NodeType::ParamList {
                    panic!("Expected paramlist for FUNDEF in preeval.");
                }

                let mut args: Vec<String> = Vec::new();

                for i in 0..params.children.len() {
                    let p = &params.children[i];
                    match &p.nodetype {
                        NodeType::Name(s) => {
                            args.push(s.clone());
                        }
                        x => panic!("Invalid parameter: {}", x)
                    }
                }

                let obj = Object::FUNCTION(fname.to_string(), body, args);
                store.add(fname, obj);

                dprint(format!("Inserted to symtable: {}", fname));
            }
            x => {
                dprint(format!("Preval considering node {}", x));
            }
        }
    }
}



pub fn eval(node: &Node, store: &mut Stack) -> Object {

    let t: &NodeType = &node.nodetype;

    match t {

        NodeType::Assign => {
            dprint("Eval: NodeType::ASSIGN");
            match &node.children[0].nodetype {
                NodeType::Name(ref s1) => {
                    let right_obj = eval(&node.children[1], store);
                    store.add(s1.as_str(), right_obj);
                    return Object::VOID;
                }
                NodeType::TypedVar(_, ref s1) => {
                    let right_obj = eval(&node.children[1], store);
                    // symtable.insert(s1.clone(), right_obj);
                    store.add(s1.as_str(), right_obj);
                    return Object::VOID;
                }
                _ => panic!("Illegal name for assignment: {}", &node.children[0].nodetype)
            }
        }

        NodeType::Not => {
            let obj = eval(&node.children[0], store);

            return match obj {
                Object::BOOL(b) => {
                    Object::BOOL(!b)
                }
                _ => panic!("Illegal operand for '!'")
            }
        }

        NodeType::LogOr => {
            let left_obj = eval(&node.children[0], store);

            match left_obj {

                Object::BOOL(b1) => {

                    let right_obj = eval(&node.children[1], store);

                    match right_obj {

                        Object::BOOL(b2) => {
                            return Object::BOOL(b1 || b2)
                        }

                        _ => panic!("Illegal right operand for ||")
                    }
                }
                _ => panic!("Illegal left operand for ||")

            }
        }

        NodeType::LogAnd => {
            let left_obj = eval(&node.children[0], store);

            match left_obj {

                Object::BOOL(b1) => {

                    let right_obj = eval(&node.children[1], store);

                    match right_obj {

                        Object::BOOL(b2) => {
                            return Object::BOOL(b1 && b2)
                        }

                        _ => panic!("Illegal right operand for &&")
                    }
                }
                _ => panic!("Illegal left operand for &&")

            }
        }

        NodeType::LessThan => {
            let left_obj = eval(&node.children[0], store);

            match left_obj {

                Object::INT(n1) => {

                    let right_obj = eval(&node.children[1], store);

                    match right_obj {

                        Object::INT(n2) => {
                            return Object::BOOL(n1 < n2)
                        }
                        Object::DOUBLE(x2) => {
                            return Object::BOOL((n1 as f64) < x2)
                        }
                        _ => panic!("Illegal right operand for <")
                    }
                }

                Object::DOUBLE(x1) => {
                    let right_obj = eval(&node.children[1], store);

                    match right_obj {

                        Object::INT(n2) => {
                            return Object::BOOL(x1 < (n2 as f64))
                        }
                        Object::DOUBLE(x2) => {
                            return Object::BOOL(x1 < x2)
                        }
                        _ => panic!("Illegal right operand for <")
                    }
                }
                _ => panic!("Illegal left operand for <")
            }
        }

        NodeType::GreaterThan => {
            let left_obj = eval(&node.children[0], store);

            match left_obj {

                Object::INT(n1) => {

                    let right_obj = eval(&node.children[1], store);

                    match right_obj {

                        Object::INT(n2) => {
                            return Object::BOOL(n1 > n2)
                        }
                        Object::DOUBLE(x2) => {
                            return Object::BOOL((n1 as f64) > x2)
                        }
                        _ => panic!("Illegal right operand for >")
                    }
                }

                Object::DOUBLE(x1) => {
                    let right_obj = eval(&node.children[1], store);

                    match right_obj {

                        Object::INT(n2) => {
                            return Object::BOOL(x1 > (n2 as f64))
                        }
                        Object::DOUBLE(x2) => {
                            return Object::BOOL(x1 > x2)
                        }
                        _ => panic!("Illegal right operand for >")
                    }
                }
                _ => panic!("Illegal left operand for >")
            }
        }

        NodeType::LessOrEq => {
            let left_obj = eval(&node.children[0], store);

            match left_obj {

                Object::INT(n1) => {

                    let right_obj = eval(&node.children[1], store);

                    match right_obj {

                        Object::INT(n2) => {
                            return Object::BOOL(n1 <= n2)
                        }
                        Object::DOUBLE(x2) => {
                            return Object::BOOL((n1 as f64) <= x2)
                        }
                        _ => panic!("Illegal right operand for <=")
                    }
                }

                Object::DOUBLE(x1) => {
                    let right_obj = eval(&node.children[1], store);

                    match right_obj {

                        Object::INT(n2) => {
                            return Object::BOOL(x1 <= (n2 as f64))
                        }
                        Object::DOUBLE(x2) => {
                            return Object::BOOL(x1 <= x2)
                        }
                        _ => panic!("Illegal right operand for <=")
                    }
                }
                _ => panic!("Illegal left operand for <=")
            }
        }

        NodeType::GreaterOrEq => {
            let left_obj = eval(&node.children[0], store);

            match left_obj {

                Object::INT(n1) => {

                    let right_obj = eval(&node.children[1], store);

                    match right_obj {

                        Object::INT(n2) => {
                            return Object::BOOL(n1 >= n2)
                        }
                        Object::DOUBLE(x2) => {
                            return Object::BOOL((n1 as f64) >= x2)
                        }
                        _ => panic!("Illegal right operand for >=")
                    }
                }

                Object::DOUBLE(x1) => {
                    let right_obj = eval(&node.children[1], store);

                    match right_obj {

                        Object::INT(n2) => {
                            return Object::BOOL(x1 >= (n2 as f64))
                        }
                        Object::DOUBLE(x2) => {
                            return Object::BOOL(x1 >= x2)
                        }
                        _ => panic!("Illegal right operand for >=")
                    }
                }
                _ => panic!("Illegal left operand for >=")
            }
        }

        NodeType::Equal => {
            let left_obj = eval(&node.children[0], store);
            let right_obj = eval(&node.children[1], store);

            match left_obj {

                Object::INT(n1) => {
                    return match right_obj {
                        Object::INT(n2) => {
                            Object::BOOL(n1 == n2)
                        }
                        Object::DOUBLE(x2) => {
                            Object::BOOL((n1 as f64) == x2)
                        }
                        _ => Object::BOOL(false)
                    }
                }
                Object::DOUBLE(x1) => {
                    return match right_obj {
                        Object::INT(n2) => {
                            Object::BOOL(x1 == (n2 as f64))
                        }
                        Object::DOUBLE(x2) => {
                            Object::BOOL(x1 == x2)
                        }
                        _ => Object::BOOL(false)
                    }
                }
                Object::BOOL(b1) => {
                    return match right_obj {
                        Object::BOOL(b2) => {
                            Object::BOOL(b1 == b2)
                        }
                        _ => Object::BOOL(false)
                    }

                }
                Object::STRING(s1) => {
                    return match right_obj {
                        Object::STRING(s2) => {
                            Object::BOOL(s1 == s2)
                        }
                        _ => Object::BOOL(false)
                    }
                }
                x => panic!("Equality not implemented for object: {:?}", x)
            }
        }

        NodeType::Add => {
            dprint("Eval: NodeType::ADD");

            let left_obj = eval(&node.children[0], store);

            match &left_obj {
                Object::INT(s1) => {

                    let right_obj = eval(&node.children[1], store);

                    match &right_obj {
                        Object::INT(s2) => {
                            Object::INT(s1 + s2)
                        }
                        Object::DOUBLE(s2) => {
                            Object::DOUBLE(*s1 as f64 + s2)
                        }
                        _ => panic!("Illegal right operand for addition: {:?}", &right_obj)
                    }
                },
                Object::DOUBLE(s1) => {
                    let right_obj = eval(&node.children[1], store);

                    match &right_obj {
                        Object::INT(s2) => {
                            Object::DOUBLE(s1 + *s2 as f64)
                        }
                        Object::DOUBLE(s2) => {
                            Object::DOUBLE(s1 + s2)
                        }
                        _ => panic!("Illegal right operand for addition: {:?}", &right_obj)
                    }
                }
                _ => panic!("Illegal left operand for addition: {:?}", &left_obj)
            }
        }

        NodeType::Sub => {
            dprint("Eval: NodeType::SUB");

            let left_obj = eval(&node.children[0], store);

            if node.children.len() == 1 {
                return match &left_obj {
                    Object::INT(n) => {
                        Object::INT(-*n)
                    }
                    Object::DOUBLE(x) => {
                        Object::DOUBLE(-*x)
                    }
                    _ => panic!("Illegal operand unary minus: {:?}", &left_obj)
                }
            }

            match &left_obj {
                Object::INT(s1) => {

                    let right_obj = eval(&node.children[1], store);

                    match &right_obj {
                        Object::INT(s2) => {
                            Object::INT(s1 - s2)
                        }
                        Object::DOUBLE(s2) => {
                            Object::DOUBLE(*s1 as f64 - s2)
                        }
                        _ => panic!("Illegal right operand for subtraction: {:?}", &right_obj)
                    }
                },
                Object::DOUBLE(s1) => {
                    let right_obj = eval(&node.children[1], store);

                    match &right_obj {
                        Object::INT(s2) => {
                            Object::DOUBLE(s1 - *s2 as f64)
                        }
                        Object::DOUBLE(s2) => {
                            Object::DOUBLE(s1 - s2)
                        }
                        _ => panic!("Illegal right operand for subtraction: {:?}", &right_obj)
                    }
                }
                _ => panic!("Illegal left operand for subtraction: {:?}", &left_obj)
            }
        }


        NodeType::Mul => {
            dprint("Eval: NodeType::MUL");

            let left_obj = eval(&node.children[0], store);

            match &left_obj {
                Object::INT(s1) => {

                    let right_obj = eval(&node.children[1], store);

                    match &right_obj {
                        Object::INT(s2) => {
                            Object::INT(s1 * s2)
                        }
                        Object::DOUBLE(s2) => {
                            Object::DOUBLE(*s1 as f64 * s2)
                        }
                        _ => panic!("Illegal right operand for multiplication: {:?}", &right_obj)
                    }
                },
                Object::DOUBLE(s1) => {
                    let right_obj = eval(&node.children[1], store);

                    match &right_obj {
                        Object::INT(s2) => {
                            Object::DOUBLE(s1 * *s2 as f64)
                        }
                        Object::DOUBLE(s2) => {
                            Object::DOUBLE(s1 * s2)
                        }
                        _ => panic!("Illegal right operand for multiplication: {:?}", &right_obj)
                    }
                }
                _ => panic!("Illegal left operand for multiplication: {:?}", &left_obj)
            }
        }

        NodeType::Div => {
            dprint("Eval: NodeType::DIV");

            let left_obj = eval(&node.children[0], store);

            match &left_obj {
                Object::INT(s1) => {

                    let right_obj = eval(&node.children[1], store);

                    match &right_obj {
                        Object::INT(s2) => {
                            Object::DOUBLE(*s1 as f64 / *s2 as f64)
                        }
                        Object::DOUBLE(s2) => {
                            Object::DOUBLE(*s1 as f64 / *s2)
                        }
                        _ => panic!("Illegal right operand for division: {:?}", &right_obj)
                    }
                },
                Object::DOUBLE(s1) => {

                    let right_obj = eval(&node.children[1], store);

                    match &right_obj {
                        Object::INT(s2) => {
                            Object::DOUBLE(*s1 as f64 / *s2 as f64)
                        }
                        Object::DOUBLE(s2) => {
                            Object::DOUBLE(*s1 as f64 / *s2)
                        }
                        _ => panic!("Illegal right operand for division: {:?}", &right_obj)
                    }
                },
                _ => panic!("Illegal left operand for division: {:?}", &left_obj)
            }
        }

        NodeType::PreIncrement => {
            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    let oldval = store.get(s).clone();

                    match oldval {
                        Object::INT(n) => {
                            let newval = Object::INT(n+1);
                            store.add(s.as_str(), newval.clone());
                            return newval;
                        }
                        _ => panic!("Illegal operand for increment.")
                    }
                }
                _ => panic!("Illegal operand for increment: {}", valnode)
            }
        }

        NodeType::PreDecrement => {
            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    let oldval = store.get(s).clone();

                    match oldval {
                        Object::INT(n) => {
                            let newval = Object::INT(n-1);
                            store.add(s.as_str(), newval.clone());
                            return newval;
                        }
                        _ => panic!("Illegal operand for increment.")
                    }
                }
                _ => panic!("Illegal operand for increment: {}", valnode)
            }
        }

        NodeType::PostIncrement => {
            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    let oldval = store.get(s).clone();

                    match oldval {
                        Object::INT(n) => {
                            let newval = Object::INT(n+1);
                            store.add(s.as_str(), newval);
                            return oldval;
                        }
                        _ => panic!("Illegal operand for increment.")
                    }
                }
                _ => panic!("Illegal operand for increment: {}", valnode)
            }
        }

        NodeType::PostDecrement => {
            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    let oldval = store.get(s).clone();

                    match oldval {
                        Object::INT(n) => {
                            let newval = Object::INT(n-1);
                            store.add(s.as_str(), newval);
                            return oldval;
                        }
                        _ => panic!("Illegal operand for decrement.")
                    }
                }
                _ => panic!("Illegal operand for decrement: {}", valnode)
            }
        }

        NodeType::Int(s) => {
            dprint("Eval: NodeType::INT");
            Object::INT(s.parse().unwrap())
        },

        NodeType::Double(s) => {
            dprint("Eval: NodeType::INT");
            // Object::DOUBLE(s.parse().unwrap())
            Object::DOUBLE((s.as_str()).parse::<f64>().unwrap())
        },

        NodeType::Bool(v) => {
            dprint("Eval: NodeType::BOOL");
            Object::BOOL(*v)
        },

        NodeType::Str(s) => {
            dprint("Eval: NodeType::STRING");
            Object::STRING(s.clone())
        },

        NodeType::Name(s) => {
            dprint("Eval: NodeType::NAME");
            store.get(s).clone()
        }

        NodeType::Return => {
            return eval(&node.children[0], store);
        }

        NodeType::FunCall(s) => {
            dprint(format!("Eval: NodeType::FUNCALL({})", s));

            if store.has(s) {
                let funcobj = store.get(s).clone();
                match funcobj {
                    Object::FUNCTION(_, body, params) => {

                        let argslist = &node.children[0];

                        store.push();
                        for i in 0 .. params.len() {
                            let argtree = &argslist.children[i];
                            let argobj = eval(argtree, store);
                            store.add(params[i].as_str(), argobj);
                        }

                        let result = eval(&body, store);

                        store.pop();

                        return result;
                    }
                    _ => panic!("Called a non function object")
                }
            }

            if builtin::has_function(s) {
                let mut args: Vec<Object> = Vec::new();

                for argtree in &node.children[0].children {
                    args.push(eval(&argtree, store));
                }

                let res: Object = builtin::call(s, &args);
                return res;
            }

            panic!("Unknown function: {}", s)
        }

        NodeType::FunDef(s) => {
            dprint("Eval: NodeType::FUNDEF");

            let params = &node.children[0];
            let body = node.children[1].clone();

            if params.nodetype != NodeType::ParamList {
                panic!("Expected paramlist for FUNDEF in eval.");
            }

            let mut args: Vec<String> = Vec::new();

            for i in 0 .. params.children.len() {
                let p = &params.children[i];
                match &p.nodetype {
                    NodeType::Name(s) => {
                        args.push(s.clone());
                    }
                    x => panic!("Invalid parameter: {}", x)
                }
            }

            let obj = Object::FUNCTION(s.to_string(), body, args);

            store.add(s, obj);
            return Object::VOID;
        }

        NodeType::Conditional => {

            for condnode in &node.children {

                match condnode.nodetype {

                    NodeType::If |
                    NodeType::ElseIf => {
                        let boolnode= &condnode.children[0];

                        let res = eval(&boolnode, store);
                        match res {

                            Object::BOOL(v) => {
                                if v {
                                    let bodynode= &condnode.children[1];
                                    return eval(&bodynode, store);
                                }
                            }
                            _ => panic!("Expected bool in conditional")
                        }
                    }

                    NodeType::Else => {
                        let bodynode= &condnode.children[0];
                        return eval(&bodynode, store);
                    }
                    _ => panic!("Invalid node in conditional!")

                }
            }

            return Object::VOID;
        }

        NodeType::Block => {
            // I think this ends up with
            // whatever last expression in a function
            // being passed as a return value.
            // BLOCK should probably only propagate
            // values that are wrapped in a RETURN object...
            // or something like that.
            let mut ret = Object::VOID;
            for s in &node.children {
                ret = eval(s, store);
            }
            return ret;
        }

        NodeType::Module => {
            dprint("Eval: NodeType::MODULE");

            eval(&node.children[1], store)
        }

        _ => panic!("Unknown node type: {}", t)
    }
}

