use node::{NodeType, Node};
use builtin;
use utils::{dprint, dart_evalerror};
use stack::Stack;
use object::Object;
use objsys::Class;
use objsys::ClassMap;
use objsys::InstanceMap;
use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, BitXor};


// Find functions that are direct children of 'node'
// and add them to the store for later lookup.
pub fn preval(
    node: &Node,
    globals: &mut HashMap<String, Object>,
    store: &mut Stack,
    classlist: &mut ClassMap,
    instlist: &mut InstanceMap) {

    dprint(" ");
    dprint("PREVAL");
    dprint(" ");

    for n in &node.children {
        let t: &NodeType = &n.nodetype;

        match t {
            NodeType::FunDef(fname) => {
                dprint(format!("Preval: NodeType::FunDef '{}'", fname));

                let params = &n.children[0];
                dprint(format!("{}", params));

                let body = n.children[1].clone();

                if let NodeType::ParamList = params.nodetype {

                    let mut args: Vec<String> = Vec::new();

                    for i in 0..params.children.len() {
                        let p = &params.children[i];
                        match &p.nodetype {
                            NodeType::Name(s) => {
                                args.push(s.clone());
                            }
                            x => panic!("Invalid parameter: {}", x)
                        }
                    }

                    let obj = Object::Function(fname.to_string(), body, args);
                    // store.add(fname, obj);
                    globals.insert(fname.to_string(), obj);

                    dprint(format!("Inserted to symtable: {}", fname));

                } else {
                    panic!("Expected paramlist for FunDef in preeval.");
                }


            }
            NodeType::Class(cname) => {
                let mut class = Class::new(cname.clone());
                preval_class(&mut class, globals, store,n, classlist, instlist);

                classlist.add(class);

            }
            x => {
                dprint(format!("Preval considering node {}", x));
            }
        }
    }
}


fn preval_class(
    classobj: &mut Class,
    globals: &mut HashMap<String, Object>,
    store: &mut Stack, classnode: &Node,
    classlist: &mut ClassMap,
    instlist: &mut InstanceMap) {

    for member in &classnode.children {
        let t: &NodeType = &member.nodetype;

        match t {
            NodeType::FunDef(fname) => {
                dprint(format!("Preval: NodeType::FunDef '{}'", fname));

                let params = &member.children[0];
                dprint(format!("{}", params));

                let body = member.children[1].clone();

                if let NodeType::ParamList = params.nodetype {
                    let mut args: Vec<String> = Vec::new();

                    for i in 0..params.children.len() {
                        let p = &params.children[i];
                        match &p.nodetype {
                            NodeType::Name(s) => {
                                args.push(s.clone());
                            }
                            x => panic!("Invalid parameter: {}", x)
                        }
                    }

                    let obj = Object::Function(fname.to_string(), body, args);
                    classobj.add_method(fname.clone(), obj);
                } else {
                    panic!("Expected paramlist for FunDef in preeval.");

                }


            }
            NodeType::Assign => {
                let namenode = member.children[0].clone();
                if let NodeType::TypedVar(ftype, fname) = namenode.nodetype {

                    // What to do with store here?
                    // Field can reference some stuff.
                    let val = eval(&member.children[1], globals, store, classlist, instlist);

                    classobj.add_field(ftype, fname, val);
                }
                else {
                    panic!("Illegal left node in assignment.")
                }
            }
            NodeType::Constructor(cname) => {

                let params = &member.children[0];
                dprint(format!("{}", params));

                let body = member.children[1].clone();

                if let NodeType::ParamList = params.nodetype {

                    let mut args: Vec<String> = Vec::new();

                    for i in 0..params.children.len() {
                        let p = &params.children[i];
                        match &p.nodetype {
                            NodeType::Name(s) => {
                                args.push(s.clone());
                            }
                            x => panic!("Invalid parameter: {}", x)
                        }
                    }

                    let obj = Object::Constructor(cname.to_string(), body, args);

                    globals.insert(cname.to_string(), obj);

                } else {
                    panic!("Expected paramlist for Constructor in preeval.");
                }
            }
            x => {
                dprint(format!("preval_class considering node {}", x));
            }
        }
    }
}


