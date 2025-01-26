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


pub fn get_name(name_node: &Node, state: &mut State) -> Object {

    if let NodeType::Name(s, linenum, symnum) = &name_node.nodetype {

        if state.in_const {
            evalerror(
                "Not a constant expression.",
                state,
                name_node
            )
        }

        // For Name, having a child means having an owner.
        if name_node.children.len() > 0 {
            // Run parent through the loop for lookup.
            let owner = eval(&name_node.children[0], state);
            return get_field(owner, s, state, name_node);
        }

        // Don't access stack or this if we are lazy evaling a topvar.
        if state.eval_var == "" {
            if state.stack.has(s) {
                return state.stack.get(s).clone();
            }
            else if state.objsys.has_this() {
                return get_field(state.objsys.get_this_object(), s, state, name_node);
            }
        }

        if state.has_global(s) {

            let n = state.get_global(s);

            match &n.nodetype {

                NodeType::TopVarLazy(typ, name, _, _) => {

                    if *name == state.eval_var {
                        evalerror(
                            format!("Top level variable '{}' depends on itself.", name),
                            state,
                            &n
                        );
                    }
                    if state.eval_var.len() == 0 {
                        state.eval_var = name.clone();
                    }

                    let res = eval(&n.children[0], state);
                    state.eval_var = String::from("");
                    let resolved_node = Node::new(NodeType::TopVar(
                        typ.clone(),
                        name.clone(),
                        Box::new(res.clone()),
                        linenum.clone(),
                        symnum.clone()
                    ));
                    state.set_global(s, resolved_node);
                    return res;
                }

                NodeType::TopVar(_, _, val, _, _) => {
                    return *val.clone();
                }

                NodeType::ConstTopLazy(typ, name, _, _) => {

                    if *name == state.eval_var {
                        evalerror(
                            format!("Top level const '{}' depends on itself.", name),
                            state,
                            &n
                        );
                    }
                    if state.eval_var.len() == 0 {
                        state.eval_var = name.clone();
                    }

                    state.in_const = true;
                    let res = eval(&n.children[0], state);
                    state.in_const = false;
                    state.eval_var = String::from("");
                    let resolved_node = Node::new(NodeType::ConstTopVar(
                        typ.clone(),
                        name.clone(),
                        Box::new(res.clone()),
                        linenum.clone(),
                        symnum.clone()
                    ));
                    state.set_global(s, resolved_node);
                    return res;
                }

                NodeType::ConstTopVar(_, _, val, _, _) => {
                    return *val.clone();
                }

                _ => panic!("Unexpected node type in globals: {}", n)
            }
        }
        if state.debug {
            state.stack.printstack();
        }
        // As dart.
        evalerror(
            format!("Undefined name: '{}'.", s),
            state,
            name_node
        );
    }
    evalerror(format!("Not a name: {}", name_node), state, name_node);
}


pub fn set_name(name_node: &Node, val: Object, state: &mut State) {

    if let NodeType::Name(name, linenum, symnum) = &name_node.nodetype {

        if name_node.children.len() > 0 {
            let left_obj = eval(&name_node.children[0], state);
            set_field(left_obj, &name, val, state, &name_node);
            return;
        }

        // Look on the stack.
        if state.stack.has(&name) {
            state.stack.update(&name, val);
            return;
        }

        // Look in 'this' instance.
        if state.objsys.has_this() {
            let this = state.objsys.get_this_instance_mut();
            if this.has_field(&name) {
                set_field(state.objsys.get_this_object(), &name, val, state, &name_node);
                return;
            }
        }

        // Look for globals.
        if state.has_global(&name) {

            let n = state.get_global_ref(&name);

            match &n.nodetype {

                NodeType::TopVarLazy(typ, _, _, _) |
                NodeType::TopVar(typ, _, _, _, _) => {
                    let newval = Node::new(
                        NodeType::TopVar(
                            typ.clone(),
                            name.clone(),
                            Box::new(val),
                            linenum.clone(),
                            symnum.clone()
                        )
                    );
                    state.set_global(&name, newval);
                    return;
                }
                NodeType::ConstTopLazy(_, name, _, _) |
                NodeType::ConstTopVar(_, name, _, _, _) => {
                    evalerror(format!(
                        "Cannot change const: {}", name),
                        state,
                        n
                    )
                }
                _ => panic!("Unexpected node type in globals: {}", n)
            }
        }

        evalerror(
            format!("Setter not found: '{}'", name),
            state,
            &name_node
        )
    }

    evalerror(
        format!("Can not assign to: {}", name_node),
        state,
        name_node
    )
}


