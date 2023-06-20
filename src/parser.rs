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
    let directive_node = directives(reader, ctx);
    root.children.push(directive_node);

    while reader.more() {
        let node = decl(reader, ctx);
        root.children.push(node);
    }
    assert_eq!(reader.position(), reader.len() - 1, "Undexpected index at end of parse: {} out of {}", reader.position(), reader.len());

    Ok(root)
}


fn directives(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: directives: {}", reader.sym()));

    let mut directives_node = Node::new(NodeType::Directives);

    while reader.more() {

        match reader.sym() {
            Token::Import(_, _) => {

                let mut node = Node::new(NodeType::Import);

                reader.next();
                if let Token::Str(s, _, _, _) = reader.sym() {
                    reader.next();
                    reader.skip(";", ctx);

                    node.children.push(Node::new(NodeType::Str(s.clone())));
                    directives_node.children.push(node);
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


fn decl(reader: &mut Reader, ctx: &Ctx) -> Node  {
    dprint(format!("Parse: decl: {}", reader.sym()));

    match reader.sym() {

        Token::Name(_, _, _) => {

            let t2 = reader.next();

            match t2 {

                Token::Name(fname, _, _) => {
                    reader.next();
                    let mut node = Node::new(NodeType::FunDef(fname.to_string()));
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
            reader.next();
            return class(reader, ctx);
        }

        Token::Import(_, _) => {
            // As Dart.
            dart_parseerror(
                "Directives must appear before any declarations.",
                ctx,
                &reader.tokens(),
                reader.position()
            );
        }

        x => {
            panic!("Expected top level declaration. Got {}", x);
        }
    }
}


fn class(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: class: {}", reader.sym()));

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


// Expecting member declaration - field or method.
fn readmembers(classname: String, reader: &mut Reader, ctx: &Ctx) -> Vec<Node> {
    dprint(format!("Parse: readmembers: {}", reader.sym()));

    let mut members : Vec<Node> = Vec::new();

    while reader.more() {

        match reader.sym() {

            Token::Name(mtype, _, _) => {

                if *mtype == classname {
                    // Constructor
                    dprint("Found constructor");

                    reader.next();

                    let mut constructor_node = Node::new(NodeType::Constructor(classname.clone()));
                    let params = constructor_paramlist(reader, ctx);
                    reader.next();
                    let body  = block(reader, ctx);

                    constructor_node.children.push(params);
                    constructor_node.children.push(body);

                    members.push(constructor_node);
                    continue;
                }

                match reader.next() {

                    Token::Name(fieldname, _, _) => {

                        match reader.next() {

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
                                reader.next();

                                let fieldnode = Node::new(NodeType::TypedVar(mtype.clone(), fieldname.clone()));
                                members.push(fieldnode);
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


fn constructor_paramlist(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: paramlist: {}", reader.sym()));

    if let Token::Paren1(_, _) = reader.sym() {

        let mut node = Node::new(NodeType::ParamList);
        let mut expect_comma = false;
        reader.next();

        while reader.more() {

            match reader.sym() {

                Token::Paren2(_, _) => {
                    reader.next();
                    return node;
                }

                Token::This(_, _) => {
                    reader.next();
                    reader.skip(".", ctx);

                    match reader.sym() {

                        Token::Name(s, _, _) => {
                            // let fieldname = Node::new(NodeType::Name(s));
                            let this_fieldinit = Node::new(NodeType::ThisFieldInit(s));
                            // this_fieldinit.children.push(fieldname);
                            node.children.push(this_fieldinit);
                            expect_comma = true;
                            reader.next();
                        }

                        x => {
                            dart_parseerror(
                                format!("Expected identifier. Got {}", x),
                                ctx,
                                reader.tokens(),
                                reader.position()
                            );
                        }
                    }
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
            ctx,
            reader.tokens(),
            reader.position() - 1
        )
    }
    panic!("Error when reading param list.")
}


fn paramlist(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: paramlist: {}", reader.sym()));

    if let Token::Paren1(_, _) = reader.sym() {

        let mut node = Node::new(NodeType::ParamList);
        let mut expect_comma = false;
        reader.next();

        while reader.more() {

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
            ctx,
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

        while reader.more() {

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
    dprint(format!("Parse: block: {}", reader.sym()));

    let mut node = Node::new(NodeType::Block);


    while reader.more() {
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

            let t2 = reader.peek();

            match t2 {

                Token::Name(name, _, _) => {
                    // Two names in a row indicate a typed variable or function definition.
                    reader.next();
                    let t3 = reader.next();
                    reader.next();

                    let typed_var = Node::new(NodeType::TypedVar(s.to_string(), name.to_string()));

                    match t3 {
                        Token::Assign(_, _) => {

                            let right_node = expression(reader, ctx);

                            let mut ass_node = Node::new(NodeType::Assign);
                            ass_node.children.push(typed_var);
                            ass_node.children.push(right_node);
                            return ass_node;
                        }

                        x => {
                            dart_parseerror(
                                format!("Unexpected token in statement. Got: {}. Expected: {}", x, "="),
                                ctx,
                                reader.tokens(),
                                reader.position()
                            );
                        }
                    }
                }

                Token::Assign(_, _) => {
                    reader.next();
                    reader.next();

                    let right_node = expression(reader, ctx);

                    let var = Node::new(NodeType::Name(s.to_string()));
                    let mut ass_node = Node::new(NodeType::Assign);
                    ass_node.children.push(var);
                    ass_node.children.push(right_node);
                    return ass_node;
                }

                Token::Paren1(_, _) => {
                    reader.next();
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

                    match reader.next() {

                        Token::Name(acc_name, _, _) => {

                            return match reader.next() {
                                Token::Paren1(_, _) => {

                                    // Method call
                                    let args = arglist(reader, ctx);
                                    let mut methcall_node = Node::new(NodeType::MethodCall(s.to_string(), acc_name.to_string()));
                                    methcall_node.children.push(args);
                                    methcall_node
                                }

                                _ => {
                                    let mut acc_node = Node::new(NodeType::Access);
                                    let obj_node = Node::new(NodeType::Name(s.to_string()));
                                    let member_node = Node::new(NodeType::Name(acc_name.to_string()));
                                    acc_node.children.push(obj_node);
                                    acc_node.children.push(member_node);
                                    acc_node
                                }
                            }
                        }

                        x => {
                            panic!("Unexpected token following '.': {}", x)
                        }
                    }
                }

                _ => {
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
            reader.skip("(", ctx);

            let boolexpr = expression(reader, ctx);

            reader.skip(")", ctx);
            reader.skip("{", ctx);

            let blocknode = block(reader, ctx);

            let mut node = Node::new(NodeType::While);
            node.children.push(boolexpr);
            node.children.push(blocknode);
            return node;
        }

        Token::Do(_, _) => {

            reader.next();
            reader.skip("{", ctx);

            let blocknode = block(reader, ctx);

            reader.skip("while", ctx);
            reader.skip("(", ctx);

            let boolexpr = expression(reader, ctx);

            reader.skip(")", ctx);

            let mut node = Node::new(NodeType::DoWhile);
            node.children.push(blocknode);
            node.children.push(boolexpr);
            return node;
        }

        Token::For(_, _) => {

            reader.next();
            reader.skip("(", ctx);

            match reader.sym() {

                Token::Name(n1, _, _) => {
                    reader.next();

                    match reader.sym() {
                        Token::Name(n2, _, _) => {
                            reader.next();

                            let typvar = Node::new(NodeType::TypedVar(n1, n2));

                            reader.skip("=", ctx);

                            let initexpr = expression(reader, ctx);

                            let mut assign = Node::new(NodeType::Assign);
                            assign.children.push(typvar);
                            assign.children.push(initexpr);

                            reader.skip(";", ctx);

                            let condexpr = expression(reader, ctx);

                            reader.skip(";", ctx);

                            let mutexpr = expression(reader, ctx);

                            reader.skip(")", ctx);
                            reader.skip("{", ctx);

                            let body = block(reader, ctx);

                            let mut forloop = Node::new(NodeType::For);
                            forloop.children.push(assign);
                            forloop.children.push(condexpr);
                            forloop.children.push(mutexpr);
                            forloop.children.push(body);
                            return forloop;
                        }

                        // Without declaration
                        // for (i=x; ...
                        Token::Assign(_, _) => {

                            reader.next();
                            let initexpr = expression(reader, ctx);

                            let mut assign = Node::new(NodeType::Assign);
                            let namenode = Node::new(NodeType::Name(n1));

                            assign.children.push(namenode);
                            assign.children.push(initexpr);

                            reader.skip(";", ctx);

                            let condexpr = expression(reader, ctx);

                            reader.skip(";", ctx);

                            let mutexpr = expression(reader, ctx);

                            reader.skip(")", ctx);
                            reader.skip("{", ctx);

                            let body = block(reader, ctx);

                            let mut forloop = Node::new(NodeType::For);
                            forloop.children.push(assign);
                            forloop.children.push(condexpr);
                            forloop.children.push(mutexpr);
                            forloop.children.push(body);
                            return forloop;
                        }

                        x => {
                            dart_parseerror(
                                format!("Expected identifier or assignment. Got: {}", x),
                                ctx,
                                &reader.tokens(),
                                reader.position()
                            );
                        }

                    }
                }

                _ => {
                    dart_parseerror(
                        "Expected identifier.",
                        ctx,
                        &reader.tokens(),
                        reader.position()
                    );
                }
            }
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
    dprint(format!("Parse: conditional: {}", reader.sym()));

    match reader.sym() {

        Token::If(_, _) => {
            reader.next();
            reader.skip("(", ctx);

            let boolnode = expression(reader, ctx);

            reader.skip(")", ctx);
            reader.skip("{", ctx);

            let bodynode = block(reader, ctx);

            let mut ifnode = Node::new(NodeType::If);
            ifnode.children.push(boolnode);
            ifnode.children.push(bodynode);
            return ifnode;
        }
        Token::Else(_, _) => {

            reader.next();

            match reader.sym() {

                Token::If(_, _) => {

                    reader.next();
                    reader.skip("(", ctx);

                    let boolnode = expression(reader, ctx);

                    reader.skip(")", ctx);
                    reader.skip("{", ctx);

                    let bodynode = block(reader, ctx);

                    let mut elseifnode = Node::new(NodeType::ElseIf);
                    elseifnode.children.push(boolnode);
                    elseifnode.children.push(bodynode);
                    return elseifnode;
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
