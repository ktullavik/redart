use std::ops::{BitAnd, BitOr, BitXor};
use std::time::Duration;
use std::time::Instant;
use crate::state::State;
use crate::node::{NodeType, Node};
use crate::builtin;
use crate::error::evalerror;
use crate::object::Object;
use crate::evalhelp::*;
use crate::heapobjs::internallist::InternalList;


static GC_TIME: Duration = Duration::from_micros(400);


pub fn eval(
    node: &Node,
    state: &mut State) -> Object {

    let t: &NodeType = &node.nodetype;

    match t {

        NodeType::Assign(_, _) => {

            let right_obj = eval(&node.children[1], state);

            match &node.children[0].nodetype {
                NodeType::Name(name, linenum, symnum) => {

                    if node.children[0].children.len() > 0 {
                        let left_obj = eval(&node.children[0].children[0], state);
                        set_field(left_obj, name, right_obj, state, &node.children[0]);
                        return Object::Null;
                    }

                    // Look on the stack.
                    if state.stack.has(name) {
                        state.stack.update(name, right_obj);
                        return Object::Null;
                    }

                    // Look in 'this' instance.
                    if state.objsys.has_this() {
                        let this = state.objsys.get_this_instance_mut();
                        if this.has_field(name) {
                            set_field(state.objsys.get_this_object(), name, right_obj, state, &node.children[0]);
                            return Object::Null;
                        }
                    }

                    // Look for globals.
                    if state.has_global(name) {

                        let n = state.get_global_ref(name);

                        match &n.nodetype {

                            NodeType::TopVarLazy(typ, _, _, _) |
                            NodeType::TopVar(typ, _, _, _, _) => {
                                let newval = Node::new(
                                    NodeType::TopVar(
                                        typ.clone(),
                                        name.clone(),
                                        Box::new(right_obj),
                                        linenum.clone(),
                                        symnum.clone()
                                    )
                                );
                                state.set_global(name, newval);
                                return Object::Null;
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
                        &node.children[0]
                    )
                }
                NodeType::TypedVar(_, name, _, _) => {
                    // TypedVar means we will allocate a new one on stack even if the name exists in a
                    // larger scope, like outside a loop or in a field. But fail if it's already on lex stack.
                    if state.stack.has_in_lexscope(name) {
                        // As dart.
                        evalerror(
                            format!("'{}' is already declared in this scope.", name),
                            state,
                            &node.children[0]
                        );
                    }
                    state.stack.add_new(name, right_obj);

                    return Object::Null;
                }

                NodeType::CollAccess(_, _) => {

                    match &node.children[0].children[0].nodetype {

                        NodeType::Name(name, linenum, symnum) => {

                            // Look on the stack.
                            if state.stack.has(name) {
                                let ulist_ref = state.stack.get(name).clone();
                                let index = eval(&node.children[0].children[1], state);
                                set_list_element(
                                    ulist_ref,
                                    index,
                                    right_obj,
                                    state,
                                    &node.children[0].children[0],
                                    &node.children[0].children[1]
                                );
                                return Object::Null;
                            }

                            // Look in 'this' instance.
                            if state.objsys.has_this() {

                                let this = state.objsys.get_this_instance_mut();

                                if this.has_field(name) {
                                    let ulist_ref = this.get_field(name);
                                    let index = eval(&node.children[0].children[1], state);
                                    set_list_element(
                                        ulist_ref,
                                        index,
                                        right_obj,
                                        state,
                                        &node.children[0].children[0],
                                        &node.children[0].children[1]
                                    );
                                    return Object::Null;
                                }
                                else {
                                    println!("No field: {}", name)
                                }
                            }

                            // Look for globals.
                            if state.has_global(name) {

                                let n = state.get_global(name);

                                match &n.nodetype {

                                    NodeType::TopVarLazy(typ, topname, _, _) => {
                                        // Eval lazy and replace.

                                        if *topname == state.eval_var {
                                            evalerror(format!("Top level variable '{}' depends on itself.", topname), state, node);
                                        }
                                        if state.eval_var.len() == 0 {
                                            state.eval_var = topname.clone();
                                        }

                                        let compval = eval(&n.children[0], state);

                                        let wrapped = Node::new(NodeType::TopVar(
                                            typ.clone(),
                                            topname.clone(),
                                            Box::new(compval),
                                            linenum.clone(),
                                            symnum.clone()
                                        ));
                                        state.eval_var = String::from("");
                                        state.set_global(topname.as_str(), wrapped);

                                        let ulist_ref = eval(&node.children[0].children[0], state);
                                        let index = eval(&node.children[0].children[1], state);
                                        set_list_element(
                                            ulist_ref,
                                            index,
                                            right_obj,
                                            state,
                                            &n,
                                            &node.children[0].children[1]
                                        );
                                    }

                                    NodeType::TopVar(_,  _, _, _, _) => {
                                        let ulist_ref = eval(&node.children[0].children[0], state);
                                        let index = eval(&node.children[0].children[1], state);
                                        set_list_element(
                                            ulist_ref,
                                            index,
                                            right_obj,
                                            state,
                                            &n,
                                            &node.children[0].children[1]
                                        );
                                    }
                                    NodeType::ConstTopLazy(_, _, _, _) |
                                    NodeType::ConstTopVar(_, _, _, _, _) => {
                                        // As dart.
                                        evalerror(
                                            "Unsupported operation: Cannot modify an unmodifiable list",
                                            state,
                                            node
                                        )
                                    }
                                    _ => panic!("Unexpected node type in globals: {}", n)
                                }
                                return Object::Null;
                            }
                        }
                        x => evalerror(
                            format!("Unexpected token: {}", x),
                            state,
                            &node.children[0].children[0]
                        )
                    }
                    return Object::Null;
                }
                _ => panic!("Illegal name for assignment: {}", &node.children[0].nodetype)
            }
        }

        NodeType::Not(_, _) => {

            let obj = eval(&node.children[0], state);

            return match obj {
                Object::Bool(b) => {
                    Object::Bool(!b)
                }
                _ => evalerror(
                    format!("Illegal operand for !: {}", obj),
                    state,
                    &node.children[0]
                )                    
            }
        }

        NodeType::LogOr(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match left_obj {

                Object::Bool(b1) => {
                    let right_obj = eval(&node.children[1], state);

                    match right_obj {
                        Object::Bool(b2) => {
                            return Object::Bool(b1 || b2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for ||: {}", right_obj),
                            state,
                            &node.children[1]
                        )                    
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for ||: {}", left_obj),
                    state,
                    &node.children[0]
                )                    
            }
        }

        NodeType::LogAnd(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match left_obj {

                Object::Bool(b1) => {
                    let right_obj = eval(&node.children[1], state);

                    match right_obj {
                        Object::Bool(b2) => {
                            return Object::Bool(b1 && b2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for &&: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for &&: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::LessThan(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match left_obj {

                Object::Int(n1) => {
                    let right_obj = eval(&node.children[1], state);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(n1 < n2)
                        }
                        Object::Double(x2) => {
                            return Object::Bool((n1 as f64) < x2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for comparison: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }

                Object::Double(x1) => {
                    let right_obj = eval(&node.children[1], state);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(x1 < (n2 as f64))
                        }
                        Object::Double(x2) => {
                            return Object::Bool(x1 < x2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for comparison: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for comparison: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::GreaterThan(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match left_obj {

                Object::Int(n1) => {
                    let right_obj = eval(&node.children[1], state);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(n1 > n2)
                        }
                        Object::Double(x2) => {
                            return Object::Bool((n1 as f64) > x2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for comparison: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }

                Object::Double(x1) => {
                    let right_obj = eval(&node.children[1], state);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(x1 > (n2 as f64))
                        }
                        Object::Double(x2) => {
                            return Object::Bool(x1 > x2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for comparison: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for comparison: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::LessOrEq(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match left_obj {

                Object::Int(n1) => {
                    let right_obj = eval(&node.children[1], state);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(n1 <= n2)
                        }
                        Object::Double(x2) => {
                            return Object::Bool((n1 as f64) <= x2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for comparison: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }

                Object::Double(x1) => {
                    let right_obj = eval(&node.children[1], state);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(x1 <= (n2 as f64))
                        }
                        Object::Double(x2) => {
                            return Object::Bool(x1 <= x2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for comparison: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for comparison: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::GreaterOrEq(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match left_obj {

                Object::Int(n1) => {
                    let right_obj = eval(&node.children[1], state);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(n1 >= n2)
                        }
                        Object::Double(x2) => {
                            return Object::Bool((n1 as f64) >= x2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for comparison: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }

                Object::Double(x1) => {
                    let right_obj = eval(&node.children[1], state);

                    match right_obj {
                        Object::Int(n2) => {
                            return Object::Bool(x1 >= (n2 as f64))
                        }
                        Object::Double(x2) => {
                            return Object::Bool(x1 >= x2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for comparison: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for comparison: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::Equal(_, _) => {

            let left_obj = eval(&node.children[0], state);
            let right_obj = eval(&node.children[1], state);

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
                _ => evalerror(
                    format!("Equality not implemented for object: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::BitAnd(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match &left_obj {

                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1.bitand(s2))
                        }
                        _ => evalerror(
                            format!("Illegal right operand for bitwise and: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for bitwise and: {}", &left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::BitOr(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match &left_obj {

                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1.bitor(s2))
                        }
                        _ => evalerror(
                            format!("Illegal right operand for bitwise or: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for bitwise or: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::BitXor(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match &left_obj {

                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1.bitxor(s2))
                        }
                        _ => evalerror(
                            format!("Illegal right operand for bitwise xor: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for bitwise xor: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::Add(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match &left_obj {
                Object::Int(s1) => {

                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1 + s2)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 + s2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for addition: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                },
                Object::Double(s1) => {
                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(s1 + *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(s1 + s2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for addition: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                Object::String(s1) => {
                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::String(s2) => {
                            let mut ret = s1.clone();
                            ret.push_str(s2);
                            return Object::String(ret);
                        }
                        _ => evalerror(
                            format!("Illegal right operand for addition: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for addition: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::Sub(_, _) => {

            let left_obj = eval(&node.children[0], state);

            if node.children.len() == 1 {
                return match &left_obj {
                    Object::Int(n) => {
                        Object::Int(-*n)
                    }
                    Object::Double(x) => {
                        Object::Double(-*x)
                    }
                    _ => evalerror(
                        format!("Illegal operand for unary minus: {}", left_obj),
                        state,
                        &node.children[0]
                    )
                }
            }

            match &left_obj {
                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1 - s2)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 - s2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for subtraction: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                },
                Object::Double(s1) => {
                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(s1 - *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(s1 - s2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for subtraction: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for subtraction: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::Mul(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match &left_obj {
                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Int(s1 * s2)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 * s2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for multiplication: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                },
                Object::Double(s1) => {
                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(s1 * *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(s1 * s2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for multiplication: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                }
                _ => evalerror(
                    format!("Illegal left operand for multiplication: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::Div(_, _) => {

            let left_obj = eval(&node.children[0], state);

            match &left_obj {
                Object::Int(s1) => {
                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(*s1 as f64 / *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 / *s2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for division: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                },
                Object::Double(s1) => {
                    let right_obj = eval(&node.children[1], state);

                    match &right_obj {
                        Object::Int(s2) => {
                            Object::Double(*s1 as f64 / *s2 as f64)
                        }
                        Object::Double(s2) => {
                            Object::Double(*s1 as f64 / *s2)
                        }
                        _ => evalerror(
                            format!("Illegal right operand for division: {}", right_obj),
                            state,
                            &node.children[1]
                        )
                    }
                },
                _ => evalerror(
                    format!("Illegal left operand for divison: {}", left_obj),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::PreIncrement(_, _) => {

            // FIXME, needs eval
            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s, _, _) => {

                    if state.stack.has(s) {
                        let oldval = state.stack.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                state.stack.update(s.as_str(), newval.clone());
                                return newval;
                            }
                            _ => evalerror(
                                format!("Illegal operand for preincrement: {}", oldval),
                                state,
                                &node.children[0]
                            )
                        }
                    }
                    else {
                        let oldval = get_field(
                            state.objsys.get_this_object(), 
                            s,
                            state,
                            valnode
                        );

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                set_field(
                                    state.objsys.get_this_object(),
                                    s,
                                    newval.clone(),
                                    state,
                                    valnode
                                );
                                return newval;
                            }
                            _ => evalerror(
                                format!("Illegal operand for preincrement: {}", oldval),
                                state,
                                &node.children[0]
                            )
                        }
                    }
                }
                _ => evalerror(
                    format!("Illegal operand for preincrement: {}", valnode),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::PreDecrement(_, _) => {

            // FIXME, needs eval
            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s, _, _) => {

                    if state.stack.has(s) {
                        let oldval = state.stack.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                state.stack.update(s.as_str(), newval.clone());
                                return newval;
                            }
                            _ => evalerror(
                                format!("Illegal operand for predecrement: {}", oldval),
                                state,
                                &node.children[0]
                            )
                        }
                    }
                    else {
                        let oldval = get_field(
                            state.objsys.get_this_object(),
                            s,
                            state,
                            valnode
                        );

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                set_field(
                                    state.objsys.get_this_object(),
                                    s,
                                    newval.clone(),
                                    state,
                                    valnode
                                );
                                return newval;
                            }
                            _ => evalerror(
                                format!("Illegal operand for predecrement: {}", oldval),
                                state,
                                &node.children[0]
                            )
                        }
                    }
                }
                _ => evalerror(
                    format!("Illegal operand for predecrement: {}", valnode),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::PostIncrement(_, _) => {

            // FIXME, needs eval
            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s, _, _) => {

                    if state.stack.has(s) {
                        let oldval = state.stack.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                state.stack.update(s.as_str(), newval);
                                return oldval;
                            }
                            _ => evalerror(
                                format!("Illegal operand for increment: {}", oldval),
                                state,
                                &node.children[0]
                            )
                        }
                    }
                    else {
                        let oldval = get_field(
                            state.objsys.get_this_object(), 
                            s,
                            state,
                            valnode
                        );

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n + 1);
                                set_field(
                                    state.objsys.get_this_object(),
                                    s,
                                    newval.clone(),
                                    state,
                                    valnode
                                );
                                return oldval;
                            }
                            _ => evalerror(
                                format!("Illegal operand for increment: {}", oldval),
                                state,
                                &node.children[0]
                            )
                        }
                    }
                }
                _ => evalerror(
                    format!("Illegal operand for increment: {}", valnode),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::PostDecrement(_, _) => {

            // FIXME, needs eval
            let valnode = &node.children[0];

            match valnode.nodetype {
                NodeType::Name(ref s, _, _) => {

                    if state.stack.has(s) {
                        let oldval = state.stack.get(s).clone();

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                state.stack.update(s.as_str(), newval);
                                return oldval;
                            }
                            _ => evalerror(
                                format!("Illegal operand for decrement: {}", oldval),
                                state,
                                &node.children[0]
                            )
                        }
                    }
                    else {
                        let oldval = get_field(
                            state.objsys.get_this_object(),
                            s,
                            state,
                            valnode
                        );

                        match oldval {
                            Object::Int(n) => {
                                let newval = Object::Int(n - 1);
                                set_field(
                                    state.objsys.get_this_object(),
                                    s,
                                    newval.clone(),
                                    state,
                                    valnode
                                );
                                return oldval;
                            }
                            _ => evalerror(
                                format!("Illegal operand for decrement: {}", oldval),
                                state,
                                &node.children[0]
                            )
                        }
                    }
                }
                _ => evalerror(
                    format!("Illegal operand for decrement: {}", valnode),
                    state,
                    &node.children[0]
                )
            }
        }

        NodeType::Int(val, _, _) => {
            Object::Int(*val)
        },

        NodeType::Double(val, _, _) => {
            Object::Double(*val)
        },

        NodeType::Bool(v, _, _) => {
            Object::Bool(*v)
        },

        NodeType::Str(s, _, _) => {

            if node.children.is_empty() {
                return Object::String(s.clone())
            }

            let mut evaled_itps = Vec::new();
            for itp in &node.children {
                evaled_itps.push(eval(itp, state));
            }

            let parts : Vec<&str> = s.as_str().split("$").collect();

            let mut built : String = String::new();

            for i in 0 .. evaled_itps.len() {

                // If it's a reference, call the toString method on the object.
                if let Object::Reference(rk) = &evaled_itps[i] {
                    let inst = state.objsys.get_instance(&rk);
                    let c = state.objsys.get_class(&inst.classname);
                    let meth_obj = c.get_method("toString", state, &node.children[i]);
                    let str_obj = call_function(
                        MaybeRef::Ref(rk.clone()), 
                        &meth_obj, 
                        &node.children[0], 
                        state
                    );
                    built = format!("{}{}{}", parts[0], built.clone(), str_obj);
                }
                else {
                    built = format!("{}{}{}", parts[0], built.clone(), evaled_itps[i].clone());
                }
            }
            built.push_str(parts.last().unwrap());

            return Object::String(built)
        },

        NodeType::Name(s, linenum, symnum) => {

            if state.in_const {
                evalerror(
                    "Not a constant expression.",
                    state,
                    node
                )
            }

            // For Name, having a child means having an owner.
            if node.children.len() > 0 {
                // Run parent through the loop for lookup.
                let owner = eval(&node.children[0], state);
                return get_field(owner, s, state, node);
            }

            // Don't access stack or this if we are lazy evaling a topvar.
            if state.eval_var == "" {
                if state.stack.has(s) {
                    return state.stack.get(s).clone();
                }
                else if state.objsys.has_this() {
                    return get_field(state.objsys.get_this_object(), s, state, node);
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
                node
            );
        }

        NodeType::Return(_, _) => {
            let retval = eval(&node.children[0], state);
            return Object::Return(Box::new(retval));
        }

        NodeType::CollAccess(_, _) => {

            let owner = eval(&node.children[0], state);

            let index_obj = eval(&node.children[1], state);

            let ilist_ref = get_field(owner, "__list", state, &node.children[0]);

            if let Object::Reference(ilist_rk) = ilist_ref {
                let ilist = state.objsys.get_list(&ilist_rk);

                if let Object::Int(n) = index_obj {
                    if n >= 0 {
                        return ilist.get_el(n as usize)
                    }
                    evalerror(format!("Index must be positive: {}", n), state, node)
                }
                evalerror(format!("Illegal index: {}", index_obj), state, node)
            }
            evalerror(format!("Expected reference, got: {}", ilist_ref), state, &node.children[0])
        }

        NodeType::MethodCall(name, owner, _filename, _, _) => {

            if state.in_const {
                evalerror(
                    "Not a constant expression.",
                    state,
                    node
                )
            }

            let reference: Object = eval(owner, state);

            if let Object::Reference(refid) = reference {
                let instance = state.objsys.get_instance(&refid);
                let c = state.objsys.get_class(&instance.classname);
                let meth_obj = c.get_method(name, state, node);
                return call_function(MaybeRef::Ref(refid), &meth_obj, &node.children[0], state)
            }
            panic!("Can't access {} of {}", name, owner);
        }

        NodeType::FunCall(s, _, _) => {

            if state.in_const {
                evalerror(
                    "Not a constant expression.",
                    state,
                    node
                )
            }

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
            if state.has_global(s) {

                let funcnode = state.get_global(s);

                return match funcnode.nodetype {
                    NodeType::FunDef(_, _, _, _) => {
                        call_function(
                            MaybeRef::None,
                            &create_function(&funcnode),
                            &node.children[0],
                            state)
                    }
                    NodeType::Constructor(_, _, _, _) => {
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
                // let mut args = argnodes_to_argobjs(
                //     &node.children[0].children,
                //     state);
                // return builtin::call(s, &mut args, state, node);
                return builtin::call(node, s, state);
            }

            evalerror(
                format!("Function not found: {}", s),
                state,
                node
            )
        }

        NodeType::FunDef(s, _, _, _) => {
            let funcobj = create_function(node);
            state.stack.add_new(s, funcobj);
            return Object::Null;
        }

        NodeType::Conditional(_, _) => {

            for condnode in &node.children {

                match condnode.nodetype {
                    NodeType::If(_, _) |
                    NodeType::ElseIf(_, _) => {
                        let boolnode= &condnode.children[0];
                        let cond = eval(&boolnode, state);

                        match cond {
                            Object::Bool(v) => {
                                if v {
                                    let bodynode= &condnode.children[1];
                                    state.stack.push_lex();
                                    let ret = eval(&bodynode, state);
                                    state.stack.pop_lex();
                                    return ret;
                                }
                            }
                            _ => panic!("Expected bool in conditional")
                        }
                    }
                    NodeType::Else(_, _) => {
                        let bodynode= &condnode.children[0];
                        state.stack.push_lex();
                        let ret = eval(&bodynode, state);
                        state.stack.pop_lex();
                        return ret;
                    }
                    _ => panic!("Invalid node in conditional!")
                }
            }
            return Object::Null;
        }

        NodeType::While(_, _) => {

            let boolnode = &node.children[0];
            let block = &node.children[1];
            let mut cond = eval(boolnode, state);

            match &cond {

                Object::Bool(mut v) => {

                    while v {
                        eval(block, state);
                        cond = eval(boolnode, state);

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

        NodeType::DoWhile(_, _) => {

            let block = &node.children[0];
            let boolnode = &node.children[1];

            eval(block, state);

            let mut cond = eval(boolnode, state);

            if let Object::Bool(mut b) = cond {

                while b {
                    eval(block, state);
                    cond = eval(boolnode, state);

                    match &cond {
                        Object::Bool(new_b) => {
                            b = *new_b;
                        }
                        _ => panic!("Conditional no longer bool: {}", cond)
                    }
                }
            }
            else {
                evalerror(
                    format!("Expected bool in conditional. Got: {}", cond),
                    state,
                    &node.children[0]
                )
            }

            return Object::Null;
        }

        NodeType::For(_, _) => {

            if node.children.len() == 3 {
                // Three children mean 'for x in name' loop.
                // First child is an the variable, second the
                // free variable, and third the body block. 
                let typedvar = &node.children[0];
                let iter_ref = eval(&node.children[1], state);
                let body = &node.children[2];

                let ilist_ref = get_field(iter_ref, "__list", state, node);

                match &ilist_ref {

                    Object::Reference(ilist_rk) => {
                        let ilist = state.objsys.get_list(&ilist_rk);

                        // Put var on new, inner, stack frame
                        let mut cloned = Vec::new();
                        for obj in &ilist.els {
                            cloned.push(obj.clone());
                        }

                        state.stack.push_lex();

                        for c in cloned {
                            match &typedvar.nodetype {
                                NodeType::TypedVar(_, name, _, _) => {
                                    state.stack.add_new(name, c);
                                    eval(body, state);
                                }
                                _ => {
                                    panic!("For loop expecped typed var. Got: {}", &typedvar);
                                }
                            }
                        }
                        state.stack.pop_lex();
                    }
                    x => panic!("Expected reference, got: {}", x)
                }
                return Object::Null;
            }
            else if node.children.len() == 4 {
                // Four children means loop of the form
                // for (var i=0, i<10, i++).
                let assign = &node.children[0];
                let condexpr = &node.children[1];
                let mutexpr = &node.children[2];
                let body = &node.children[3];

                eval(assign, state);

                loop {
                    let condobj = eval(condexpr, state);

                    match condobj {
                        Object::Bool(b) => {

                            if !b {
                                break;
                            }
                            eval(body, state);
                            eval(mutexpr, state);
                        }
                        _ => evalerror(
                            "Expected boolean expression.",
                            state,
                            &node.children[1]
                        )
                    }
                }
            }
            return Object::Null;
        }

        NodeType::Block(_, _) => {

            for c in &node.children {

                if Instant::now() - state.last_gc > GC_TIME {
                    let gc_start = state.start_time.elapsed();
                    state.stack.garbagecollect(&mut state.objsys, &state.constructing, &state.globals);
                    let gc_end = state.start_time.elapsed();
                    state.last_gc = Instant::now();
                    println!("Garbage collected in {}μs", (gc_end - gc_start).as_micros());
                }

                let retval = eval(c, state);

                match &retval {
                    Object::Return(_) => {
                        return retval;
                    }
                    _ => {}
                }
            }
            return Object::Null;
        }

        NodeType::List(_, _) => {

            let class = state.objsys.get_class("List");
            let mut inst = class.instantiate();
            
            let mut vals: Vec<Object> = Vec::new();
            for c in &node.children {
                let v = eval(c, state);
                vals.push(v);
            }

            let mut ilist = InternalList::new();
            ilist.set_elements(vals);

            inst.set_field(String::from("__list"), Object::Reference(ilist.id.clone()));
            state.objsys.register_list(ilist);

            let instref = state.objsys.register_instance(*inst);
            return instref;
        }

        NodeType::This(_, _) => {

            if state.objsys.has_this() {
                return Object::Reference(state.objsys.get_this());
            }
            evalerror(
                "Not found in context: 'this'.",
                state,
                node
            )
        }

        NodeType::Null(_, _) => {
            return Object::Null;
        }

        _ => panic!("Unknown node type: {}", t)
    }
}
