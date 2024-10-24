use crate::evaluator::eval;
use crate::heapobjs::instance::MaybeObject;
use crate::node::{Node, NodeType};
use crate::object::{Object, ParamObj};
use crate::objsys::RefKey;
use crate::state::State;
use crate::error::evalerror;


pub enum MaybeRef {
    None,
    Ref(RefKey)
}


pub fn get_field(obj: Object, field: &str, state: &State, node: &Node) -> Object {

    if let Object::Reference(rk) = obj {
        let inst = state.objsys.get_instance(&rk);
        if inst.has_field(field) {
            return inst.get_field(field);
        }
        if let MaybeObject::Some(p) = &inst.parent {
            return get_field(p.clone(), field, state, node);
        }
        evalerror(
            format!("Object of type '{}' has no field '{}'", inst.classname, field),
            state,
            node
        )
    }
    panic!("Not a reference: {}", obj);
}


pub fn set_field(obj: Object, field: &str, val: Object, state: &mut State, node: &Node) {

    if let Object::Reference(rk) = &obj {
        let inst = state.objsys.get_instance_mut(&rk);
        
        if inst.has_field(field) {
            inst.set_field(String::from(field), val);
            return;
        }
        if let MaybeObject::Some(p) = &inst.parent {
            set_field(p.clone(), field, val, state, node);
            return;
        }
        evalerror(
            format!("Object of type '{}' has no field '{}'", inst.classname, field),
            state,
            node
        )
    }
    panic!("Not a reference: {}", obj);
}


pub fn set_list_element(ulist_ref: Object, index: Object, value: Object, state: &mut State, owner_node: &Node, index_node: &Node) -> Object {

    let ilist_ref = get_field(ulist_ref, "__list", state, owner_node);

    if let Object::Reference(ilist_rk) = ilist_ref {
        let ilist = state.objsys.get_list_mut(&ilist_rk);

        if let Object::Int(i) = index {
            if i < 0 {
                evalerror("Index must be positive.", state, index_node)
            }

            ilist.set_el(i as usize, value);
            return Object::Null;
        }
    }
    panic!("Expected reference when setting list element.")
}


pub fn create_function(funcnode: &Node) -> Object {

    match &funcnode.nodetype {

        NodeType::FunDef(fname, filename, _, _) => {
            let paramnodes = &funcnode.children[0];
            let bodynode = &funcnode.children[1];
            let mut paramobjs: Vec<ParamObj> = Vec::new();

            for i in 0..paramnodes.children.len() {
                let p = &paramnodes.children[i];
                match &p.nodetype {
                    NodeType::Name(s, _, _) => {
                        paramobjs.push(ParamObj { typ: String::from("var"), name: s.clone(), fieldinit: false });
                    }
                    x => panic!("Invalid parameter: {}", x)
                }
            }
            return Object::Function(fname.clone(), filename.clone(), bodynode.clone(), paramobjs);
        }
        _ => panic!("Invalid node type.")
    }
}


pub fn call_function(
    instance: MaybeRef,
    func: &Object,
    args: &Node,
    state: &mut State) -> Object {
    
    match func {

        Object::Function(funcname, filename, body, params) => {

            if args.children.len() != params.len() {
                evalerror(
                    format!("In method call {}, {} arguments expected but {} given.",
                        funcname, params.len(), args.children.len()),
                    state,
                    args
                );
            }
    
            let mut argobjs = argnodes_to_argobjs(
                &args.children,
                state
            );
    
            // Argtrees must be evaluated in the callers context,
            // but stored in the new call frame.
    
            state.stack.push_call();

            // Loop params backwards so that we can use pop().
            let mut i = params.len() as isize;
            loop {
                i -= 1;
                if i < 0 {
                    break;
                }
                state.stack.add_new(params[i as usize].name.as_str(), argobjs.pop().unwrap());
            }
    
            let oldfilename = state.filepath.clone();
            state.filepath = filename.clone();

            let mut oldthis = MaybeRef::None;
            if let MaybeRef::Ref(rk) = instance {
                oldthis = MaybeRef::Ref(state.objsys.get_this());
                state.objsys.set_this(rk);
            }
    
            let result = eval(&body, state);
    
            if let MaybeRef::Ref(old_rk) = oldthis {
                state.objsys.set_this(old_rk);
            }

            state.filepath = oldfilename;
            state.stack.pop_call();
    
            return match result {
                Object::Return(v) => {
                    *v.clone()
                }
    
                _ => {
                    result.clone()
                }
            }
        }

        x => panic!("Called a non-function object: {}", x)
    }
}