pub fn get_field(obj: Object, field: &str, state: &mut State, node: &Node) -> Object {

    if let Object::Reference(rk) = obj {
        let inst = state.objsys.get_instance(&rk);

        let c = state.objsys.get_class(&inst.classname);

        if c.has_getter(field) {
            let g = c.get_getter(field, state, node);
            return call_function(
                MaybeRef::Ref(rk.clone()), 
                &g,
                &Node::new(NodeType::ArgList(0, 0)), 
                state
            )
        }

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

        NodeType::Constructor(cname, paramlist, initlist, body, filename, _, _) => {

            let mut paramobjs : Vec<ParamObj> = Vec::new();

            for i in 0..paramlist.children.len() {
                let p = &paramlist.children[i];
                match &p.nodetype {
                    NodeType::Name(s, _, _) => {
                        paramobjs.push(ParamObj{ typ: String::from(""), name: s.clone(), fieldinit: false });
                    }
                    NodeType::TypedVar(t, s, _, _) => {
                        paramobjs.push(ParamObj{ typ: t.clone(), name: s.clone(), fieldinit: false });
                    }
                    NodeType::ThisFieldInit(s, _, _) => {
                        paramobjs.push(ParamObj{ typ: String::from(""), name: s.clone(), fieldinit: true });
                    }
                    x => panic!("Invalid parameter: {}", x)
                }
            }
            return Object::Constructor(cname.to_string(), filename.clone(), paramobjs, *initlist.clone(), *body.clone());
        }
        _ => panic!("Invalid node type.")
    }
}


pub fn call_constructor(
    funcobj: &Object,
    args: &Node,
    state: &mut State) -> Object {

    match funcobj {

        Object::Constructor(cname, filename, params, initlist, body) => {

            let args = argnodes_to_argobjs(
                &args.children,
                state
            );

            // Argtrees must be evaluated in callers context, but stored in new context.

            state.stack.push_call();
            let oldfilename = state.filepath.clone();
            state.filepath = filename.clone();

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

            match &instref {
                Object::Reference(refid) => {

                    state.constructing.push(refid.clone());
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


                    let mut parent_args = &Node::new(NodeType::ArgList(0, 0));

                    // Initializer list
                    for i in 0 .. initlist.children.len() {
                        let initter = &initlist.children[i];

                        if let NodeType::Initializer(_, _) = initter.nodetype {

                            if let NodeType::Name(fieldname, _, _) = &initter.children[0].nodetype {

                                let fieldval = eval(&initter.children[1], state);
                                let inst = state.objsys.get_this_instance_mut();
                                inst.set_field(fieldname.clone(), fieldval)
                            }
                            else {
                                panic!("Expected name node in initializer list");
                            }
                        }
                        else if let NodeType::Super(_, _) = initter.nodetype {
                            parent_args = &initter.children[0];
                        }
                        else {
                            panic!("Expected initializer node in initializer list");
                        }
                    }
                     
                    // Initialize parent if it exists.
                    // TODO: initializer list

                    if parent_name != "" {

                        if state.has_global(parent_name.as_str()) {

                            let parent_cons = state.get_global_ref(parent_name.as_str());

                            match parent_cons.nodetype {
                                NodeType::Constructor(_, _, _, _, _, _, _) => {
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
                    assert!(state.constructing.last().unwrap() == refid);
                    state.constructing.pop();
                    state.filepath = oldfilename;
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

