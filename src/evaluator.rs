use std::collections::HashMap;
use parser::Node;
use parser::NodeType;
use builtin;


#[derive(Debug)]
#[derive(Clone)]
pub enum Object {

    NUM(i32),
    STRING(String),
    NAME(String),
    ASSIGN(String),
    FUNCTION(String, Node),
    EFFECT(String),
    VOID
}


pub fn eval(node: &Node, symtable: &mut HashMap<String, Object>) -> Object {

    let t: &NodeType = &node.nodetype;

    match t {

        NodeType::ASSIGN => {
            println!("NodeType::ASSIGN");
            match &node.children[0].nodetype {
                NodeType::NAME(ref s1) => {

                    let right_obj = eval(&node.children[1], symtable);
                    symtable.insert(String::from(s1.clone()), right_obj);
                    return Object::VOID;

                    // return Object::ASSIGN(String::from(s1.clone()));

                    // match &right_obj {
                    //
                    //     Object::NUM(ref s2) => {
                    //         symtable.insert(String::from(s1.clone()), Object::NUM(*s2));
                    //         return Object::ASSIGN(String::from(s1.clone()));
                    //     }
                    //     _ => panic!("Illegal value for assignment: {:?}", &right_obj)
                    // }
                },
                _ => panic!("Illegal name for assignment: {}", &node.children[0].nodetype)
            }
        },

        NodeType::ADD => {
            println!("NodeType::ADD");

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
            println!("NodeType::SUB");

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
            println!("NodeType::MUL");

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

        NodeType::NUM(s) => {
            println!("NodeType::NUM");
            Object::NUM(s.parse().unwrap())
        },

        NodeType::STRING(s) => {
            println!("NodeType::STRING");
            // Object::STRING(s.parse().unwrap())
            Object::STRING(s.clone())
        },

        NodeType::NAME(s) => {
            println!("NodeType::NAME");
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
            println!("NodeType::FUNCALL");

            if builtin::has_function(s) {
                let res: Object = builtin::call(s, &node.children);
                return res;
            }

            if symtable.contains_key(s) {
                let funcobj = symtable[s].clone();
                match funcobj {
                    Object::FUNCTION(_, body) => {
                        return eval(&body, symtable);
                    }
                    _ => panic!("Called a non function object")
                }
            }

            if s == "main" {
                let params = &node.children[0];
                let body = &node.children[1];
                println!("params: {}", params);
                println!("body: {}", body);
                return eval(body, symtable);
            }

            panic!("Unknown function: {}", s)
        }

        NodeType::FUNDEF(s) => {
            println!("NodeType::FUNDEF");

            let params = &node.children[0];
            // let body = &node.children[1];
            let body = node.children[1].clone();

            if params.nodetype != NodeType::PARAMLIST {
                panic!("Expected paramlist for FUNDEF in eval.");
            }

            let mut args: Vec<Object> = Vec::new();

            for i in 0 .. params.children.len() {
                args.push(eval(&params.children[i], symtable));
            }

            let obj = Object::FUNCTION(s.to_string(), body);

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
            println!("NodeType::MODULE");

            eval(&node.children[1], symtable)
        }



        _ => panic!("Unknown node type: {}", t)
    }
}