pub fn create_constructor(funcnode: &Node) -> Object {

    match &funcnode.nodetype {

        NodeType::Constructor(cname, filename, _, _) => {

            let paramnodes = &funcnode.children[0];
            let bodynode = &funcnode.children[1];

            let mut paramobjs : Vec<ParamObj> = Vec::new();

            for i in 0..paramnodes.children.len() {
                let p = &paramnodes.children[i];
                match &p.nodetype {
                    NodeType::Name(s, _, _) => {
                        paramobjs.push(ParamObj{ typ: String::from(""), name: s.clone(), fieldinit: false });
                    }
                    NodeType::ThisFieldInit(s, _, _) => {
                        paramobjs.push(ParamObj{ typ: String::from(""), name: s.clone(), fieldinit: true });
                    }
                    x => panic!("Invalid parameter: {}", x)
                }
            }
            return Object::Constructor(cname.to_string(), filename.clone(), bodynode.clone(), paramobjs);
        }
        _ => panic!("Invalid node type.")
    }
}


pub fn call_constructor(
    funcobj: &Object,
    args: &Node,
    state: &mut State) -> Object {

    match funcobj {

        Object::Constructor(cname, filename, body, params) => {

            let args = argnodes_to_argobjs(
                &args.children,
                state
            );

            // Argtrees must be evaluated in callers context, but stored in new context.

            state.stack.push_call();

            // Make an instance.

            let class = state.objsys.get_class(cname.as_str());
            let mut inst = class.instantiate();
            let parent_name = class.parent.clone();

            // Evaluate the initial field values.

            let field_nodes = class.fields.clone();
            for (_, fname, initexpr) in &field_nodes {
                inst.set_field(fname.clone(), eval(initexpr, state));
            }

            let instref = state.objsys.register_instance(*inst).clone();

            // Run the constructor body.

            match &instref {
                Object::Reference(refid) => {

                    state.constructing.push(refid.clone());
                    let oldfilename = state.filepath.clone();
                    state.filepath = filename.clone();
                    let oldthis = state.objsys.get_this();
                    state.objsys.set_this(refid.clone());

                    let inst = state.objsys.get_this_instance_mut();

                    for i in 0..params.len() {
                        // Set fields from params that uses "this" to auto-init.
                        // Ie Bike(this.gears)
                        if params[i].fieldinit {
                            inst.set_field(params[i].name.clone(),args[i].clone());
                        }
                        else {
                            state.stack.add_new(params[i].name.as_str(), args[i].clone());
                        }
                    }
                     
                    // Initialize parent if it exists.
                    // TODO: initializer list

                    if parent_name != "" {
                        let parent_args = &Node::new(NodeType::ArgList(0, 0));

                        if state.has_global(parent_name.as_str()) {

                            let parent_cons = state.get_global_ref(parent_name.as_str());

                            match parent_cons.nodetype {
                                NodeType::Constructor(_, _, _, _) => {
                                    let parent_rk = call_constructor(&create_constructor(parent_cons), parent_args, state);
                                    let inst = state.objsys.get_this_instance_mut();
                                    inst.parent = MaybeObject::Some(parent_rk);
                                }
                                _ => panic!("Not a constructor.")
                            }
                        }
                    }                    

                    // Run body
                    eval(&body, state);

                    state.objsys.set_this(oldthis);
                    state.filepath = oldfilename;
                    assert!(state.constructing.last().unwrap() == refid);
                    state.constructing.pop();
                    state.stack.pop_call();

                    return instref.clone();
                }
                _ => panic!("Couldn't find intance that was just created.")
            }
        }

        _ => {
            panic!("Called a non-constructor object.")
        }
    }
}


pub fn argnodes_to_argobjs(
    argnodes: &Vec<Node>,
    state: &mut State) -> Vec<Object> {

    argnodes.iter().map(
        |argtree| eval(&argtree, state)
    ).collect()
}

