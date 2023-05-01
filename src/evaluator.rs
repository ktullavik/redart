use std::collections::HashMap;
use parser::Node;
use parser::NodeType;
use builtin;
use utils::dprint;


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


pub fn preval(node: &Node, symtable: &mut HashMap<String, Object>) {
    dprint(" ");
    dprint("PREVAL");
    dprint(" ");


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

                symtable.insert(fname.to_string(), obj);
                dprint(format!("Inserted to symtable: {}", fname));
            }
            x => {
                dprint(format!("Preval considering node {}", x));
            }
        }
    }
}



pub fn eval(node: &Node, symtable: &mut HashMap<String, Object>) -> Object {

    let t: &NodeType = &node.nodetype;

    match t {

        NodeType::ASSIGN => {
            dprint("Eval: NodeType::ASSIGN");
            match &node.children[0].nodetype {
                NodeType::NAME(ref s1) => {
                    let right_obj = eval(&node.children[1], symtable);
                    symtable.insert(s1.clone(), right_obj);
                    return Object::VOID;
                }
                NodeType::TYPEDVAR(_, ref s1) => {
                    let right_obj = eval(&node.children[1], symtable);
                    symtable.insert(s1.clone(), right_obj);
                    return Object::VOID;
                }
                _ => panic!("Illegal name for assignment: {}", &node.children[0].nodetype)
            }
        },

        NodeType::ADD => {
            dprint("Eval: NodeType::ADD");

            let left_obj = eval(&node.children[0], symtable);

            match &left_obj {
                Object::INT(s1) => {

                    let right_obj = eval(&node.children[1], symtable);

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
                    let right_obj = eval(&node.children[1], symtable);

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

            let left_obj = eval(&node.children[0], symtable);

            match &left_obj {
                Object::INT(s1) => {

                    let right_obj = eval(&node.children[1], symtable);

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
                    let right_obj = eval(&node.children[1], symtable);

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

            let left_obj = eval(&node.children[0], symtable);

            match &left_obj {
                Object::INT(s1) => {

                    let right_obj = eval(&node.children[1], symtable);

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
                    let right_obj = eval(&node.children[1], symtable);

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

            let left_obj = eval(&node.children[0], symtable);

            match &left_obj {
                Object::INT(s1) => {

                    let right_obj = eval(&node.children[1], symtable);

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

                    let right_obj = eval(&node.children[1], symtable);

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
            if *v {
                Object::BOOL(true)
            }
            else {
                Object::BOOL(false)
            }
        },

        NodeType::STRING(s) => {
            dprint("Eval: NodeType::STRING");
            // Object::STRING(s.parse().unwrap())
            Object::STRING(s.clone())
        },

        NodeType::NAME(s) => {
            dprint("Eval: NodeType::NAME");
            if symtable.contains_key(s) {
                match symtable.get(s).unwrap() {
                    &Object::INT(ref v) => {
                        Object::INT(*v)
                    },
                    &Object::DOUBLE(ref v) => {
                        Object::DOUBLE(*v)
                    },
                    &Object::STRING(ref v) => {
                        Object::STRING(String::from(v.clone()))
                    },
                    _ => {
                        panic!("Illegal value found in symbol table: {:?}", symtable.get(s))
                    }
                }
            }
            else {
                panic!("Undefined variable: {}", s)
            }
        }

        NodeType::FUNCALL(s) => {
            dprint(format!("Eval: NodeType::FUNCALL({})", s));

            if symtable.contains_key(s) {
                let funcobj = symtable[s].clone();
                match funcobj {
                    Object::FUNCTION(_, body, params) => {

                        let argslist = &node.children[0];

                        for i in 0 .. params.len() {

                            let argtree = &argslist.children[i];
                            let argobj = eval(argtree, symtable);

                            symtable.insert(params[i].clone(), argobj);
                        }

                        let result = eval(&body, symtable);

                        for i in 0 .. params.len() {
                            symtable.remove(params[i].as_str());
                        }

                        return result;
                    }
                    _ => panic!("Called a non function object")
                }
            }

            if builtin::has_function(s) {
                let mut args: Vec<Object> = Vec::new();

                for argtree in &node.children[0].children {
                    args.push(eval(&argtree, symtable));
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

            symtable.insert(s.to_string(), obj);
            return Object::VOID;
        }

        NodeType::CONDITIONAL => {

            let boolnode = &node.children[0];
            let bodynode = &node.children[1];

            let res = eval(&boolnode, symtable);
            match res {

                Object::BOOL(v) => {
                    if v {
                        return eval(&bodynode, symtable);
                    }
                }
                _ => panic!("Expected bool in conditional")
            }

            return Object::VOID;
        }

        NodeType::BLOCK => {
            for s in &node.children {
                eval(s, symtable);
            }
            return Object::VOID;
        }

        NodeType::MODULE => {
            dprint("Eval: NodeType::MODULE");

            eval(&node.children[1], symtable)
        }

        _ => panic!("Unknown node type: {}", t)
    }
}

