use state::State;
use node::{NodeType, Node};
use builtin;
use utils::dart_evalerror;
use object::{Object, ParamObj};
use std::ops::{BitAnd, BitOr, BitXor};
use objsys::RefKey;


pub fn eval(
    node: &Node,
    state: &mut State,
    resolve: bool) -> Object {

    let t: &NodeType = &node.nodetype;

    match t {

        NodeType::Assign => {

            match &node.children[0].nodetype {
                NodeType::Name(name) => {

                    let left_obj = eval(&node.children[0], state, false);
                    let right_obj = eval(&node.children[1], state, true);

                    if state.stack.has(name) {
                        // If right_obj is a reference, then it should be cloned?
                        state.stack.add(name, right_obj);
                        return Object::Null;
                    }


                    if let Object::Reference(refid) = left_obj {
                        let left_ref = state.objsys.get_instance_mut(&refid);
                        left_ref.set_field(name.to_string(), right_obj);
                        return Object::Null;
                    }

                    if !state.objsys.has_this() {
                        // As dart.
                        dart_evalerror(format!("Setter not found: '{}'", name), state)
                    }
                    let this = state.objsys.get_this_instance_mut();

                    if !this.has_field(name.to_string()) {
                        // As dart.
                        dart_evalerror(format!("The setter '{}' isn't defined for the class '{}'", name, this.classname), state)
                    }
                    this.set_field(name.to_string(), right_obj);

                    return Object::Null;
                }
                NodeType::TypedVar(_, name) => {

                    let right_obj = eval(&node.children[1], state, true);

                    // TypedVar means we will allocate a new one on stack even if the name exists in a
                    // larger scope, like outside a loop or in a field. But fail if it's already on lex stack.
                    if state.stack.has_in_lexscope(name) {
                        // As dart.
                        dart_evalerror(format!("'{}' is already declared in this scope.", name), state);
                    }
                    state.stack.add(name, right_obj);

                    return Object::Null;
                }
                _ => panic!("Illegal name for assignment: {}", &node.children[0].nodetype)
            }
        }

        NodeType::Not => {

            let obj = eval(&node.children[0], state, true);

            return match obj {
                Object::Bool(b) => {
                    Object::Bool(!b)
                }
                _ => dart_evalerror(format!("Illegal right operand for !: {}", obj), state)                    
            }
        }

        NodeType::LogOr => {

            let left_obj = eval(&node.children[0], state, true);

            match left_obj {

                Object::Bool(b1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match right_obj {
                        Object::Bool(b2) => {
                            return Object::Bool(b1 || b2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for ||: {}", right_obj), state)                    
                    }
                }
                _ => dart_evalerror(format!("Illegal left operand for ||: {}", left_obj), state)                    
            }
        }

        NodeType::LogAnd => {

            let left_obj = eval(&node.children[0], state, true);

            match left_obj {

                Object::Bool(b1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match right_obj {
                        Object::Bool(b2) => {
                            return Object::Bool(b1 && b2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for &&: {}", right_obj), state)
                    }
                }
                _ => dart_evalerror(format!("Illegal left operand for &&: {}", left_obj), state)
            }
        }

        NodeType::LessThan => {

            let left_obj = eval(&node.children[0], state, true);

            match left_obj {

                Object::Int(n1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(n1 < n2)
                        }
                        Object::Double(x2) => {
                            return Object::Bool((n1 as f64) < x2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for comparison: {}", right_obj), state)
                    }
                }

                Object::Double(x1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(x1 < (n2 as f64))
                        }
                        Object::Double(x2) => {
                            return Object::Bool(x1 < x2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for comparison: {}", right_obj), state)
                    }
                }
                _ => dart_evalerror(format!("Illegal left operand for comparison: {}", left_obj), state)
            }
        }

        NodeType::GreaterThan => {

            let left_obj = eval(&node.children[0], state, true);

            match left_obj {

                Object::Int(n1) => {
                    let right_obj = eval(&node.children[1], state, true);

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
                    let right_obj = eval(&node.children[1], state, true);

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

            let left_obj = eval(&node.children[0], state, true);

            match left_obj {

                Object::Int(n1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(n1 <= n2)
                        }
                        Object::Double(x2) => {
                            return Object::Bool((n1 as f64) <= x2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for comparison: {}", right_obj), state)
                    }
                }

                Object::Double(x1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(x1 <= (n2 as f64))
                        }
                        Object::Double(x2) => {
                            return Object::Bool(x1 <= x2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for comparison: {}", right_obj), state)
                    }
                }
                _ => dart_evalerror(format!("Illegal left operand for comparison: {}", left_obj), state)
            }
        }

        NodeType::GreaterOrEq => {

            let left_obj = eval(&node.children[0], state, true);

            match left_obj {

                Object::Int(n1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(n1 >= n2)
                        }
                        Object::Double(x2) => {
                            return Object::Bool((n1 as f64) >= x2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for comparison: {}", right_obj), state)
                    }
                }

                Object::Double(x1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(x1 >= (n2 as f64))
                        }
                        Object::Double(x2) => {
                            return Object::Bool(x1 >= x2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for comparison: {}", right_obj), state)
                    }
                }
                _ => dart_evalerror(format!("Illegal left operand for comparison: {}", left_obj), state)
            }
        }

        NodeType::Equal => {

            let left_obj = eval(&node.children[0], state, true);
            let right_obj = eval(&node.children[1], state, true);

            return match left_obj {

                Object::Int(n1) => {
                    match right_obj {
                        Object::Int(n2) => Object::Bool(n1 == n2),
                        Object::Double(x2) => Object::Bool((n1 as f64) == x2),
                        _ => Object::Bool(false)
                    }
                }
                Object::Double(x1) => {
                    match right_obj {
                        Object::Int(n2) => Object::Bool(x1 == (n2 as f64)),
                        Object::Double(x2) => Object::Bool(x1 == x2),
                        _ => Object::Bool(false)
                    }
                }
                Object::Bool(b1) => {
                    match right_obj {
                        Object::Bool(b2) => Object::Bool(b1 == b2),
                        _ => Object::Bool(false)
                    }
                }
                Object::String(s1) => {
                    match right_obj {
                        Object::String(s2) => Object::Bool(s1 == s2),
                        _ => Object::Bool(false)
                    }
                }
                Object::Reference(k1) => {
                    match right_obj {
                        Object::Reference(k2) => Object::Bool(k1 == k2),
                        _ => Object::Bool(false)
                    }
                }
                _ => dart_evalerror(format!("Equality not implemented for object: {}", left_obj), state)
            }
        }

        NodeType::BitAnd => {

            let left_obj = eval(&node.children[0], state, true);

            match &left_obj {

                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1.bitand(s2))
                        }
                        _ => dart_evalerror(format!("Illegal right operand for bitwise and: {}", right_obj), state)
                    }
                }
                _ => panic!("Illegal left operand for bitwise and: {}", &left_obj)
            }
        }

        NodeType::BitOr => {

            let left_obj = eval(&node.children[0], state, true);

            match &left_obj {

                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1.bitor(s2))
                        }
                        _ => dart_evalerror(format!("Illegal right operand for bitwise or: {}", right_obj), state)
                    }
                }
                _ => dart_evalerror(format!("Illegal left operand for bitwise or: {}", left_obj), state)
            }
        }

        NodeType::BitXor => {

            let left_obj = eval(&node.children[0], state, true);

            match &left_obj {

                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1.bitxor(s2))
                        }
                        _ => dart_evalerror(format!("Illegal right operand for bitwise xor: {}", right_obj), state)
                    }
                }
                _ => dart_evalerror(format!("Illegal left operand for bitwise xor: {}", left_obj), state)
            }
        }

        NodeType::Add => {

            let left_obj = eval(&node.children[0], state, true);

            match &left_obj {
                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1 + s2)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 + s2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for addition: {}", right_obj), state)
                    }
                },
                Object::Double(s1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(s1 + *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(s1 + s2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for addition: {}", right_obj), state)
                    }
                }
                Object::String(s1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::String(s2) => {
                            let mut ret = s1.clone();
                            ret.push_str(s2);
                            return Object::String(ret);
                        }
                        _ => dart_evalerror(format!("Illegal right operand for addition: {}", right_obj), state)
                    }
                }
                _ => dart_evalerror(format!("Illegal left operand for addition: {}", left_obj), state)
            }
        }

        NodeType::Sub => {

            let left_obj = eval(&node.children[0], state, true);

            if node.children.len() == 1 {
                return match &left_obj {
                    Object::Int(n) => {
                        Object::Int(-*n)
                    }
                    Object::Double(x) => {
                        Object::Double(-*x)
                    }
                    _ => dart_evalerror(format!("Illegal operand for unary minus: {}", left_obj), state)
                }
            }

            match &left_obj {
                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1 - s2)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 - s2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for subtraction: {}", right_obj), state)
                    }
                },
                Object::Double(s1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(s1 - *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(s1 - s2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for subtraction: {}", right_obj), state)
                    }
                }
                _ => dart_evalerror(format!("Illegal left operand for subtraction: {}", left_obj), state)
            }
        }

        NodeType::Mul => {

            let left_obj = eval(&node.children[0], state, true);

            match &left_obj {
                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1 * s2)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 * s2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for multiplication: {}", right_obj), state)
                    }
                },
                Object::Double(s1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(s1 * *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(s1 * s2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for multiplication: {}", right_obj), state)
                    }
                }
                _ => dart_evalerror(format!("Illegal left operand for multiplication: {}", left_obj), state)
            }
        }

        NodeType::Div => {

            let left_obj = eval(&node.children[0], state, true);

            match &left_obj {
                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(*s1 as f64 / *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 / *s2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for division: {}", right_obj), state)
                    }
                },
                Object::Double(s1) => {
                    let right_obj = eval(&node.children[1], state, true);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(*s1 as f64 / *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 / *s2)
                        }
                        _ => dart_evalerror(format!("Illegal right operand for division: {}", right_obj), state)
                    }
                },
                _ => dart_evalerror(format!("Illegal left operand for divison: {}", left_obj), state)
            }
        }

        NodeType::PreIncrement => {

            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    if state.stack.has(s) {
                        let oldval = state.stack.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                state.stack.add(s.as_str(), newval.clone());
                                return newval;
                            }
                            _ => dart_evalerror(format!("Illegal operand for preincrement: {}", oldval), state)
                        }
                    }
                    else {
                        let this = state.objsys.get_this_instance_mut();
                        let oldval = this.get_field(s.clone()).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                this.set_field(s.clone(), newval.clone());
                                return newval;
                            }
                            _ => dart_evalerror(format!("Illegal operand for preincrement: {}", oldval), state)
                        }
                    }
                }
                _ => panic!("Illegal operand for preincrement: {}", valnode)
            }
        }

        NodeType::PreDecrement => {

            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    if state.stack.has(s) {
                        let oldval = state.stack.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                state.stack.add(s.as_str(), newval.clone());
                                return newval;
                            }
                            _ => dart_evalerror(format!("Illegal operand for predecrement: {}", oldval), state)
                        }
                    }
                    else {
                        let this = state.objsys.get_this_instance_mut();
                        let oldval = this.get_field(s.clone()).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                this.set_field(s.clone(), newval.clone());
                                return newval;
                            }
                            _ => dart_evalerror(format!("Illegal operand for predecrement: {}", oldval), state)
                        }
                    }
                }
                _ => dart_evalerror(format!("Illegal operand for predecrement: {}", valnode), state)
            }
        }

        NodeType::PostIncrement => {

            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    if state.stack.has(s) {
                        let oldval = state.stack.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                state.stack.add(s.as_str(), newval);
                                return oldval;
                            }
                            _ => dart_evalerror(format!("Illegal operand for increment: {}", oldval), state)
                        }
                    }
                    else {
                        let this = state.objsys.get_this_instance_mut();
                        let oldval = this.get_field(s.clone()).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                this.set_field(s.clone(), newval);
                                return oldval;
                            }
                            _ => dart_evalerror(format!("Illegal operand for increment: {}", oldval), state)
                        }
                    }
                }
                _ => dart_evalerror(format!("Illegal operand for increment: {:?}", valnode), state)
            }
        }

        NodeType::PostDecrement => {

            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s) => {

                    if state.stack.has(s) {
                        let oldval = state.stack.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                state.stack.add(s.as_str(), newval);
                                return oldval;
                            }
                            _ => dart_evalerror(format!("Illegal operand for decrement: {}", oldval), state)
                        }
                    }
                    else {
                        let this = state.objsys.get_this_instance_mut();
                        let oldval = this.get_field(s.clone()).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                this.set_field(s.clone(), newval);
                                return oldval;
                            }
                            _ => dart_evalerror(format!("Illegal operand for decrement: {}", oldval), state)
                        }
                    }
                }
                _ => dart_evalerror(format!("Illegal operand for decrement: {:?}", valnode), state)
            }
        }

        NodeType::Int(val) => {
            Object::Int(*val)
        },

        NodeType::Double(val) => {
            Object::Double(*val)
        },

        NodeType::Bool(v) => {
            Object::Bool(*v)
        },

        NodeType::Str(s) => {

            if node.children.is_empty() {
                return Object::String(s.clone())
            }

            let mut evaled_itps = Vec::new();
            for itp in &node.children {
                evaled_itps.push(eval(itp, state, true));
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

            // For Name, having a child means having an owner.
            if node.children.len() > 0 {

                // Run parent through the loop for lookup.
                let owner = eval(&node.children[0], state, true);

                // Owner is reference
                if let Object::Reference(refid) = owner.clone() {

                    if resolve {
                        let instance = state.objsys.get_instance(&refid);
                        return instance.get_field(s.to_string()).clone();
                    }
                    else {
                        // When evaluating left side of '=',
                        // we don't want to lookup the value.
                        // Return reference to the owner and then Assign will do
                        // set_field.
                        return owner;
                    }
                }

                panic!("Unexpected owner for {}: {}", s, owner);
            }

            if state.stack.has(s) {
                return state.stack.get(s).clone();
            }
            else if state.objsys.has_this() {
                let this = state.objsys.get_this_instance_mut();
                if !resolve {
                    return Object::Reference(this.id.clone());
                }
                return this.get_field(s.clone()).clone();
            }
            else {
                state.stack.printstack();
                // As dart.
                dart_evalerror(format!("Undefined name: '{}'.", s), state);
            }
        }

        NodeType::Return => {
            let retval = eval(&node.children[0], state, true);
            return Object::Return(Box::new(retval));
        }

        NodeType::MethodCall(name, owner, _filename) => {

            let reference: Object = eval(owner, state, true);

            if let Object::Reference(refid) = reference {
                let instance = state.objsys.get_instance(&refid);
                let c = state.objsys.get_class(&instance.classname);
                let meth_obj = c.get_method(name);

                return call_function(MaybeRef::Ref(refid), &meth_obj, &node.children[0], state)
            }
            panic!("Can't access {} of {}", name, owner);
        }

        NodeType::FunCall(s) => {

            // First look in stack.
            if state.stack.has(s) {
                let funcobj = state.stack.get(s).clone();

                return match funcobj {
                    Object::Function(_, _, _, _) => {
                        call_function(MaybeRef::None, &funcobj, &node.children[0], state)
                    }
                    Object::Constructor(_, _, _, _) => {
                        call_constructor(&funcobj, &node.children[0], state)
                    }
                    _ => panic!("Called non-callable.")
                }
            }

            // Next we look at other functions available from current file.
            let ltable = &state.looktables[&state.filepath];
            if ltable.contains_key(s) {

                let funcindex = ltable.get(s).unwrap();
                let funcnode = &state.globals[*funcindex];

                return match funcnode.nodetype {
                    NodeType::FunDef(_, _) => {
                        call_function(
                            MaybeRef::None,
                            &create_function(&funcnode),
                            &node.children[0],
                            state)
                    }
                    NodeType::Constructor(_, _) => {
                        call_constructor(
                            &create_constructor(&funcnode),
                            &node.children[0],
                            state)
                    }
                    _ => panic!("Expected function definition or constructor.")
                }
            }

            // Third we check if we have a built-in function.
            if builtin::has_function(s) {
                let mut args = argnodes_to_argobjs(
                    &node.children[0].children,
                    state);
                return builtin::call(s, &mut args, state);
            }

            dart_evalerror("Function not found.", state)
        }

        NodeType::FunDef(s, _) => {
            let funcobj = create_function(node);
            state.stack.add(s, funcobj);
            return Object::Null;
        }

        NodeType::Conditional => {

            for condnode in &node.children {

                match condnode.nodetype {

                    NodeType::If |
                    NodeType::ElseIf => {
                        let boolnode= &condnode.children[0];

                        let cond = eval(&boolnode, state, true);
                        match cond {

                            Object::Bool(v) => {
                                if v {
                                    let bodynode= &condnode.children[1];
                                    state.stack.push_lex();
                                    let ret = eval(&bodynode, state, true);
                                    state.stack.pop_lex();
                                    return ret;
                                }
                            }
                            _ => panic!("Expected bool in conditional")
                        }
                    }

                    NodeType::Else => {
                        let bodynode= &condnode.children[0];
                        state.stack.push_lex();
                        let ret = eval(&bodynode, state, true);
                        state.stack.pop_lex();
                        return ret;
                    }
                    _ => panic!("Invalid node in conditional!")

                }
            }
            return Object::Null;
        }

        NodeType::While => {

            let boolnode = &node.children[0];
            let block = &node.children[1];
            let mut cond = eval(boolnode, state, true);

            match &cond {

                Object::Bool(mut v) => {

                    while v {
                        eval(block, state, true);
                        cond = eval(boolnode, state, true);

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

            let block = &node.children[0];
            let boolnode = &node.children[1];

            eval(block, state, true);

            let mut cond = eval(boolnode, state, true);

            if let Object::Bool(mut b) = cond {

                while b {
                    eval(block, state, true);
                    cond = eval(boolnode, state, true);

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

            let assign = &node.children[0];
            let condexpr = &node.children[1];
            let mutexpr = &node.children[2];
            let body = &node.children[3];

            eval(assign, state, true);

            loop {
                let condobj = eval(condexpr, state, true);

                match condobj {
                    Object::Bool(b) => {

                        if !b {
                            break;
                        }
                        eval(body, state, true);
                        eval(mutexpr, state, true);
                    }
                    x => dart_evalerror(format!("Expected bool. Got: {}", x), state)
                }
            }
            return Object::Null;
        }

        NodeType::Block => {

            for c in &node.children {

                state.stack.garbagecollect(&mut state.objsys, &state.constructing);

                let retval = eval(c, state, true);

                match &retval {
                    Object::Return(_) => {
                        return retval;
                    }
                    _ => {}
                }
            }
            return Object::Null;
        }

        NodeType::List => {

            let class = state.objsys.get_class("List");
            let mut inst = class.instantiate();
            
            let mut vals: Vec<Object> = Vec::new();
            for c in &node.children {
                let v = eval(c, state, true);
                vals.push(v);
            }
            let ilist = Object::__InternalList(vals);
            inst.set_field(String::from("__list"), ilist);

            let instref = state.objsys.register_instance(inst);
            return instref;
        }

        NodeType::Null => {
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


pub enum MaybeRef {
    None,
    Ref(RefKey)
}


pub fn call_function(
    instance: MaybeRef,
    func: &Object,
    args: &Node,
    state: &mut State) -> Object {
    
    match func {

        Object::Function(funcname, filename, body, params) => {

            if args.children.len() != params.len() {
                dart_evalerror(format!("In method call {}, {} arguments expected but {} given.",
                funcname, params.len(), args.children.len()), state);
            }
    
            let mut argobjs = argnodes_to_argobjs(
                &args.children,
                state
            );
    
            // Argtrees must be evaluated in the callers context,
            // but stored in the new call frame.
    
            state.stack.push_call();
            for i in 0 .. params.len() {
                state.stack.add(params[i].name.as_str(), argobjs.pop().unwrap());
            }
    
            let oldfilename = state.filepath.clone();
            state.filepath = filename.clone();

            let mut oldthis = MaybeRef::None;
            if let MaybeRef::Ref(rk) = instance {
                oldthis = MaybeRef::Ref(state.objsys.get_this());
                state.objsys.set_this(rk);
            }
    
            let result = eval(&body, state, true);
    
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
    state: &mut State) -> Object {

    match funcobj {

        Object::Constructor(cname, filename, body, params) => {

            let mut args = argnodes_to_argobjs(
                &args.children,
                state
            );

            // Argtrees must be evaluated in callers context, but stored in new context.

            state.stack.push_call();
            for i in 0..params.len() {
                // Field initializers are set directly on the instance. See below.
                if !params[i].fieldinit {
                    state.stack.add(params[i].name.as_str(), args.remove(i));
                }
            }

            // Make an instance.

            let class = state.objsys.get_class(cname.as_str());
            let mut inst = class.instantiate();

            // Evaluate the initial field values.

            let field_nodes = class.fields.clone();
            for (_, fname, initexpr) in &field_nodes {
                inst.set_field(fname.clone(), eval(initexpr, state, true));
            }

            let instref = state.objsys.register_instance(inst);

            // Run the constructor body.

            match &instref {
                Object::Reference(refid) => {

                    state.constructing.push(refid.clone());
                    let oldfilename = state.filepath.clone();
                    state.filepath = filename.clone();
                    let oldthis = state.objsys.get_this();
                    state.objsys.set_this(refid.clone());

                    // Set fields from params that uses "this" to auto-init.
                    // Ie Bike(this.gears)
                    let inst = state.objsys.get_this_instance_mut();
                    for i in 0..params.len() {
                        if params[i].fieldinit {
                            inst.set_field(params[i].name.clone(),args[i].clone());
                        }
                    }

                    // Run body
                    eval(&body, state, true);

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


fn argnodes_to_argobjs(
    argnodes: &Vec<Node>,
    state: &mut State) -> Vec<Object> {

    argnodes.iter().map(
        |argtree| eval(&argtree, state, true)
    ).collect()
}
