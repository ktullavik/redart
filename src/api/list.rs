use crate::{error::evalerror, node::Node, object::Object, state::State};


pub fn add(
    fnode: &Node,
    args: Vec<Object>,
    state: &mut State) -> Object {

    if args.len() != 2 {
        // First arg is hidden from user.
        evalerror("One argument expected by List.add()", state, fnode)
    }

    if let Object::Reference(rk) = &args[0] {
        let ilist = state.objsys.get_list_mut(rk);
        ilist.add(args[1].clone());
        return Object::Null;
    }
    panic!("Unexpected type of internal argument for List.add(): {}", &args[0])
}


pub fn add_all(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    if args.len() != 2 {
        // First arg is hidden from user.
        evalerror("One argument expected by List.addAll()", state, fnode)
    }

    if let Object::Reference(rk1) = &args[0] {

        if let Object::Reference(rk2) = &args[1] {

            let arg_list_inst = state.objsys.get_instance(rk2);
            if !arg_list_inst.has_field("__list") {
                evalerror(
                    format!("Unexpected argument type for List.addAll(): {}",
                            arg_list_inst.classname),
                    state,
                    &argnodes[1]
                )
            }

            let arg_ilist_ref = arg_list_inst.get_field("__list");

            if let Object::Reference(arg_ilist_rk) = arg_ilist_ref {

                if !state.objsys.has_list(&arg_ilist_rk) {
                    panic!("Could not find list in __list reference: {}", arg_ilist_rk)
                }        
                let new_els = state.objsys.get_list(&arg_ilist_rk).els.clone();
                let ilist = state.objsys.get_list_mut(rk1);                            
                ilist.add_all(new_els);
                return Object::Null;
            }
            panic!("Internal __list field was not a reference: {}", arg_ilist_ref)
        }
        evalerror(
            format!("Unexpected argument for List.addAll(): {}", &args[1]),
            state,
            &argnodes[1]
        )
    }
    panic!("Unexpected internal argument for List.addAll: {}", &args[0])

}


pub fn clear(
    fnode: &Node,
    args: Vec<Object>,
    state: &mut State) -> Object {

    if args.len() != 1 {
        // Arg is hidden from user.
        evalerror("Zero arguments expected by List.clear()", state, fnode);
    }

    if let Object::Reference(rk) = &args[0] {

        let ilist = state.objsys.get_list_mut(rk);
        ilist.set_elements(Vec::new());
        return Object::Null;
    }
    panic!("Unexepcted internal argument for List.clear(): {}", &args[0])
}


pub fn insert(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    if args.len() != 3 {
        // First arg is hidden from user.
        evalerror("Two arguments expected by List.insert()", state, fnode);
    }

    if let Object::Reference(rk) = &args[0] {

        if let Object::Int(n) = &args[1] {

            let ilist = state.objsys.get_list_mut(rk);

            if *n < 0 {
                evalerror(
                    format!("Negative index: {}", n),
                    state,
                    &argnodes[1]
                )
            }
            if *n > (ilist.els.len() as i64) {
                evalerror(
                    format!("Out of bounds index: {} (list length is {})", n, ilist.els.len()),
                    state,
                    &argnodes[1]
                )
            }
            ilist.insert(n.clone() as usize, args[2].clone());
            return Object::Null;
        }
        evalerror(
            format!("Unexpected argument for List.insert(): {}", &args[1]),
            state,
            &argnodes[1]
        )
    }
    panic!("Unexpected internal argument for List.insert(): {}", &args[0])
}


pub fn remove_at(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    if args.len() != 2 {
        // First arg is hidden from user.
        evalerror("One argument expected by List.removeAt()", state, fnode);
    }

    if let Object::Reference(rk) = &args[0] {

        if let Object::Int(n) = &args[1] {

            let ilist = state.objsys.get_list_mut(rk);

            if *n < 0 {
                evalerror(
                    format!("Negative index: {}", n),
                    state,
                    &argnodes[1]
                )
            }
            if *n >= (ilist.els.len() as i64) {
                evalerror(
                    format!("Out of bounds index: {} (list length is {})", n, ilist.els.len()),
                    state,
                    &argnodes[1]
                )
            }
            return ilist.remove_at(*n as usize)
        }
        evalerror(
            format!("Unexpected argument for List.removeAt(): {}", &args[1]),
            state,
            &argnodes[1]
        )
    }
    panic!("Unexpected internal argument for List.removeAt(): {}", &args[0])
}


pub fn remove_last(
    fnode: &Node,
    args: Vec<Object>,
    state: &mut State) -> Object {

    if args.len() != 1 {
        // Arg is hidden from user.
        evalerror("Zero arguments expected by List.removeLast()", state, fnode);
    }

    if let Object::Reference(rk) = &args[0] {
        let ilist = state.objsys.get_list_mut(rk);
        return ilist.remove_last();
    }
    panic!("Unexepected internal argument for List.removeLast(): {}", &args[0])
}


pub fn remove_range(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    if args.len() != 3 {
        // First arg is hidden from user.
        evalerror("Two arguments expected by List.removeLast()", state, fnode);
    }

    if let Object::Reference(rk) = &args[0] {

        let ilist = state.objsys.get_list_mut(rk);

        if let Object::Int(n1) = &args[1] {

            if *n1 < 0 {
                evalerror(
                    format!("Negative index: {}", n1),
                    state,
                    &argnodes[1])
            }

            if let Object::Int(n2) = &args[2] {

                if *n2 < 0 {
                    evalerror(
                        format!("Negative index: {}", n2),
                        state,
                        &argnodes[2]
                    )
                }
                if *n2 < *n1 {
                    evalerror(
                        format!("Second arg must be greater than first arg"),
                        state,
                        &argnodes[2]
                    )
                }
                if *n2 as usize > ilist.els.len() {
                    evalerror(
                        format!("Out of bounds index: {} (list length is {})", n2, ilist.els.len()),
                        state,
                        &argnodes[2]
                    )
                }

                ilist.remove_range(*n1 as usize, *n2 as usize);
                return Object::Null;
            }

            evalerror(
                format!("Unexpected argument: {}", &args[2]),
                state,
                &argnodes[2]
            )
        }
        evalerror(
            format!("Unexpected argument: {}", &args[1]),
            state,
            &argnodes[1]
        )
    }
    panic!("Unexpected internal argument for List.removeRange(): {}", &args[0])
}


pub fn shuffle(
    fnode: &Node,
    args: Vec<Object>,
    state: &mut State) -> Object {

    if args.len() != 1 {
        // First arg is hidden from user.
        evalerror("Zero arguments expected by List.shuffle()", state, fnode);
    }

    if let Object::Reference(rk) = &args[0]  {
        let ilist = state.objsys.get_list_mut(rk);
        ilist.shuffle();
        return Object::Null;
    }
    panic!("Unexpected internal argument for List.shuffle(): {}", args[0])
}


pub fn to_string(
    fnode: &Node,
    args: Vec<Object>,
    state: &mut State) -> Object {

    if args.len() != 1 {
        // First arg is hidden from user.
        evalerror("Zero arguments expected by List.toString()", state, fnode);
    }

    if let Object::Reference(rk) = &args[0]  {
        let ilist = state.objsys.get_list(rk);
        return Object::String(ilist.to_string());
    }
    panic!("Unexpected internal argument for List.toString(): {}", args[0])
}
