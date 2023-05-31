use std::fmt;
use token::Token;
use node::{NodeType, Node};
use utils::{dprint, darterror};


impl Node {

    pub fn new(nodetype: NodeType) -> Node {
        Node {
            nodetype,
            children: Vec::new(),
        }
    }


    pub fn print(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {

        match self.children.len() {

            0 => {
                writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype)
            },

            1 => {
                let res = writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype);
                if let Err(e) = self.children[0].print(f,depth + 1) {
                    println!("Error when printing node children: {}", e.to_string())
                }
                return res
            },

            2 => {
                let res = writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype);
                if let Err(e) = self.children[0].print(f,depth + 1) {
                    println!("Error when printing node children: {}", e.to_string())
                }
                if let Err(e) = self.children[1].print(f,depth + 1) {
                    println!("Error when printing node children: {}", e.to_string())
                }
                return res;
            }

            x => {
                let res = writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype);
                for i in 0 .. x {
                    if let Err(e) = self.children[i].print(f, depth + 1) {
                        println!("Error when printing node children: {}", e.to_string());
                    }
                }
                if let Err(e) = write!(f, "") {
                    println!("Error when printing node: {}", e.to_string())
                }
                return res;
            }
        }
    }
}


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print(f, 0)
    }
}


pub fn parse(tokens: &Vec<Token>) -> Result<Node, String> {
    dprint(" ");
    dprint("PARSE");
    dprint(" ");

    let mut root = Node::new(NodeType::Module);
    let directive_node = directives(tokens, 0);
    let mut i = directive_node.1;
    root.children.push(directive_node.0);


    while i < tokens.len() - 1 {
        let (funnode, new_pos) = fundef(tokens, i);
        root.children.push(funnode);

        dprint(format!("Parse: read len: {}", new_pos));
        i = new_pos;
    }

    if i < tokens.len() - 1 {
        return Err(format!("Expected end of input, found {} at {}", tokens[i], i))
    }
    else if i > tokens.len() - 1 {
        return Err(format!("Index returned beyond end of token array. Index: {}, len: {}", i, tokens.len()))
    }

    dprint(format!("Parse: finished at index: {}", i));
    Ok(root)
}


fn directives(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;
    let mut directives_node = Node::new(NodeType::Directives);

    while i < tokens.len() {

        match &tokens[i] {
            Token::Import => {

                let mut node = Node::new(NodeType::Import);

                i += 1;
                if let Token::Str(s) = &tokens[i] {
                    node.children.push(Node::new(NodeType::Str(s.clone())));

                    i += 1;
                    if let Token::EndSt = tokens[i] {
                        i += 1;

                        directives_node.children.push(node);
                    }
                    else {
                        panic!("Error: Expected ';' after import.")
                    }
                }
                else {
                    panic!("Error: Expected string after 'import'.")
                }
            }
            _  => break
        }
    }

    (directives_node, i)
}


fn fundef(tokens: &Vec<Token>, pos: usize) -> (Node, usize)  {

    let mut i: usize = pos;
    let t: &Token = tokens.get(i).unwrap();
    i += 1;

    match t {
        Token::Name(s) => {
            dprint(format!("fundef found NAME {}", s));

            let t2: &Token = tokens.get(i).unwrap();
            i += 1;

            match t2 {

                Token::Name(fname) => {
                    let mut node = Node::new(NodeType::FunDef(fname.to_string()));
                    dprint("Calling paramlist from fundef");
                    let (params, new_pos) = paramlist(tokens, i);
                    i = new_pos;
                    node.children.push(params);

                    let t3: &Token = tokens.get(i).unwrap();
                    i += 1;

                    match t3 {
                        Token::Block1 => {
                            // Could increment i here. But is it better for block parse to
                            // just expect starting at '{'?
                            let (body, new_pos) = block(tokens, i);
                            node.children.push(body);
                            i = new_pos;
                            dprint(format!("Parse: fundef parsed to {}", new_pos));
                            return (node, i)
                        }

                        _ => {
                            panic!("Expected {{. Got: {}", t3)
                        }
                    }
                }

                _ => {
                    panic!("Expected function name.")
                }
            }
        }

        Token::Class => {
            let (cnode, new_pos) = class(tokens, i);
            dprint(format!("parsed class to pos {}", new_pos));
            return (cnode, new_pos);
        }

        Token::Import => {
            // As Dart.
            panic!("Directives must appear before any declarations.");
        }

        x => {
            panic!("Expected top level declaration. Got {}", x);
        }
    }
}


