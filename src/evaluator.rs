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
            NodeType::FUNDEF(fname) => {
                dprint(format!("Preval: NodeType::FUNDEF '{}'", fname));

                let params = &n.children[0];
                dprint(format!("{}", params));

                let body = n.children[1].clone();

                if params.nodetype != NodeType::PARAMLIST {
                    panic!("Expected paramlist for FUNDEF in preeval.");
                }

                let mut args: Vec<String> = Vec::new();

                for i in 0..params.children.len() {
                    let p = &params.children[i];
                    match &p.nodetype {
                        NodeType::NAME(s) => {
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

        NodeType::ASSIGN => {
            dprint("Eval: NodeType::ASSIGN");
            match &node.children[0].nodetype {
                NodeType::NAME(ref s1) => {
                    let right_obj = eval(&node.children[1], store);
                    store.add(s1.as_str(), right_obj);
                    return Object::VOID;
                }
                NodeType::TYPEDVAR(_, ref s1) => {
                    let right_obj = eval(&node.children[1], store);
                    // symtable.insert(s1.clone(), right_obj);
                    store.add(s1.as_str(), right_obj);
                    return Object::VOID;
                }
                _ => panic!("Illegal name for assignment: {}", &node.children[0].nodetype)
            }
        },

        NodeType::ADD => {
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
        },

        NodeType::SUB => {
            dprint("Eval: NodeType::SUB");

            let left_obj = eval(&node.children[0], store);

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
        },


        NodeType::MUL => {
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
        },

        NodeType::DIV => {
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
        },

        NodeType::INT(s) => {
            dprint("Eval: NodeType::INT");
            Object::INT(s.parse().unwrap())
        },

        NodeType::DOUBLE(s) => {
            dprint("Eval: NodeType::INT");
            // Object::DOUBLE(s.parse().unwrap())
            Object::DOUBLE((s.as_str()).parse::<f64>().unwrap())
        },

        NodeType::BOOL(v) => {
            dprint("Eval: NodeType::BOOL");
            Object::BOOL(*v)
        },

        NodeType::STRING(s) => {
            dprint("Eval: NodeType::STRING");
            Object::STRING(s.clone())
        },

        NodeType::NAME(s) => {
            dprint("Eval: NodeType::NAME");
            store.get(s).clone()
        }

        NodeType::FUNCALL(s) => {
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
                            // symtable.insert(params[i].clone(), argobj);
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

        NodeType::FUNDEF(s) => {
            dprint("Eval: NodeType::FUNDEF");

            let params = &node.children[0];
            let body = node.children[1].clone();

            if params.nodetype != NodeType::PARAMLIST {
                panic!("Expected paramlist for FUNDEF in eval.");
            }

            let mut args: Vec<String> = Vec::new();

            for i in 0 .. params.children.len() {
                let p = &params.children[i];
                match &p.nodetype {
                    NodeType::NAME(s) => {
                        args.push(s.clone());
                    }
                    x => panic!("Invalid parameter: {}", x)
                }
            }

            let obj = Object::FUNCTION(s.to_string(), body, args);

            store.add(s, obj);
            return Object::VOID;
        }

        NodeType::CONDITIONAL => {

            let boolnode = &node.children[0];
            let bodynode = &node.children[1];

            let res = eval(&boolnode, store);
            match res {

                Object::BOOL(v) => {
                    if v {
                        return eval(&bodynode, store);
                    }
                }
                _ => panic!("Expected bool in conditional")
            }

            return Object::VOID;
        }

        NodeType::BLOCK => {
            for s in &node.children {
                eval(s, store);
            }
            return Object::VOID;
        }

        NodeType::MODULE => {
            dprint("Eval: NodeType::MODULE");

            eval(&node.children[1], store)
        }

        _ => panic!("Unknown node type: {}", t)
    }
}

