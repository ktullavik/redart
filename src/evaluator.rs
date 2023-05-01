use std::collections::HashMap;
use parser::Node;
use parser::NodeType;
use builtin;
use utils;


#[derive(Debug)]
#[derive(Clone)]
pub enum Object {
    INT(i64),
    NUM(f64),
    STRING(String),
    FUNCTION(String, Node, Vec<String>),
    VOID
}


pub fn preval(node: &Node, symtable: &mut HashMap<String, Object>) {
    utils::dprint(" ");
    utils::dprint("PREVAL");
    utils::dprint(" ");


    for n in &node.children {
        let t: &NodeType = &n.nodetype;

        match t {
            NodeType::FUNDEF(fname) => {
                utils::dprint(format!("Preval: NodeType::FUNDEF '{}'", fname));

                let params = &n.children[0];
                utils::dprint(format!("{}", params));

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
                utils::dprint(format!("Inserted to symtable: {}", fname));
            }
            x => {
                utils::dprint(format!("Preval considering node {}", x));
            }
        }
    }
}



pub fn eval(node: &Node, symtable: &mut HashMap<String, Object>) -> Object {

    let t: &NodeType = &node.nodetype;

    match t {

        NodeType::ASSIGN => {
            utils::dprint("Eval: NodeType::ASSIGN");
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
            utils::dprint("Eval: NodeType::ADD");

            let left_obj = eval(&node.children[0], symtable);

            match &left_obj {
                Object::NUM(s1) => {

                    let right_obj = eval(&node.children[1], symtable);

                    match &right_obj {
                        Object::NUM(s2) => {
                            Object::NUM(s1 + s2)
                        }
                        _ => panic!("Illegal right operand for addition: {:?}", &right_obj)
                    }
                },
                _ => panic!("Illegal left operand for addition: {:?}", &left_obj)
            }
        },

        NodeType::SUB => {
            utils::dprint("Eval: NodeType::SUB");

            let left_obj = eval(&node.children[0], symtable);

            match &left_obj {
                Object::NUM(s1) => {

                    let right_obj = eval(&node.children[1], symtable);

                    match &right_obj {
                        Object::NUM(s2) => {
                            Object::NUM(s1 - s2)
                        }
                        _ => panic!("Illegal right operand for subtraction: {:?}", &right_obj)
                    }
                },
                _ => panic!("Illegal left operand for subtraction: {:?}", &left_obj)
            }
        },


        NodeType::MUL => {
            utils::dprint("Eval: NodeType::MUL");

            let left_obj = eval(&node.children[0], symtable);

            match &left_obj {
                Object::NUM(s1) => {

                    let right_obj = eval(&node.children[1], symtable);

                    match &right_obj {
                        Object::NUM(s2) => {
                            Object::NUM(s1 * s2)
                        }
                        _ => panic!("Illegal right operand for multiplication: {:?}", &right_obj)
                    }
                },
                _ => panic!("Illegal left operand for multiplication: {:?}", &left_obj)
            }
        },

        NodeType::DIV => {
            utils::dprint("Eval: NodeType::DIV");

            let left_obj = eval(&node.children[0], symtable);

            match &left_obj {
                Object::NUM(s1) => {

                    let right_obj = eval(&node.children[1], symtable);

                    match &right_obj {
                        Object::NUM(s2) => {
                            Object::NUM(*s1 as f64 / *s2 as f64)
                        }
                        _ => panic!("Illegal right operand for division: {:?}", &right_obj)
                    }
                },
                _ => panic!("Illegal left operand for division: {:?}", &left_obj)
            }
        },

        NodeType::NUM(s) => {
            utils::dprint("Eval: NodeType::NUM");
            Object::NUM(s.parse().unwrap())
        },

        NodeType::STRING(s) => {
            utils::dprint("Eval: NodeType::STRING");
            // Object::STRING(s.parse().unwrap())
            Object::STRING(s.clone())
        },

        NodeType::NAME(s) => {
            utils::dprint("Eval: NodeType::NAME");
            if symtable.contains_key(s) {
                match symtable.get(s).unwrap() {
                    &Object::NUM(ref v) => {
                        Object::NUM(*v)
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
            utils::dprint(format!("Eval: NodeType::FUNCALL({})", s));

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
                let res: Object = builtin::call(s, &node.children, symtable);
                return res;
            }

            if s == "main" {
                let params = &node.children[0];
                let body = &node.children[1];
                return eval(body, symtable);
            }

            panic!("Unknown function: {}", s)
        }

        NodeType::FUNDEF(s) => {
            utils::dprint("Eval: NodeType::FUNDEF");

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

        NodeType::BLOCK => {
            for s in &node.children {
                eval(s, symtable);
            }
            return Object::VOID;
        }

        NodeType::MODULE => {
            utils::dprint("Eval: NodeType::MODULE");

            eval(&node.children[1], symtable)
        }

        _ => panic!("Unknown node type: {}", t)
    }
}