fn class(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;

    match &tokens[i] {
        Token::Name(classname) => {

            let mut classnode = Node::new(NodeType::Class(classname.clone()));
            i += 1;

            if let Token::Block1 = tokens[i] {
                i += 1;

                let (members, new_pos) = readmembers(classname.clone(), tokens, i);
                classnode.children = members;
                i = new_pos;
                // }

                if let Token::Block2 = tokens[i] {
                    return (classnode, i + 1)
                }
                panic!("Expected '}' to end class.")
            }
            else {
                panic!("Error: Expected '{' after class name")
            }

        }
        x => {
            panic!("Error: Expected class name. Got: {}", x)
        }
    }
}


fn readmembers(classname: String, tokens: &Vec<Token>, pos: usize) -> (Vec<Node>, usize) {
    // Expecting member declaration - field or method.

    let mut i = pos;
    let mut members : Vec<Node> = Vec::new();

    while i < tokens.len() {

        match &tokens[i] {

            Token::Name(mtype) => {

                if *mtype == classname {
                    // Constructor
                    dprint("Found constructor");

                    i += 1;

                    let mut constructor_node = Node::new(NodeType::Constructor(classname.clone()));
                    let (params, new_pos) = paramlist(tokens, i);
                    let (body, new_pos)  = block(tokens, new_pos + 1);
                    i = new_pos;

                    constructor_node.children.push(params);
                    constructor_node.children.push(body);


                    members.push(constructor_node);
                    continue;
                }

                i += 1;


                match &tokens[i] {
                    Token::Name(fieldname) => {
                        i += 1;

                        match &tokens[i] {
                            Token::Paren1 => {
                                // Method
                                dprint("Found method");

                                let mut method_node = Node::new(NodeType::FunDef(fieldname.clone()));
                                let (param_node, new_pos) = paramlist(tokens, i);
                                i = new_pos;

                                if let Token::Block1 = tokens[i] {
                                    i += 1;
                                    let (body, new_pos) = block(tokens, i);
                                    i = new_pos;

                                    method_node.children.push(param_node);
                                    method_node.children.push(body);

                                    members.push(method_node);
                                    // return (method_node, i)
                                }
                                else {
                                    panic!("Expected opening brace in method declaration: '{'")
                                }
                            }

                            Token::EndSt => {
                                // Uninitialized field declare
                                dprint("Found uninitialized field");

                                let fieldnode = Node::new(NodeType::TypedVar(mtype.clone(), fieldname.clone()));
                                members.push(fieldnode);
                                i += 1;
                            }

                            Token::Assign => {
                                // Initialized field declare
                                dprint("Found initialized field");


                                i += 1;

                                let (val, new_pos) = expression(tokens, i);
                                i = new_pos;

                                if let Token::EndSt = tokens[i] {
                                    dprint("Got endst after field init");
                                    i += 1;
                                }
                                else {
                                    panic!("Expected ';' after field initialization.")
                                }

                                let mut eqnode = Node::new(NodeType::Assign);
                                let fieldnode = Node::new(NodeType::TypedVar(mtype.clone(), fieldname.clone()));

                                eqnode.children.push(fieldnode);
                                eqnode.children.push(val);
                                members.push(eqnode);
                            }

                            Token::Block2 => {
                                break;
                            }

                            x => panic!("Unexpected token when parsing class member: '{}'", x)
                        }
                    }

                    Token::Block2 => {
                        break;
                    }

                    x => panic!("Expected class member declaration. Got: '{}'", x)
                }
            }

            Token::Block2 => {
                break;
            }

            x => panic!("Unexpected first token when parsing class member: {}", x)
        }
    }

    (members, i)
}