pub fn eval(
    node: &Node, globals: &mut HashMap<String, Object>,
    store: &mut Stack,
    classlist: &mut ClassMap,
    instlist: &mut InstanceMap) -> Object {

    let t: &NodeType = &node.nodetype;

    match t {

        NodeType::Assign => {
            dprint("Eval: NodeType::Assign");
            match &node.children[0].nodetype {
                NodeType::Name(ref s1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

                    if store.has(s1.as_str()) {
                        store.add(s1.as_str(), right_obj);
                    }
                    else {
                        let this = instlist.get_this();
                        this.set_field(s1.clone(), right_obj);
                    }
                    return Object::Null;
                }
                NodeType::TypedVar(_, ref s1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

                    if store.has_in_lexscope(s1.as_str()) {
                        // As dart.
                        dart_evalerror(format!("'{}' is already declared in this scope.", s1));
                    }
                    else {
                        if instlist.has_this() {
                            let this = instlist.get_this();
                            if this.has_field(s1.clone()) {
                                panic!("Variable with name {} already exists.", s1);
                            }
                        }
                        store.add(s1.as_str(), right_obj);
                    }

                    return Object::Null;
                }
                _ => panic!("Illegal name for assignment: {}", &node.children[0].nodetype)
            }
        }

        NodeType::Not => {
            dprint("Eval: NodeType::Not");

            let obj = eval(&node.children[0], globals, store, classlist, instlist);

            return match obj {
                Object::Bool(b) => {
                    Object::Bool(!b)
                }
                _ => panic!("Illegal operand for '!'")
            }
        }

        NodeType::LogOr => {
            dprint("Eval: NodeType::LogOr");

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match left_obj {

                Object::Bool(b1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match left_obj {

                Object::Bool(b1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match left_obj {

                Object::Int(n1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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
                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match left_obj {

                Object::Int(n1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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
                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match left_obj {

                Object::Int(n1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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
                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match left_obj {

                Object::Int(n1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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
                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);
            let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match &left_obj {

                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match &left_obj {

                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match &left_obj {

                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match &left_obj {
                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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
                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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
                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

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

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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
                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match &left_obj {
                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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
                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

            let left_obj = eval(&node.children[0], globals, store, classlist, instlist);

            match &left_obj {
                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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

                    let right_obj = eval(&node.children[1], globals, store, classlist, instlist);

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
                        let this = instlist.get_this();
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
                        let this = instlist.get_this();
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
                        let this = instlist.get_this();
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
                        let this = instlist.get_this();
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

        NodeType::Int(s) => {
            dprint("Eval: NodeType::Int");
            Object::Int(s.parse().unwrap())
        },

        NodeType::Double(s) => {
            dprint("Eval: NodeType::Double");
            Object::Double((s.as_str()).parse::<f64>().unwrap())
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
                evaled_itps.push(eval(itp, globals, store, classlist, instlist));
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
            if store.has(s) {
                dprint(format!("got value for {}", s));
                return store.get(s).clone();
            }
            else if instlist.has_this() {
                let this = instlist.get_this();
                return this.get_field(s.clone()).clone();
            }
            else {
                store.printstack();
                // As dart.
                dart_evalerror(format!("Undefined name: '{}'.", s));
            }
        }

        NodeType::Return => {
            dprint(format!("Eval: NodeType::Return"));
            let retval = eval(&node.children[0], globals, store, classlist, instlist);
            return Object::Return(Box::new(retval));
        }

        NodeType::MethodCall(qname, methname) => {
            dprint(format!("objname: {}", qname));

            let reference = store.get(qname);
            if let Object::Reference(refid) = reference {

                let instance = instlist.get(refid);
                let c = classlist.get(&instance.classname);


                let meth = c.get_method(methname);
                if let Object::Function(_, node, params) = meth {

                    let argslist = &node.children[0];

                    store.push_call();
                    instlist.this = instance.id.clone();


                    for i in 0 .. params.len() {

                        let argtree = &argslist.children[i];
                        dprint(format!("about to eval method argtree: {}", argtree));

                        let argobj = eval(argtree, globals, store, classlist, instlist);
                        store.add(params[i].as_str(), argobj);

                    }

                    let result = eval(&node, globals, store, classlist, instlist);

                    instlist.this = String::from("");
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
            panic!("Can't access {} of {}", qname, methname)


        }

        NodeType::FunCall(s) => {
            dprint(format!("Eval: NodeType::FunCall({})", s));

            let funcobj : Object;
            if store.has(s) {
                funcobj = store.get(s).clone();
            }
            else if globals.contains_key(s) {
                funcobj = globals.get(s).unwrap().clone();
            }
            else if builtin::has_function(s) {
                let mut args: Vec<Object> = Vec::new();

                for argtree in &node.children[0].children {
                    args.push(eval(&argtree, globals, store, classlist, instlist));
                }

                let res: Object = builtin::call(s, &args);
                return res;
            }
            else {
                panic!("Unknown function: {}", s)
            }

            match &funcobj {
                Object::Function(fname, body, params) => {

                    let argslist = &node.children[0];

                    let mut evaled_args : Vec<Object> = Vec::new();
                    for i in 0 .. params.len() {
                        dprint(format!("about to eval function({}) argtree {}: {}", fname, i, params[i]));

                        let argtree = &argslist.children[i];
                        let argobj = eval(argtree, globals, store, classlist, instlist);
                        evaled_args.push(argobj);
                    }

                    // Argtrees must be evaluated in callers context, but stored in new context.

                    store.push_call();
                    for i in 0 .. params.len() {
                        store.add(params[i].as_str(), evaled_args.remove(0));
                    }

                    let result = eval(&body, globals, store, classlist, instlist);

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
                Object::Constructor(cname, body, params) => {

                    let argslist = &node.children[0];

                    // store.push_call();
                    let mut evaled_args : Vec<Object> = Vec::new();
                    for i in 0 .. params.len() {
                        dprint(format!("about to eval argtree {}: {}", i, params[i]));

                        let argtree = &argslist.children[i];
                        let argobj = eval(argtree, globals, store, classlist, instlist);
                        evaled_args.push(argobj);
                    }

                    // Argtrees must be evaluated in callers context, but stored in new context.

                    store.push_call();
                    for i in 0 .. params.len() {
                        store.add(params[i].as_str(), evaled_args.remove(0));
                    }

                    // Lookup class.
                    let class = classlist.get(cname.as_str());

                    // Creates instance with data fields.
                    let instref = class.instantiate(instlist);

                    match &instref {
                        Object::Reference(refid) => {
                            instlist.this = refid.clone();

                            // Run body
                            eval(&body, globals, store, classlist, instlist);

                            instlist.this = String::from("");

                            store.pop_call();

                            return instref.clone();
                        }
                        _ => panic!("Couldn't find intance that was just created.")
                    }
                }
                _ => panic!("Called a non function object")
            }

        }

        NodeType::FunDef(s) => {
            dprint("Eval: NodeType::FunDef");

            let params = &node.children[0];
            let body = node.children[1].clone();

            if let NodeType::ParamList = params.nodetype {

                let mut args: Vec<String> = Vec::new();

                for i in 0 .. params.children.len() {
                    let p = &params.children[i];
                    match &p.nodetype {
                        NodeType::Name(s) => {
                            args.push(s.clone());
                        }
                        x => panic!("Invalid parameter: {}", x)
                    }
                }

                let obj = Object::Function(s.to_string(), body, args);

                store.add(s, obj);
                return Object::Null;
            } else {
                panic!("Expected paramlist for FunDef in eval.");
            }
        }

        NodeType::Conditional => {
            dprint("Eval: NodeType::Conditional");

            for condnode in &node.children {

                match condnode.nodetype {

                    NodeType::If |
                    NodeType::ElseIf => {
                        let boolnode= &condnode.children[0];

                        let cond = eval(&boolnode, globals, store, classlist, instlist);
                        match cond {

                            Object::Bool(v) => {
                                if v {
                                    let bodynode= &condnode.children[1];
                                    store.push_lex();
                                    let ret = eval(&bodynode, globals, store, classlist, instlist);
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
                        let ret = eval(&bodynode, globals, store, classlist, instlist);
                        store.pop_lex();
                        return ret;
                    }
                    _ => panic!("Invalid node in conditional!")

                }
            }

            return Object::Null;
        }

        NodeType::Block => {
            dprint("Eval: NodeType::Block");

            for c in &node.children {

                let retval = eval(c, globals, store, classlist, instlist);

                match &retval {
                    Object::Return(_) => {
                        return retval;
                    }
                    _ => {}
                }
            }
            return Object::Null;
        }

        NodeType::Module => {
            dprint("Eval: NodeType::Module");

            eval(&node.children[1], globals, store, classlist, instlist)
        }

        _ => panic!("Unknown node type: {}", t)
    }
}

