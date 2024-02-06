use context::Ctx;
use node::{NodeType, Node};
use builtin;
use utils::{dprint, dart_evalerror};
use stack::Stack;
use object::{Object, ParamObj};
use objsys::ObjSys;
use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, BitXor};


pub fn eval(
    node: &Node,
    looktables: &HashMap<String, HashMap<String, usize>>,
    globals: &Vec<Node>,
    store: &mut Stack,
    objsys: &mut ObjSys,
    ctx: &mut Ctx) -> Object {

    let t: &NodeType = &node.nodetype;

    match t {

        NodeType::Assign => {
            dprint("Eval: NodeType::Assign");
            match &node.children[0].nodetype {
                NodeType::Name(name) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    if store.has(name) {
                        store.add(name, right_obj);
                        return Object::Null;
                    }

                    if !objsys.has_this() {
                        // As dart.
                        dart_evalerror(format!("Setter not found: '{}'", name), ctx)
                    }
                    let this = objsys.get_this_instance_mut();

                    if !this.has_field(name.to_string()) {
                        // As dart.
                        dart_evalerror(format!("The setter '{}' isn't defined for the class '{}'", name, this.classname), ctx)
                    }
                    this.set_field(name.to_string(), right_obj);

                    return Object::Null;
                }
                NodeType::TypedVar(_, name) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    if store.has_in_lexscope(name) {
                        // As dart.
                        dart_evalerror(format!("'{}' is already declared in this scope.", name), ctx);
                    }
                    else {
                        if objsys.has_this() {
                            let this = objsys.get_this_instance_mut();
                            if this.has_field(name.to_string()) {
                                panic!("Variable with name {} already exists.", name);
                            }
                        }
                        store.add(name, right_obj);
                    }

                    return Object::Null;
                }
                _ => panic!("Illegal name for assignment: {}", &node.children[0].nodetype)
            }
        }

        NodeType::Not => {
            dprint("Eval: NodeType::Not");

            let obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            return match obj {
                Object::Bool(b) => {
                    Object::Bool(!b)
                }
                _ => panic!("Illegal operand for '!'")
            }
        }

        NodeType::LogOr => {
            dprint("Eval: NodeType::LogOr");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match left_obj {

                Object::Bool(b1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match right_obj {

                        Object::Bool(b2) => {
                            return Object::Bool(b1 || b2)
                        }
                        _ => panic!("Illegal right operand for ||")
                    }
                }
                _ => panic!("Illegal left operand for ||")
            }
        }

        NodeType::LogAnd => {
            dprint("Eval: NodeType::LogAnd");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match left_obj {

                Object::Bool(b1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match right_obj {

                        Object::Bool(b2) => {
                            return Object::Bool(b1 && b2)
                        }
                        _ => panic!("Illegal right operand for &&")
                    }
                }
                _ => panic!("Illegal left operand for &&")
            }
        }

        NodeType::LessThan => {
            dprint("Eval: NodeType::LessThan");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match left_obj {

                Object::Int(n1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match right_obj {

                        Object::Int(n2) => {
                            return Object::Bool(n1 < n2)
                        }
                        Object::Double(x2) => {
                            return Object::Bool((n1 as f64) < x2)
                        }
                        _ => panic!("Illegal right operand for <")
                    }
                }

                Object::Double(x1) => {
                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match right_obj {

                        Object::Int(n2) => {
                            return Object::Bool(x1 < (n2 as f64))
                        }
                        Object::Double(x2) => {
                            return Object::Bool(x1 < x2)
                        }
                        _ => panic!("Illegal right operand for <")
                    }
                }
                _ => panic!("Illegal left operand for <")
            }
        }

        NodeType::GreaterThan => {
            dprint("Eval: NodeType::GreaterThan");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match left_obj {

                Object::Int(n1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match right_obj {

                        Object::Int(n2) => {
                            return Object::Bool(n1 > n2)
                        }
                        Object::Double(x2) => {
                            return Object::Bool((n1 as f64) > x2)
                        }
                        _ => panic!("Illegal right operand for >")
                    }
                }

                Object::Double(x1) => {
                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match right_obj {

                        Object::Int(n2) => {
                            return Object::Bool(x1 > (n2 as f64))
                        }
                        Object::Double(x2) => {
                            return Object::Bool(x1 > x2)
                        }
                        _ => panic!("Illegal right operand for >")
                    }
                }
                _ => panic!("Illegal left operand for >")
            }
        }

        NodeType::LessOrEq => {
            dprint("Eval: NodeType::LessOrEq");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match left_obj {

                Object::Int(n1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match right_obj {

                        Object::Int(n2) => {
                            return Object::Bool(n1 <= n2)
                        }
                        Object::Double(x2) => {
                            return Object::Bool((n1 as f64) <= x2)
                        }
                        _ => panic!("Illegal right operand for <=")
                    }
                }

                Object::Double(x1) => {
                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match right_obj {

                        Object::Int(n2) => {
                            return Object::Bool(x1 <= (n2 as f64))
                        }
                        Object::Double(x2) => {
                            return Object::Bool(x1 <= x2)
                        }
                        _ => panic!("Illegal right operand for <=")
                    }
                }
                _ => panic!("Illegal left operand for <=")
            }
        }

        NodeType::GreaterOrEq => {
            dprint("Eval: NodeType::GreaterOrEq");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match left_obj {

                Object::Int(n1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match right_obj {

                        Object::Int(n2) => {
                            return Object::Bool(n1 >= n2)
                        }
                        Object::Double(x2) => {
                            return Object::Bool((n1 as f64) >= x2)
                        }
                        _ => panic!("Illegal right operand for >=")
                    }
                }

                Object::Double(x1) => {
                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match right_obj {

                        Object::Int(n2) => {
                            return Object::Bool(x1 >= (n2 as f64))
                        }
                        Object::Double(x2) => {
                            return Object::Bool(x1 >= x2)
                        }
                        _ => panic!("Illegal right operand for >=")
                    }
                }
                _ => panic!("Illegal left operand for >=")
            }
        }

        NodeType::Equal => {
            dprint("Eval: NodeType::Equal");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);
            let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

            match left_obj {

                Object::Int(n1) => {
                    return match right_obj {
                        Object::Int(n2) => {
                            Object::Bool(n1 == n2)
                        }
                        Object::Double(x2) => {
                            Object::Bool((n1 as f64) == x2)
                        }
                        _ => Object::Bool(false)
                    }
                }
                Object::Double(x1) => {
                    return match right_obj {
                        Object::Int(n2) => {
                            Object::Bool(x1 == (n2 as f64))
                        }
                        Object::Double(x2) => {
                            Object::Bool(x1 == x2)
                        }
                        _ => Object::Bool(false)
                    }
                }
                Object::Bool(b1) => {
                    return match right_obj {
                        Object::Bool(b2) => {
                            Object::Bool(b1 == b2)
                        }
                        _ => Object::Bool(false)
                    }

                }
                Object::String(s1) => {
                    return match right_obj {
                        Object::String(s2) => {
                            Object::Bool(s1 == s2)
                        }
                        _ => Object::Bool(false)
                    }
                }
                x => panic!("Equality not implemented for object: {:?}", x)
            }
        }

        NodeType::BitAnd => {
            dprint("Eval: NodeType::BitAnd");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match &left_obj {

                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {

                        Object::Int(s2) => {
                            Object::Int(s1.bitand(s2))
                        }
                        _ => panic!("Illegal right operand for bitwise and: {:?}", &right_obj)
                    }
                }
                _ => panic!("Illegal left operand for bitwise and: {:?}", &left_obj)
            }
        }

        NodeType::BitOr => {
            dprint("Eval: NodeType::BitOr");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match &left_obj {

                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {

                        Object::Int(s2) => {
                            Object::Int(s1.bitor(s2))
                        }
                        _ => panic!("Illegal right operand for bitwise or: {:?}", &right_obj)
                    }
                }
                _ => panic!("Illegal left operand for bitwise or: {:?}", &left_obj)
            }
        }

        NodeType::BitXor => {
            dprint("Eval: NodeType::BitXor");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match &left_obj {

                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {

                        Object::Int(s2) => {
                            Object::Int(s1.bitxor(s2))
                        }
                        _ => panic!("Illegal right operand for bitwise xor: {:?}", &right_obj)
                    }
                }
                _ => panic!("Illegal left operand for bitwise xor: {:?}", &left_obj)
            }
        }

        NodeType::Add => {
            dprint("Eval: NodeType::Add");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match &left_obj {
                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1 + s2)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 + s2)
                        }
                        _ => panic!("Illegal right operand for addition: {:?}", &right_obj)
                    }
                },
                Object::Double(s1) => {
                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(s1 + *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(s1 + s2)
                        }
                        _ => panic!("Illegal right operand for addition: {:?}", &right_obj)
                    }
                }
                Object::String(s1) => {
                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {
                        Object::String(s2) => {
                            let mut ret = s1.clone();
                            ret.push_str(s2);
                            return Object::String(ret);
                        }
                        _ => panic!("Illegal right operand for addition: {:?}", &right_obj)
                    }
                }
                _ => panic!("Illegal left operand for addition: {:?}", &left_obj)
            }
        }

        NodeType::Sub => {
            dprint("Eval: NodeType::Sub");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            if node.children.len() == 1 {
                return match &left_obj {
                    Object::Int(n) => {
                        Object::Int(-*n)
                    }
                    Object::Double(x) => {
                        Object::Double(-*x)
                    }
                    _ => panic!("Illegal operand unary minus: {:?}", &left_obj)
                }
            }

            match &left_obj {
                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1 - s2)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 - s2)
                        }
                        _ => panic!("Illegal right operand for subtraction: {:?}", &right_obj)
                    }
                },
                Object::Double(s1) => {
                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(s1 - *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(s1 - s2)
                        }
                        _ => panic!("Illegal right operand for subtraction: {:?}", &right_obj)
                    }
                }
                _ => panic!("Illegal left operand for subtraction: {:?}", &left_obj)
            }
        }


        NodeType::Mul => {
            dprint("Eval: NodeType::Mul");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match &left_obj {
                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1 * s2)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 * s2)
                        }
                        _ => panic!("Illegal right operand for multiplication: {:?}", &right_obj)
                    }
                },
                Object::Double(s1) => {
                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(s1 * *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(s1 * s2)
                        }
                        _ => panic!("Illegal right operand for multiplication: {:?}", &right_obj)
                    }
                }
                _ => panic!("Illegal left operand for multiplication: {:?}", &left_obj)
            }
        }

        NodeType::Div => {
            dprint("Eval: NodeType::Div");

            let left_obj = eval(&node.children[0], looktables, globals, store, objsys, ctx);

            match &left_obj {
                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(*s1 as f64 / *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 / *s2)
                        }
                        _ => panic!("Illegal right operand for division: {:?}", &right_obj)
                    }
                },
                Object::Double(s1) => {

                    let right_obj = eval(&node.children[1], looktables, globals, store, objsys, ctx);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(*s1 as f64 / *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 / *s2)
                        }
                        _ => panic!("Illegal right operand for division: {:?}", &right_obj)
                    }
                },
                _ => panic!("Illegal left operand for division: {:?}", &left_obj)
            }
        }

        NodeType::PreIncrement => {
            dprint("Eval: NodeType::PreIncrement");

            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    if store.has(s) {
                        let oldval = store.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                store.add(s.as_str(), newval.clone());
                                return newval;
                            }
                            _ => panic!("Illegal operand for preincrement.")
                        }
                    }
                    else {
                        let this = objsys.get_this_instance_mut();
                        let oldval = this.get_field(s.clone()).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                this.set_field(s.clone(), newval.clone());
                                return newval;
                            }
                            _ => panic!("Illegal operand for preincrement.")
                        }
                    }
                }
                _ => panic!("Illegal operand for preincrement: {}", valnode)
            }
        }

        NodeType::PreDecrement => {
            dprint("Eval: NodeType::PreDecrement");

            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    if store.has(s) {
                        let oldval = store.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                store.add(s.as_str(), newval.clone());
                                return newval;
                            }
                            _ => panic!("Illegal operand for preincrement.")
                        }
                    }
                    else {
                        let this = objsys.get_this_instance_mut();
                        let oldval = this.get_field(s.clone()).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                this.set_field(s.clone(), newval.clone());
                                return newval;
                            }
                            _ => panic!("Illegal operand for predecrement.")
                        }
                    }
                }
                _ => panic!("Illegal operand for predecrement: {}", valnode)
            }
        }

        NodeType::PostIncrement => {
            dprint("Eval: NodeType::PostIncrement");

            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    if store.has(s) {
                        let oldval = store.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                store.add(s.as_str(), newval);
                                return oldval;
                            }
                            _ => panic!("Illegal operand for increment.")
                        }
                    }
                    else {
                        let this = objsys.get_this_instance_mut();
                        let oldval = this.get_field(s.clone()).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                this.set_field(s.clone(), newval);
                                return oldval;
                            }
                            _ => panic!("Illegal operand for increment.")
                        }

                    }
                }
                _ => panic!("Illegal operand for increment: {}", valnode)
            }
        }

        NodeType::PostDecrement => {
            dprint("Eval: NodeType::PostDecrement");

            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    if store.has(s) {
                        let oldval = store.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                store.add(s.as_str(), newval);
                                return oldval;
                            }
                            _ => panic!("Illegal operand for decrement.")
                        }
                    }
                    else {
                        let this = objsys.get_this_instance_mut();
                        let oldval = this.get_field(s.clone()).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                this.set_field(s.clone(), newval);
                                return oldval;
                            }
                            _ => panic!("Illegal operand for decrement.")
                        }
                    }
                }
                _ => panic!("Illegal operand for decrement: {}", valnode)
            }
        }

        NodeType::Int(val) => {
            dprint("Eval: NodeType::Int");
            Object::Int(*val)
        },

        NodeType::Double(val) => {
            dprint("Eval: NodeType::Double");
            Object::Double(*val)
        },

        NodeType::Bool(v) => {
            dprint("Eval: NodeType::Bool");
            Object::Bool(*v)
        },

        NodeType::Str(s) => {
            dprint("Eval: NodeType::Str");
            if node.children.is_empty() {
                return Object::String(s.clone())
            }

            let mut evaled_itps = Vec::new();
            for itp in &node.children {
                evaled_itps.push(eval(itp, looktables, globals, store, objsys, ctx));
            }

            let parts : Vec<&str> = s.as_str().split("$").collect();

            let mut built : String = String::new();

            for i in 0 .. evaled_itps.len() {
                built = String::from(format!("{}{}{}", parts[0], built.clone(), evaled_itps[i].clone()).as_str());
            }
            built.push_str(parts.last().unwrap());

            return Object::String(built)
        },

        NodeType::Name(s) => {
            dprint(format!("Eval: NodeType::Name({})", s));

            // For Name, having a child means having an owner.
            if node.children.len() > 0 {
                let owner = eval(&node.children[0], looktables, globals, store, objsys, ctx);
                
                if let Object::Reference(refid) = owner {
                    let instance = objsys.get_instance(&refid);
                    return instance.get_field(s.to_string()).clone();
                }

                panic!("Unexpected owner for {}: {}", s, owner)
            }


            if store.has(s) {
                dprint(format!("got value for {}", s));
                return store.get(s).clone();
            }
            else if objsys.has_this() {
                let this = objsys.get_this_instance_mut();
                return this.get_field(s.clone()).clone();
            }
            else {
                store.printstack();
                // As dart.
                dart_evalerror(format!("Undefined name: '{}'.", s), ctx);
            }
        }

        NodeType::Return => {
            dprint(format!("Eval: NodeType::Return"));
            let retval = eval(&node.children[0], looktables, globals, store, objsys, ctx);
            return Object::Return(Box::new(retval));
        }

        NodeType::MethodCall(name, owner, filename) => {
            dprint(format!("Eval: NodeType::MethodCall({})", name));


            let reference: Object = eval(owner, looktables, globals, store, objsys, ctx);


            if let Object::Reference(refid) = reference {

                let instance = objsys.get_instance(&refid);
                let c = objsys.get_class(&instance.classname);

                let meth = c.get_method(name);
                if let Object::Function(_, _, node, params) = meth {

                    let argslist = &node.children[0];

                    store.push_call();

                    let oldfilename = ctx.filepath.clone();
                    ctx.filepath = filename.clone();

                    let oldthis = objsys.get_this();
                    objsys.set_this(instance.id.clone());

                    for i in 0 .. params.len() {
                        let argtree = &argslist.children[i];
                        let argobj = eval(argtree, looktables, globals, store, objsys, ctx);
                        store.add(params[i].name.as_str(), argobj);
                    }

                    let result = eval(&node, looktables, globals, store, objsys, ctx);

                    objsys.set_this(oldthis);
                    ctx.filepath = oldfilename;
                    store.pop_call();

                    return match result {
                        Object::Return(v) => {
                            *v.clone()
                        }

                        _ => {
                            result.clone()
                        }
                    }
                }
            }
            panic!("Can't access {} of {}", name, owner);
        }

        NodeType::FunCall(s) => {
            dprint(format!("Eval: NodeType::FunCall({})", s));

            if store.has(s) {
                let funcobj = store.get(s).clone();

                return match funcobj {
                    Object::Function(_, _, _, _) => {
                        call_function(funcobj, &node.children[0], looktables, globals, store, objsys, ctx)
                    }
                    Object::Constructor(_, _, _, _) => {
                        call_constructor(&funcobj, &node.children[0], looktables, globals, store, objsys, ctx)
                    }
                    _ => panic!("Called non-callable.")
                }
            }
            else if builtin::has_function(s) {

                let args = argnodes_to_argobjs(
                    &node.children[0].children,
                    looktables,
                    globals,
                    store,
                    objsys,
                    ctx
                );

                return builtin::call(s, &args, ctx);
            }
            else {
                println!("FuncCall, table: {}", &ctx.filepath);
                let ltable = &looktables[&ctx.filepath];
                if ltable.contains_key(s) {

                    let funcindex = ltable.get(s).unwrap().clone();
                    let funcnode = &globals[funcindex];

                    return match funcnode.nodetype {
                        NodeType::FunDef(_, _) => {
                            call_function(
                                create_function(&funcnode),
                                &node.children[0],
                                looktables,
                                globals,
                                store,
                                objsys,
                                ctx)
                        }
                        NodeType::Constructor(_, _) => {
                            call_constructor(
                                &create_constructor(&funcnode),
                                &node.children[0],
                                looktables,
                                globals,
                                store,
                                objsys,
                                ctx)
                        }
                        _ => panic!("Expected function definition or constructor.")
                    }
                }

                panic!("Unknown function: {}", s)
            }
        }

        NodeType::FunDef(s, _) => {
            dprint("Eval: NodeType::FunDef");
            let funcobj = create_function(node);
            store.add(s, funcobj);
            return Object::Null;
        }

        NodeType::Conditional => {
            dprint("Eval: NodeType::Conditional");

            for condnode in &node.children {

                match condnode.nodetype {

                    NodeType::If |
                    NodeType::ElseIf => {
                        let boolnode= &condnode.children[0];

                        let cond = eval(&boolnode, looktables, globals, store, objsys, ctx);
                        match cond {

                            Object::Bool(v) => {
                                if v {
                                    let bodynode= &condnode.children[1];
                                    store.push_lex();
                                    let ret = eval(&bodynode, looktables, globals, store, objsys, ctx);
                                    store.pop_lex();
                                    return ret;
                                }
                            }
                            _ => panic!("Expected bool in conditional")
                        }
                    }

                    NodeType::Else => {
                        let bodynode= &condnode.children[0];
                        store.push_lex();
                        let ret = eval(&bodynode, looktables, globals, store, objsys, ctx);
                        store.pop_lex();
                        return ret;
                    }
                    _ => panic!("Invalid node in conditional!")

                }
            }

            return Object::Null;
        }

        NodeType::While => {
            dprint("Eval: NodeType::While");

            let boolnode = &node.children[0];
            let block = &node.children[1];

            let mut cond = eval(boolnode, looktables, globals, store, objsys, ctx);

            match &cond {

                Object::Bool(mut v) => {

                    while v {
                        eval(block, looktables, globals, store, objsys, ctx);
                        cond = eval(boolnode, looktables, globals, store, objsys, ctx);

                        match &cond {
                            Object::Bool(newcond) => {
                                v = *newcond;
                            }
                            _ => {
                                panic!("Conditional no longer bool: {}", cond)
                            }
                        }
                    }
                }
                _ => panic!("Expected bool in conditional")
            }
            return Object::Null;
        }

        NodeType::DoWhile => {
            dprint("Eval: NodeType::DoWhile");

            let block = &node.children[0];
            let boolnode = &node.children[1];

            eval(block, looktables, globals, store, objsys, ctx);

            let mut cond = eval(boolnode, looktables, globals, store, objsys, ctx);

            if let Object::Bool(mut b) = cond {

                while b {

                    eval(block, looktables, globals, store, objsys, ctx);
                    cond = eval(boolnode, looktables, globals, store, objsys, ctx);

                    match &cond {
                        Object::Bool(new_b) => {
                            b = *new_b;
                        }
                        _ => panic!("Conditional no longer bool: {}", cond)
                    }
                }
            }
            else {
                panic!("Expected bool in conditional")
            }

            return Object::Null;
        }

        NodeType::For => {
            dprint("Eval: NodeType::For");

            let assign = &node.children[0];
            let condexpr = &node.children[1];
            let mutexpr = &node.children[2];
            let body = &node.children[3];

            eval(assign, looktables, globals, store, objsys, ctx);

            loop {

                let condobj = eval(condexpr, looktables, globals, store, objsys, ctx);

                match condobj {
                    Object::Bool(b) => {

                        if !b {
                            break;
                        }

                        eval(body, looktables, globals, store, objsys, ctx);
                        eval(mutexpr, looktables, globals, store, objsys, ctx);
                    }
                    x => dart_evalerror(format!("Expected bool. Got: {}", x), ctx)

                }
            }
            return Object::Null;
        }

        NodeType::Block => {
            dprint("Eval: NodeType::Block");

            for c in &node.children {

                let retval = eval(c, looktables, globals, store, objsys, ctx);

                match &retval {
                    Object::Return(_) => {
                        return retval;
                    }
                    _ => {}
                }
            }
            return Object::Null;
        }

        NodeType::Null => {
            dprint("Eval:: NodeType::Null");
            return Object::Null;
        }

        _ => panic!("Unknown node type: {}", t)
    }
}


