use crate::{error::{check_argc, evalerror}, evalhelp::{call_function, MaybeRef}, node::{Node, NodeType}, object::Object, state::State};



pub fn assert(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    if args.len() < 1 {
        evalerror(
            format!("Exoected 1 or 2 arguments for assert(). Got: {}", args.len()),
            state,
            fnode
        );
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
    return Object::Null;
}


pub fn print(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("print", 1, args.len(), fnode, state);

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
            x => evalerror(
                format!("toString was not a function of print argument: {}", x),
                state,
                fnode
            )
        }
    }
    else {
        println!("{}", &args[0]);
    }
    return Object::Null;
}

