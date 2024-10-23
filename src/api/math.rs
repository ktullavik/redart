use rand::Rng;
use crate::{node::Node, object::Object, state::State};
use crate::error::{check_argc, err_arg_type};


// pub fn err_arg_count(
//     fname: &str,
//     expected: usize,
//     got: usize,
//     fnode: &Node,
//     state: &State) -> ! {

//     evalerror(
//         format!("Expected {} argument(s) for {}(). Got: {}", got, fname, expected),
//         state,
//         fnode)
// }


// pub fn err_arg_type(
//     fname: &str,
//     expected: &str,
//     got: &Object,
//     argnode: &Node,
//     state: &State) -> ! {

//     evalerror(
//         format!("Illegal argument '{}' for {}(). Expected type: {}", got, fname, expected),
//         state,
//         argnode);
// }


// pub fn check_argc(
//     fname: &str,
//     expected: usize,
//     got: usize,
//     fnode: &Node,
//     state: &State) {

//     if expected != got {
//         err_arg_count(fname, expected, got, fnode, state)
//     }
// }


pub fn acos(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.acos", 1, args.len(), fnode, state);

    if let Object::Double(x) = &args[0] {
        return Object::Double(x.acos());
    }
    if let Object::Int(n) = &args[0] {
        return Object::Double((*n as f64).acos());
    }
    err_arg_type(
        "math.acos", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn asin(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.asin", 1, args.len(), fnode, state);

    if let Object::Double(x) = &args[0] {
        return Object::Double(x.asin());
    }
    if let Object::Int(n) = &args[0] {
        return Object::Double((*n as f64).asin());
    }
    err_arg_type(
        "math.asin", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn atan(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.atan", 1, args.len(), fnode, state);

    if let Object::Double(x) = &args[0] {
        return Object::Double(x.atan());
    }
    if let Object::Int(n) = &args[0] {
        return Object::Double((*n as f64).atan());
    }
    err_arg_type(
        "math.atan", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn atan2(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.atan2", 2, args.len(), fnode, state);

    if let Object::Double(y) = &args[0] {

        if let Object::Double(x) = &args[0] {
            return Object::Double(y.atan2(*x));
        }
        if let Object::Int(x) = &args[0] {
            return Object::Double(y.atan2(*x as f64));
        }
    }
    if let Object::Int(y) = &args[0] {

        if let Object::Double(x) = &args[0] {
            return Object::Double((*y as f64).atan2(*x));
        }
        if let Object::Int(x) = &args[0] {
            return Object::Double((*y as f64).atan2(*x as f64));
        }
    }
    err_arg_type(
        "math.atan2", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn cos(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.cos", 1, args.len(), fnode, state);

    if let Object::Double(x) = &args[0] {
        return Object::Double(x.cos());
    }
    if let Object::Int(x) = &args[0] {
        return Object::Double((*x as f64).cos());
    }
    err_arg_type(
        "math.cos", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn exp(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.exp", 1, args.len(), fnode, state);

    if let Object::Double(x) = &args[0] {
        return Object::Double(x.exp());
    }
    if let Object::Int(x) = &args[0] {
        return Object::Double((*x as f64).exp());
    }
    err_arg_type(
        "math.exp", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn log(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.log", 1, args.len(), fnode, state);

    if let Object::Double(x) = &args[0] {
        return Object::Double(x.ln());
    }
    if let Object::Int(x) = &args[0] {
        return Object::Double((*x as f64).ln());
    }
    err_arg_type(
        "math.log", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn max(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.max", 2, args.len(), fnode, state);
        
    if let Object::Double(x1) = &args[0] {

        if let Object::Double(x2) = &args[1] {
            if *x1 >= *x2 {
                return Object::Double(x1.clone());
            }
            return Object::Double(x2.clone());
        }
        if let Object::Int(n2) = &args[1] {
            if *x1 >= (*n2 as f64) {
                return Object::Double(x1.clone());
            }
            return Object::Int(n2.clone())
        }
        err_arg_type(
            "math.max", 
            "num", 
            &args[1], 
            &argnodes[1], 
            state);
    }
    if let Object::Int(n1) = &args[0] {
        
        if let Object::Double(x2) = &args[1] {
            if (*n1 as f64) > *x2 {
                return Object::Int(n1.clone());
            }
            return Object::Double(x2.clone());
        }
        if let Object::Int(n2) = &args[1] {
            if *n1 >= *n2 {
                return Object::Int(n1.clone());
            }
            return Object::Int(n2.clone())
        }
        err_arg_type(
            "math.max", 
            "num", 
            &args[1], 
            &argnodes[1], 
            state);
    }
    err_arg_type(
        "math.max", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn min(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.min", 2, args.len(), fnode, state);

    if let Object::Double(x1) = &args[0] {

        if let Object::Double(x2) = &args[1] {
            if *x1 <= *x2 {
                return Object::Double(x1.clone());
            }
            return Object::Double(x2.clone());
        }
        if let Object::Int(n2) = &args[1] {
            if *x1 <= (*n2 as f64) {
                return Object::Double(x1.clone());
            }
            return Object::Int(n2.clone())
        }
        err_arg_type(
            "math.min", 
            "num", 
            &args[1], 
            &argnodes[1], 
            state);
    }
    if let Object::Int(n1) = &args[0] {

        if let Object::Double(x2) = &args[1] {
            if (*n1 as f64) < *x2 {
                return Object::Int(n1.clone());
            }
            return Object::Double(x2.clone());
        }
        if let Object::Int(n2) = &args[1] {
            if *n1 <= *n2 {
                return Object::Int(n1.clone());
            }
            return Object::Int(n2.clone())
        }
        err_arg_type(
            "math.min", 
            "num", 
            &args[1], 
            &argnodes[1], 
            state);
    }
    err_arg_type(
        "math.min", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn pow(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.pow", 2, args.len(), fnode, state);
    
    if let Object::Double(x1) = &args[0] {

        if let Object::Double(x2) = &args[1] {
            return Object::Double(x1.powf(*x2));
        }
        if let Object::Int(n2) = &args[1] {
            return Object::Double(x1.powi(*n2 as i32));
        }
        err_arg_type(
            "math.pow", 
            "num", 
            &args[1], 
            &argnodes[1], 
            state);
    }
    if let Object::Int(n1) = &args[0] {

        if let Object::Double(x2) = &args[1] {
            return Object::Double((*n1 as f64).powf(*x2));
        }
        if let Object::Int(n2) = &args[1] {
            return Object::Double((*n1 as f64).powi(*n2 as i32));
        }
        err_arg_type(
            "math.pow", 
            "num", 
            &args[1], 
            &argnodes[1], 
            state);
    }
    err_arg_type(
        "math.pow", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn sin(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.sin", 1, args.len(), fnode, state);
    
    if let Object::Double(x) = &args[0] {
        return Object::Double(x.sin());
    }
    if let Object::Int(n) = &args[0] {
        return Object::Double((*n as f64).sin())
    }
    err_arg_type(
        "math.sin", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn sqrt(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.sqrt", 1, args.len(), fnode, state);

    if let Object::Int(n) = &args[0] {
        return Object::Double((*n as f64).sqrt());
    }
    if let Object::Double(x) = &args[0] {
        return Object::Double(x.sqrt());
    }
    err_arg_type(
        "math.sqrt", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn tan(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.tan", 1, args.len(), fnode, state);
    
    if let Object::Int(n) = &args[0] {
        return Object::Double((*n as f64).tan());
    }
    if let Object::Double(x) = &args[0] {
        return Object::Double(x.tan());
    }
    err_arg_type(
        "math.tan", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}


pub fn next_bool(
    fnode: &Node,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.nextBool", 0, args.len(), fnode, state);
    
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0 .. 2);
    return Object::Bool(r == 1);
}


pub fn next_double(
    fnode: &Node,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.nextDouble", 0, args.len(), fnode, state);
    
    let mut rng = rand::thread_rng();
    let r = rng.gen::<f64>();
    return Object::Double(r);
}


pub fn next_int(
    fnode: &Node,
    argnodes: &Vec<Node>,
    args: Vec<Object>,
    state: &mut State) -> Object {

    check_argc("math.nextInt", 1, args.len(), fnode, state);

    if let Object::Int(n) = &args[0] {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0 .. *n);
        return Object::Int(r);
    }
    err_arg_type(
        "math.nextInt", 
        "num", 
        &args[0], 
        &argnodes[0], 
        state);
}