fn create_function(funcnode: &Node) -> Object {

    match &funcnode.nodetype {

        NodeType::FunDef(fname, filename) => {
            let paramnodes = &funcnode.children[0];
            let bodynode = &funcnode.children[1];
            let mut paramobjs: Vec<ParamObj> = Vec::new();

            for i in 0..paramnodes.children.len() {
                let p = &paramnodes.children[i];
                match &p.nodetype {
                    NodeType::Name(s) => {
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


fn call_function(
    funcobj: Object,
    args: &Node,
    looktables: &HashMap<String, HashMap<String, usize>>,
    globals: &Vec<Node>,
    store: &mut Stack,
    objsys: &mut ObjSys,
    ctx: &mut Ctx) -> Object {

    match funcobj {

        Object::Function(_, filename, body, params) => {

            let mut argobjs = argnodes_to_argobjs(
                &args.children,
                looktables,
                globals,
                store,
                objsys,
                ctx
            );

            // Argtrees must be evaluated in callers context, but stored in new context.

            store.push_call();
            for i in 0..params.len() {
                store.add(params[i].name.as_str(), argobjs.remove(0));
            }

            let oldfilepath = ctx.filepath.clone();

            ctx.filepath = filename;
            println!("Setting filepath: {}", &ctx.filepath);

            let result = eval(&body, looktables, globals, store, objsys, ctx);

            ctx.filepath = oldfilepath;
            println!("Restoring filepath: {}", &ctx.filepath);

            store.pop_call();

            return match result {
                Object::Return(v) => {
                    *v
                }
                _ => {
                    result
                }
            }
        }
        _ => panic!("Called a non-function object.")
    }
}


fn create_constructor(funcnode: &Node) -> Object {

    match &funcnode.nodetype {

        NodeType::Constructor(cname, filename) => {

            let paramnodes = &funcnode.children[0];
            let bodynode = &funcnode.children[1];

            let mut paramobjs : Vec<ParamObj> = Vec::new();

            for i in 0..paramnodes.children.len() {
                let p = &paramnodes.children[i];
                match &p.nodetype {
                    NodeType::Name(s) => {
                        paramobjs.push(ParamObj{ typ: String::from(""), name: s.clone(), fieldinit: false });
                    }
                    NodeType::ThisFieldInit(s) => {
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


fn call_constructor(
    funcobj: &Object,
    args: &Node,
    looktables: &HashMap<String, HashMap<String, usize>>,
    globals: &Vec<Node>,
    store: &mut Stack,
    objsys: &mut ObjSys,
    ctx: &mut Ctx) -> Object {

    match funcobj {

        Object::Constructor(cname, filename, body, params) => {

            let args = argnodes_to_argobjs(
                &args.children,
                looktables,
                globals,
                store,
                objsys,
                ctx
            );

            // Argtrees must be evaluated in callers context, but stored in new context.

            store.push_call();
            for i in 0..params.len() {
                // Field initializers does not need to be in symbol table.
                // They are set directly on the instance. See below.
                if !params[i].fieldinit {
                    store.add(params[i].name.as_str(), args[i].clone());
                }
            }

            // Make an instance.

            let class = objsys.get_class(cname.as_str());
            let mut inst = class.instantiate();

            // Evaluate the initial field values.

            let field_nodes = class.fields.clone();
            for (_, fname, initexpr) in &field_nodes {
                inst.set_field(fname.clone(), eval(initexpr, looktables, globals, store, objsys, ctx));
            }

            let instref = objsys.register_instance(inst);

            // Run the constructor body.

            match &instref {
                Object::Reference(refid) => {

                    let oldfilename = ctx.filepath.clone();
                    ctx.filepath = filename.clone();


                    let oldthis = objsys.get_this();
                    objsys.set_this(refid.clone());
                    println!("Set this: {}, classname: {}, filename: {}", refid, cname, filename);

                    // Set fields from params that uses "this" to auto-init.
                    // Ie Bike(this.gears)
                    let inst = objsys.get_this_instance_mut();
                    for i in 0..params.len() {
                        if params[i].fieldinit {
                            inst.set_field(params[i].name.clone(),args[i].clone());
                        }
                    }

                    // Run body
                    eval(&body, looktables, globals, store, objsys, ctx);

                    objsys.set_this(oldthis);
                    ctx.filepath = oldfilename;
                    println!("Resetting filepath to {}", ctx.filepath);

                    store.pop_call();

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


fn argnodes_to_argobjs(
    argnodes: &Vec<Node>,
    looktables: &HashMap<String, HashMap<String, usize>>,
    globals: &Vec<Node>,
    store: &mut Stack,
    objsys: &mut ObjSys,
    ctx: &mut Ctx) -> Vec<Object> {

    argnodes.iter().map(
        |argtree| eval(&argtree, looktables, globals, store, objsys, ctx)
    ).collect()
}