fn paramlist(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {
    dprint(format!("Paramlist on {}", tokens[pos]));

    let mut i = pos;

    if let Token::Paren1 = &tokens[i] {

        let mut node = Node::new(NodeType::ParamList);
        let mut expect_comma = false;
        i += 1;

        while i < tokens.len() {

            match &tokens[i] {

                Token::Paren2 => {
                    return (node, i + 1);
                }

                Token::Comma => {
                    if !expect_comma {
                        panic!("Error: Unexpected separator in parameter list: ','.");
                    }
                    i += 1;
                    expect_comma = false;
                }

                Token::Name(s) => {
                    let paramnode= Node::new(NodeType::Name(s.to_string()));
                    node.children.push(paramnode);
                    expect_comma = true;
                    i += 1;
                }

                _ => {
                    panic!("Unexpected token when reading parameters: {}", &tokens[i])
                }
            }
        }
    }
    else {
        darterror("A function declaration needs an explicit list of parameters.")
    }
    panic!("Error when reading param list.")
}


fn arglist(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;

    if let Token::Paren1 = &tokens[i] {

        let mut node = Node::new(NodeType::ArgList);
        let mut expect_comma = false;
        i += 1;

        while i < tokens.len() {

            match &tokens[i] {

                Token::Paren2 => {
                    return (node, i + 1);
                }

                Token::Comma => {
                    if !expect_comma {
                        panic!("Error: Unexpected separator in arg list: ','.");
                    }
                    i += 1;
                    expect_comma = false;
                }

                _ => {
                    if expect_comma {
                        panic!("Error: Expected separator in arg list.");
                    }
                    let (arg, new_pos) = expression(tokens, i);
                    node.children.push(arg);
                    i = new_pos;
                    expect_comma = true;
                }
            }
        }
    }
    else {
        panic!("Error: Expected start of arglist: '('. Found: {}", &tokens[i])
    }
    panic!("Error when reading arg list.")
}


/// Parse a series of statements.
///
/// Expects first token after block started by {.
/// Consumes the end-block token }.
fn block(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut node = Node::new(NodeType::Block);
    let mut i = pos;

    while i < tokens.len() {
        dprint(format!("Parse: block loop at: {}, token: {}", i, &tokens[i]));

        if tokens[i] == Token::Block2 {
            dprint(String::from("Parse: token is end-of-block, breaking."));
            i += 1;
            break;
        }
        if tokens[i] == Token::End {
            dprint(String::from("Parse: token is end, breaking."));
            break;
        }

        let (snode, new_pos) = statement(tokens, i);
        node.children.push(snode);

        i = new_pos;

        match &tokens[i] {

            Token::Block2 => {
                // i += 1;
                continue;
            }
            Token::EndSt => {
                // ENDST should be consumed by statement?
                i += 1;
                continue;
            }
            _ => continue
        }
    }

    return (node, i)
}


fn statement(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    dprint(format!("Parse: statement: {}", &tokens[pos]));

    let mut i = pos;

    match &tokens[i] {

        Token::Name(s) => {

            i = i + 1;
            let t2 = &tokens[i];

            match t2 {

                Token::Name(name) => {
                    // Two names in a row indicate a typed variable or function definition.
                    i += 1;
                    let t3 = &tokens[i];
                    i += 1;


                    let typed_var = Node::new(NodeType::TypedVar(s.to_string(), name.to_string()));

                    match t3 {
                        Token::Assign => {
                            let mut ass_node = Node::new(NodeType::Assign);
                            ass_node.children.push(typed_var);
                            let (right_node, i) = expression(tokens, i);
                            ass_node.children.push(right_node);
                            dprint(format!("Parse: returning statement at token {}", i));
                            return (ass_node, i)
                        }

                        _ => panic!("Unexpected token in statement. Expected: =. Got: {}", t3)
                    }
                }

                Token::Assign => {
                    i += 1;
                    let mut ass_node = Node::new(NodeType::Assign);

                    let var = Node::new(NodeType::Name(s.to_string()));
                    let (right_node, i) = expression(tokens, i);

                    ass_node.children.push(var);
                    ass_node.children.push(right_node);

                    return (ass_node, i)
                }

                Token::Paren1 => {
                    // Function call.
                    // These are also handled in term. Maybe we can just pass this along?
                    let (args_node, new_pos) = arglist(tokens, i);
                    i = new_pos;
                    let mut funcall_node = Node::new(NodeType::FunCall(s.to_string()));
                    funcall_node.nodetype = NodeType::FunCall(s.to_string());
                    funcall_node.children.push(args_node);
                    return (funcall_node, i)
                }

                Token::Access => {

                    i += 1;
                    let t3 = &tokens[i];

                    match t3 {

                        Token::Name(acc_name) => {


                            i += 1;
                            let t4 = &tokens[i];

                            return match t4 {
                                Token::Paren1 => {

                                    // method call
                                    let (args, new_pos) = arglist(tokens, i);
                                    i = new_pos;
                                    let mut methcall_node = Node::new(NodeType::MethodCall(s.to_string(), acc_name.to_string()));
                                    methcall_node.children.push(args);

                                    if tokens[i] != Token::EndSt {
                                        panic!("Unexpected token at pos {}: {}", i, tokens[i]);
                                    }
                                    i += 1;

                                    (methcall_node, i)
                                }

                                _ => {
                                    let mut acc_node = Node::new(NodeType::Access);
                                    let obj_node = Node::new(NodeType::Name(s.to_string()));
                                    let member_node = Node::new(NodeType::Name(acc_name.to_string()));
                                    acc_node.children.push(obj_node);
                                    acc_node.children.push(member_node);

                                    (acc_node, i)
                                }
                            }
                        }

                        _ => {
                            panic!("Unexpected token following '.': {}", t3)
                        }
                    }
                }
                _ => return expression(tokens, pos)
            }
        }

        Token::If => {
            dprint("Parse: if");

            let mut condnode = Node::new(NodeType::Conditional);

            let (condpart, next_pos) = conditional(tokens, i);
            condnode.children.push(condpart);
            i = next_pos;


            loop {

                let next_token = &tokens[i];

                match next_token {

                    Token::Else => {
                        dprint("Parse: if-else");

                        let (lastcond, last_pos) = conditional(tokens, i);
                        condnode.children.push(lastcond);
                        i = last_pos;
                    }

                    _ => {
                        break;
                    }
                }
            }
            return (condnode, i)
        }

        Token::Return => {
            let (val, new_pos) = expression(tokens, i + 1);
            let mut ret = Node::new(NodeType::Return);
            ret.children.push(val);
            return (ret, new_pos);
        }

        _ => {
            return expression(tokens, pos);
        }
    }
}


fn conditional(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;

    match &tokens[i] {

        Token::If => {

            i += 1;

            match tokens[i] {
                Token::Paren1 => {
                    i += 1;
                    let (boolnode, new_pos) = expression(tokens, i);

                    match tokens[new_pos] {
                        Token::Paren2 => {
                            i = new_pos + 1;

                            match tokens[i] {
                                Token::Block1 => {
                                    i += 1;
                                    let (bodynode, new_pos) = block(tokens, i);

                                    i = new_pos;

                                    let mut ifnode = Node::new(NodeType::If);
                                    ifnode.children.push(boolnode);
                                    ifnode.children.push(bodynode);
                                    return (ifnode, i)
                                }
                                _ => panic!("Expected body of conditional")
                            }
                        }
                        _ => panic!("Expected closing paren after conditional expression")
                    }
                }
                _ => panic!("Unexpected token after 'if'")
            }
        }
        Token::Else => {

            i += 1;

            match &tokens[i] {

                Token::If => {

                    i += 1;

                    match tokens[i] {
                        Token::Paren1 => {
                            i += 1;
                            let (boolnode, new_pos) = expression(tokens, i);

                            match tokens[new_pos] {
                                Token::Paren2 => {
                                    i = new_pos + 1;

                                    match tokens[i] {
                                        Token::Block1 => {
                                            i += 1;
                                            let (bodynode, new_pos) = block(tokens, i);
                                            i = new_pos;

                                            let mut elseifnode = Node::new(NodeType::ElseIf);
                                            elseifnode.children.push(boolnode);
                                            elseifnode.children.push(bodynode);

                                            return (elseifnode, i)
                                        }
                                        _ => panic!("Expected body of conditional")
                                    }
                                }
                                _ => panic!("Expected closing paren after conditional expression")
                            }
                        }
                        _ => panic!("Unexpected token after 'if'")
                    }
                }

                Token::Block1 => {

                    i += 1;

                    let (bodynode, new_pos) = block(tokens, i);
                    i = new_pos;

                    let mut elsenode = Node::new(NodeType::Else);
                    elsenode.children.push(bodynode);

                    return (elsenode, i)
                }

                x => {
                    panic!("Unexpeced token afer 'else': {}", x)
                }
            }
        }

        _ => {
            panic!("Expected conditional")
        }
    }
}



