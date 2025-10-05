
use crate::{node::{Node, NodeType}, state::State};


fn hastype(t: &String, state: &State) -> bool {


    if t == "void"
    || t == "bool"
    || t == "int"
    || t == "double"
    || t == "num"
    || t == "String" {
        return true;
    }

    if state.has_global(&t) {
        let funcnode = state.get_global(&t);
        if let NodeType::Constructor(_, _, _, _, _, _, _) = funcnode.nodetype {
            return true;
        }
    }
    return false;
}



pub fn typecheck(state: &mut State) {


    let globals = &mut state.globals;


    for n in globals {

        match &n.nodetype {

            NodeType::FunDef(typ, name, filename, _, _) => {
                println!("Typechecking: {} in {}", name, filename);

                let oldfilepath = state.filepath.clone();
                state.filepath = filename.clone();

                // if !hastype(&typ, state) {
                    // panic!("Type not found: {}", typ);
                // }
                // if !state.has_type(&typ) {
                //     panic!("Type not found: {}", typ);
                // }

                tc_block(&mut n.children[1]);

                state.filepath = oldfilepath;
            }

            NodeType::Constructor(name, _, _, _, filename, _, _) => {
                // println!("Typechecking: {} in {}", name, filename)
            }

            NodeType::ConstTopLazy(typ, name, _, _) => {
                // println!("Typechecking: {}:{}", name, typ)
            }

            NodeType::TopVarLazy(typ, name, _, _) => {
                // println!("Typechecking: {}:{}", name, typ)
            }

            _ => panic!("Not typechecking: {}", n)
        }
    }

}


// pub fn tc_block(node: &mut Node, state: &State) {
pub fn tc_block(node: &mut Node) {


    for n in &mut node.children {
        tc_statement(n);
    }
}


// pub fn tc_statement(node: &mut Node, state: &State) -> String {
pub fn tc_statement(node: &mut Node) -> String {

    match node.nodetype {

        NodeType::Assign(_, _) => {

            let n = &node.children[0];
            let v = &node.children[1];

            match &n.nodetype {

                NodeType::TypedVar(typ, name, lc, cc) => {


                    //let nn = Node::new(NodeType::Return(1, 1));
                    //n.children.push(n.clone());


                    let res = tc_expression(v);
                    if res != *typ && *typ != "var" {
                        panic!("Typecheck expected: {}, got: {}", typ, res)
                    }
                    let nn = Node::new(NodeType::TypedVar(res.clone(), name.clone(), *lc, *cc));
                    node.children[0] = nn;

                    return res;
                }

                _ => panic!("Not typechecking1")
            }
        }

        NodeType::FunCall(_, _, _) => String::from("void"),

        _ => {
            panic!("Not typechecking2")
        }
    }
}


// pub fn tc_expression(node: &Node, state: &State) -> String {
pub fn tc_expression(node: &Node) -> String {


    match node.nodetype {

        NodeType::Int(i, _, _) => String::from("int"),

        NodeType::Add(ln, cn) |
        NodeType::Sub(ln, cn) |
        NodeType::Mul(ln, cn) => {

            let t1 = tc_expression(&node.children[0]);
            let t2 = tc_expression(&node.children[1]);

            match t1.as_str() {

                "int" => {

                    match t2.as_str() {
                        "int" => String::from("int"),
                        "num" => String::from("num"),
                        _ => panic!("no type check")
                    }
                }

                "num" => {

                    match t2.as_str() {
                        "int" => String::from("num"),
                        "num" => String::from("num"),
                        _ => panic!("no type check")
                    }
                }


                _ => panic!("Type check failed")
            }
        }

        _ => panic!("Type check not implemented")
    }
}



