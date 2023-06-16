use context::Ctx;
use reader::Reader;
use token::Token;
use node::{NodeType, Node};
use expression::expression;
use utils::{dprint, dart_parseerror};


pub fn parse(reader: &mut Reader, ctx: &Ctx) -> Result<Node, String> {
    dprint(" ");
    dprint("PARSE");
    dprint(" ");

    let mut root = Node::new(NodeType::Module);
    let directive_node = directives(reader);
    root.children.push(directive_node);

    while reader.position() < reader.len() - 1 {
        let funnode= fundef(reader, ctx);
        root.children.push(funnode);
        dprint(format!("Parse: read len: {}", reader.position()));
    }

    if reader.position() < reader.len() - 1 {
        return Err(format!("Expected end of input, found {} at {}", reader.sym(), reader.position()))
    }
    else if reader.position() > reader.len() - 1 {
        return Err(format!("Index returned beyond end of token array. Index: {}, len: {}", reader.position(), reader.len()))
    }

    dprint(format!("Parse: finished at index: {}", reader.position()));
    Ok(root)
}


fn directives(reader: &mut Reader) -> Node {

    let mut directives_node = Node::new(NodeType::Directives);

    while reader.position() < reader.len() {

        match reader.sym() {
            Token::Import(_, _) => {

                let mut node = Node::new(NodeType::Import);

                reader.next();
                if let Token::Str(s, _, _, _) = reader.sym() {
                    node.children.push(Node::new(NodeType::Str(s.clone())));

                    reader.next();
                    if let Token::EndSt(_, _) = reader.sym() {
                        reader.next();
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
    directives_node
}


fn fundef(reader: &mut Reader, ctx: &Ctx) -> Node  {

    let t = reader.sym();
    reader.next();

    match t {
        Token::Name(s, _, _) => {
            dprint(format!("fundef found NAME {}", s));

            let t2 = reader.sym();
            reader.next();

            match t2 {

                Token::Name(fname, _, _) => {
                    let mut node = Node::new(NodeType::FunDef(fname.to_string()));
                    dprint("Calling paramlist from fundef");
                    let params = paramlist(reader, ctx);
                    node.children.push(params);

                    let t3 = reader.sym();
                    reader.next();

                    match t3 {
                        Token::Block1(_, _) => {
                            // Could increment i here. But is it better for block parse to
                            // just expect starting at '{'?
                            let body = block(reader, ctx);
                            node.children.push(body);
                            dprint(format!("Parse: fundef parsed to {}", reader.position()));
                            return node;
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

        Token::Class(_, _) => {
            let cnode = class(reader, ctx);
            dprint(format!("parsed class to pos {}", reader.position()));
            return cnode;
        }

        Token::Import(_, _) => {
            // As Dart.
            panic!("Directives must appear before any declarations.");
        }

        x => {
            panic!("Expected top level declaration. Got {}", x);
        }
    }
}


fn class(reader: &mut Reader, ctx: &Ctx) -> Node {

    match reader.sym() {
        Token::Name(classname, _, _) => {

            reader.next();

            let mut classnode = Node::new(NodeType::Class(classname.clone()));

            if let Token::Block1(_, _) = reader.sym() {
                reader.next();

                let members = readmembers(classname.clone(), reader, ctx);
                classnode.children = members;

                if let Token::Block2(_, _) = reader.sym() {
                    reader.next();
                    return classnode;
                }
                panic!("{}", "Expected '}' to end class.")
            }
            else {
                panic!("{}", "Error: Expected '{' after class name")
            }

        }
        x => {
            panic!("Error: Expected class name. Got: {}", x)
        }
    }
}


fn readmembers(classname: String, reader: &mut Reader, ctx: &Ctx) -> Vec<Node> {
    // Expecting member declaration - field or method.

    let mut members : Vec<Node> = Vec::new();

    while reader.position() < reader.len() {

        match reader.sym() {

            Token::Name(mtype, _, _) => {

                if *mtype == classname {
                    // Constructor
                    dprint("Found constructor");

                    reader.next();

                    let mut constructor_node = Node::new(NodeType::Constructor(classname.clone()));
                    let params = paramlist(reader, ctx);
                    reader.next();
                    let body  = block(reader, ctx);

                    constructor_node.children.push(params);
                    constructor_node.children.push(body);

                    members.push(constructor_node);
                    continue;
                }

                reader.next();

                match reader.sym() {
                    Token::Name(fieldname, _, _) => {
                        reader.next();

                        match reader.sym() {
                            Token::Paren1(_, _) => {
                                // Method
                                dprint("Found method");

                                let mut method_node = Node::new(NodeType::FunDef(fieldname.clone()));
                                let param_node = paramlist(reader, ctx);

                                if let Token::Block1(_, _) = reader.sym() {
                                    reader.next();
                                    let body = block(reader, ctx);

                                    method_node.children.push(param_node);
                                    method_node.children.push(body);

                                    members.push(method_node);
                                }
                                else {
                                    panic!("{}", "Expected opening brace in method declaration: '{'")
                                }
                            }

                            Token::EndSt(_, _) => {
                                // Uninitialized field declare
                                dprint("Found uninitialized field");

                                let fieldnode = Node::new(NodeType::TypedVar(mtype.clone(), fieldname.clone()));
                                members.push(fieldnode);
                                reader.next();
                            }

                            Token::Assign(_, _) => {
                                // Initialized field declare
                                dprint("Found initialized field");

                                reader.next();

                                let val = expression(reader, ctx);

                                if let Token::EndSt(_, _) = reader.sym() {
                                    dprint("Got endst after field init");
                                    reader.next();
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

                            Token::Block2(_, _) => {
                                break;
                            }

                            x => panic!("Unexpected token when parsing class member: '{}'", x)
                        }
                    }

                    Token::Block2(_, _) => {
                        break;
                    }

                    x => panic!("Expected class member declaration. Got: '{}'", x)
                }
            }

            Token::Block2(_, _) => {
                break;
            }

            x => panic!("Unexpected first token when parsing class member: {}", x)
        }
    }

    members
}


fn paramlist(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Paramlist on {}", reader.sym()));

    if let Token::Paren1(_, _) = reader.sym() {

        let mut node = Node::new(NodeType::ParamList);
        let mut expect_comma = false;
        reader.next();

        while reader.position() < reader.len() {

            match reader.sym() {

                Token::Paren2(_, _) => {
                    reader.next();
                    return node;
                }

                Token::Comma(_, _) => {
                    if !expect_comma {
                        panic!("Error: Unexpected separator in parameter list: ','.");
                    }
                    reader.next();
                    expect_comma = false;
                }

                Token::Name(s, _, _) => {
                    let paramnode= Node::new(NodeType::Name(s.to_string()));
                    node.children.push(paramnode);
                    expect_comma = true;
                    reader.next();
                }

                _ => {
                    panic!("Unexpected token when reading parameters: {}", reader.sym())
                }
            }
        }
    }
    else {
        dart_parseerror(
            "A function declaration needs an explicit list of parameters.",
            &ctx.filepath,
            reader.tokens(),
            reader.position() - 1
        )
    }
    panic!("Error when reading param list.")
}


pub fn arglist(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("arglist on {}", reader.sym()));

    if let Token::Paren1(_, _) = reader.sym() {

        let mut node = Node::new(NodeType::ArgList);
        let mut expect_comma = false;
        reader.next();

        while reader.position() < reader.len() {

            match reader.sym() {

                Token::Paren2(_, _) => {
                    reader.next();
                    return node;
                }

                Token::Comma(_, _) => {
                    if !expect_comma {
                        panic!("Error: Unexpected separator in arg list: ','.");
                    }
                    reader.next();
                    expect_comma = false;
                }

                x => {
                    if expect_comma {
                        panic!("Error: Expected separator in arg list. Got: {}", x);
                    }
                    let arg = expression(reader, ctx);
                    node.children.push(arg);
                    expect_comma = true;
                }
            }
        }
    }
    else {
        panic!("Error: Expected start of arglist: '('. Found: {}", reader.sym())
    }
    panic!("Error when reading arg list.")
}


/// Parse a series of statements.
///
/// Expects first token after block started by {.
/// Consumes the end-block token }.
fn block(reader: &mut Reader, ctx: &Ctx) -> Node {

    let mut node = Node::new(NodeType::Block);

    while reader.position() < reader.len() {
        dprint(format!("Parse: block loop at: {}, token: {}", reader.position(), reader.sym()));

        match reader.sym() {

            Token::Block2(_, _) => {
                dprint(String::from("Parse: token is end-of-block, breaking."));
                reader.next();
                break;
            }

            Token::End => {
                dprint(String::from("Parse: token is end, breaking."));
                break;
            }

            Token::EndSt(_, _) => {
                // Dart allows redundant semicolons. Analyzer complains.
                // Warranted semicolons are handled below, when statement returns.
                //
                // Analyser("info • bin/redarter.dart:5:1 • Unnecessary empty statement. Try removing the empty statement or restructuring the code. • empty_statements");
                reader.next();
                continue;
            }

            _ => {
                let snode = statement(reader, ctx);
                node.children.push(snode);

                match reader.sym() {

                    Token::Block2(_, _) => {
                        // i += 1;
                        reader.next();
                        continue;
                    }
                    Token::EndSt(_, _) => {
                        // ENDST should be consumed by statement?
                        reader.next();
                        continue;
                    }
                    _ => continue
                }
            }
        }
    }

    return node
}


fn statement(reader: &mut Reader, ctx: &Ctx) -> Node {

    dprint(format!("Parse: statement: {}", reader.sym()));

    match reader.sym() {

        Token::Name(s, _, _) => {

            reader.next();
            let t2 = reader.sym();

            match t2 {

                Token::Name(name, _, _) => {
                    // Two names in a row indicate a typed variable or function definition.
                    reader.next();
                    let t3 = reader.sym();
                    reader.next();


                    let typed_var = Node::new(NodeType::TypedVar(s.to_string(), name.to_string()));

                    match t3 {
                        Token::Assign(_, _) => {
                            let mut ass_node = Node::new(NodeType::Assign);
                            ass_node.children.push(typed_var);
                            let right_node = expression(reader, ctx);
                            ass_node.children.push(right_node);
                            dprint(format!("Parse: returning statement at token {}", reader.position()));
                            return ass_node;
                        }

                        _ => panic!("Unexpected token in statement. Expected: =. Got: {}", t3)
                    }
                }

                Token::Assign(_, _) => {
                    reader.next();
                    let mut ass_node = Node::new(NodeType::Assign);

                    let var = Node::new(NodeType::Name(s.to_string()));
                    let right_node = expression(reader, ctx);

                    ass_node.children.push(var);
                    ass_node.children.push(right_node);

                    return ass_node;
                }

                Token::Paren1(_, _) => {
                    // Function call.
                    // These are also handled in term. Maybe we can just pass this along?
                    let args_node = arglist(reader, ctx);
                    let mut funcall_node = Node::new(NodeType::FunCall(s.to_string()));
                    funcall_node.nodetype = NodeType::FunCall(s.to_string());
                    funcall_node.children.push(args_node);
                    return funcall_node;
                }

                Token::Access(_, _) => {

                    reader.next();
                    let t3 = reader.sym();

                    match t3 {

                        Token::Name(acc_name, _, _) => {

                            reader.next();
                            let t4 = reader.sym();

                            return match t4 {
                                Token::Paren1(_, _) => {

                                    // Method call
                                    let args = arglist(reader, ctx);
                                    let mut methcall_node = Node::new(NodeType::MethodCall(s.to_string(), acc_name.to_string()));
                                    methcall_node.children.push(args);

                                    match reader.sym() {
                                        Token::EndSt(_, _) => {
                                            reader.next();
                                            methcall_node
                                        }

                                        x => {
                                            panic!("Unexpected token at pos {}: {}", reader.position(), x);
                                        }
                                    }
                                }

                                _ => {
                                    let mut acc_node = Node::new(NodeType::Access);
                                    let obj_node = Node::new(NodeType::Name(s.to_string()));
                                    let member_node = Node::new(NodeType::Name(acc_name.to_string()));
                                    acc_node.children.push(obj_node);
                                    acc_node.children.push(member_node);
                                    return acc_node;
                                }
                            }
                        }

                        _ => {
                            panic!("Unexpected token following '.': {}", t3)
                        }
                    }
                }

                _ => {
                    reader.back();
                    return expression(reader, ctx)
                }
            }
        }

        Token::If(_, _) => {
            dprint("Parse: if");

            let mut condnode = Node::new(NodeType::Conditional);

            let condpart = conditional(reader, ctx);
            condnode.children.push(condpart);


            loop {

                let next_token = reader.sym();

                match next_token {

                    Token::Else(_, _) => {
                        dprint("Parse: if-else");

                        let lastcond = conditional(reader, ctx);
                        condnode.children.push(lastcond);
                    }

                    _ => {
                        break;
                    }
                }
            }
            return condnode;
        }

        Token::While(_, _) => {
            reader.next();
            if let Token::Paren1(_, _) = reader.sym() {

                reader.next();
                let boolexpr = expression(reader, ctx);

                if let Token::Paren2(_, _) = reader.sym() {

                    reader.next();

                    if let Token::Block1(_, _) = reader.sym() {
                        reader.next();
                        let blocknode = block(reader, ctx);

                        let mut node = Node::new(NodeType::While);
                        node.children.push(boolexpr);
                        node.children.push(blocknode);

                        return node;
                    }
                    dart_parseerror(format!("Unexpected token1: {}", reader.sym()), String::from(&ctx.filepath), reader.tokens(), reader.position());
                }
                dart_parseerror(format!("Unexpected token: {}", reader.sym()), String::from(&ctx.filepath), reader.tokens(), reader.position());
            }
            // As dart.
            dart_parseerror("Expected to find '('", &ctx.filepath, reader.tokens(), reader.position());
        }

        Token::Return(_, _) => {
            reader.next();
            let val = expression(reader, ctx);
            let mut ret = Node::new(NodeType::Return);
            ret.children.push(val);
            return ret;
        }

        _ => {
            return expression(reader, ctx);
        }
    }
}


fn conditional(reader: &mut Reader, ctx: &Ctx) -> Node {

    match reader.sym() {

        Token::If(_, _) => {

            reader.next();

            match reader.sym() {
                Token::Paren1(_, _) => {
                    reader.next();
                    let boolnode = expression(reader, ctx);

                    match reader.sym() {
                        Token::Paren2(_, _) => {
                            reader.next();

                            match reader.sym() {
                                Token::Block1(_, _) => {
                                    reader.next();
                                    let bodynode = block(reader, ctx);

                                    let mut ifnode = Node::new(NodeType::If);
                                    ifnode.children.push(boolnode);
                                    ifnode.children.push(bodynode);
                                    return ifnode;
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
        Token::Else(_, _) => {

            reader.next();

            match reader.sym() {

                Token::If(_, _) => {

                    reader.next();

                    match reader.sym() {
                        Token::Paren1(_, _) => {
                            reader.next();
                            let boolnode = expression(reader, ctx);

                            match reader.sym() {
                                Token::Paren2(_, _) => {
                                    reader.next();

                                    match reader.sym() {
                                        Token::Block1(_, _) => {
                                            reader.next();
                                            let bodynode = block(reader, ctx);

                                            let mut elseifnode = Node::new(NodeType::ElseIf);
                                            elseifnode.children.push(boolnode);
                                            elseifnode.children.push(bodynode);

                                            return elseifnode;
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

                Token::Block1(_, _) => {

                    reader.next();

                    let bodynode = block(reader, ctx);

                    let mut elsenode = Node::new(NodeType::Else);
                    elsenode.children.push(bodynode);

                    return elsenode;
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



