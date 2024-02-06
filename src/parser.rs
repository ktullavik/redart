use context::Ctx;
use reader::Reader;
use token::Token;
use node::{NodeType, Node};
use expression::expression;
use utils::{dprint, dart_parseerror};
use object::{ParamObj, Object};
use objsys::{Class, ObjSys};


pub fn parse(reader: &mut Reader,
             globals: &mut Vec<Node>,
             objsys: &mut ObjSys,
             ctx: &Ctx) -> Vec<String> {

    dprint(" ");
    dprint("PARSE");
    dprint(" ");

    let imports = directives(reader, ctx);

    while reader.more() {
        decl(reader, objsys, globals, ctx);
    }
    assert_eq!(reader.pos(), reader.len() - 1, "Unexpected index at end of parse: {} out of {}", reader.pos(), reader.len());

    return imports;
}


fn directives(reader: &mut Reader, ctx: &Ctx) -> Vec<String> {
    dprint(format!("Parse: directives: {}", reader.sym()));

    let mut imports : Vec<String> = Vec::new();

    while reader.more() {

        match reader.sym() {
            Token::Import(_, _) => {

                reader.next();
                if let Token::Str(s, _, _, _) = reader.sym() {
                    reader.next();
                    reader.skip(";", ctx);

                    imports.push(s.clone());
                }
                else {
                    panic!("Error: Expected string after 'import'.")
                }
            }
            _  => break
        }
    }
    imports
}


fn decl(reader: &mut Reader, objsys: &mut ObjSys, globals: &mut Vec<Node>, ctx: &Ctx) {
    dprint(format!("Parse: decl: {}", reader.sym()));

    match reader.sym() {

        // The return type of a top level function definition.
        Token::Name(_, _, _) => {

            match reader.next() {

                // Name of top level function.
                Token::Name(fname, _, _) => {
                    reader.next();
                    let mut node = Node::new(NodeType::FunDef(fname.to_string(), ctx.filepath.clone()));
                    let params = paramlist(reader, ctx);
                    node.children.push(params);

                    reader.skip("{", ctx);
                    let body = block(reader, ctx);
                    node.children.push(body);
                    globals.push(node.clone());
                    return;
                }

                _ => {
                    panic!("Expected function name.")
                }
            }
        }

        Token::Class(_, _) => {
            class(reader, objsys, globals, ctx);
        }

        Token::Import(_, _) => {
            // As Dart.
            dart_parseerror(
                "Directives must appear before any declarations.",
                ctx,
                &reader.tokens(),
                reader.pos()
            );
        }

        x => {
            panic!("Expected top level declaration. Got {}", x);
        }
    }
}


fn class(reader: &mut Reader, objsys:&mut ObjSys, globals: &mut Vec<Node>, ctx: &Ctx) {
    dprint(format!("Parse: class: {}", reader.sym()));

    match reader.next() {
        Token::Name(classname, _, _) => {

            let mut class = objsys.new_class(classname.clone());

            reader.next();
            reader.skip("{", ctx);
            readmembers(&mut class, reader, globals, ctx);
            reader.skip("}", ctx);
            objsys.register_class(class);
        }

        x => {
            panic!("Error: Expected class name. Got: {}", x)
        }
    }
}


// Expecting member declaration - field or method, or constructor.
fn readmembers(class: &mut Class, reader: &mut Reader, globals: &mut Vec<Node>, ctx: &Ctx) {
    dprint(format!("Parse: readmembers: {}", reader.sym()));

    while reader.more() {

        match reader.sym() {

            Token::Name(mtype, _, _) => {

                if *mtype == class.name {
                    // Constructor

                    reader.next();

                    let mut constructor_node = Node::new(NodeType::Constructor(class.name.clone(), ctx.filepath.clone()));
                    let params = constructor_paramlist(reader, ctx);
                    reader.next();
                    let body  = block(reader, ctx);

                    constructor_node.children.push(params);
                    constructor_node.children.push(body);

                    globals.push(constructor_node);
                    continue;
                }

                match reader.next() {

                    Token::Name(fieldname, _, _) => {

                        match reader.next() {

                            Token::Paren1(_, _) => {
                                // Method

                                let param_node = paramlist(reader, ctx);

                                reader.skip("{", ctx);

                                let body = block(reader, ctx);

                                let mut args: Vec<ParamObj> = Vec::new();

                                for i in 0..param_node.children.len() {
                                    let p = &param_node.children[i];
                                    match &p.nodetype {
                                        NodeType::Name(s) => {
                                            args.push(ParamObj{typ: String::from("var"), name: s.clone(), fieldinit: false});
                                        }
                                        x => panic!("Invalid parameter: {}", x)
                                    }
                                }

                                let methodobj = Object::Function(fieldname.to_string(), ctx.filepath.clone(), body, args);
                                class.add_method(fieldname.clone(), methodobj);
                            }

                            Token::EndSt(_, _) => {
                                // Uninitialized field declare
                                reader.next();
                                class.add_field(mtype, fieldname, Node::new(NodeType::Null));
                            }

                            Token::Assign(_, _) => {
                                // Initialized field declare
                                reader.next();

                                let val = expression(reader, ctx);

                                if let Token::EndSt(_, _) = reader.sym() {
                                    reader.next();
                                }
                                else {
                                    panic!("Expected ';' after field initialization.")
                                }

                                class.add_field(mtype, fieldname, val);
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
                            let this_fieldinit = Node::new(NodeType::ThisFieldInit(s));
                            node.children.push(this_fieldinit);
                            expect_comma = true;
                            reader.next();
                        }

                        x => {
                            dart_parseerror(
                                format!("Expected identifier. Got {}", x),
                                ctx,
                                reader.tokens(),
                                reader.pos()
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
            reader.pos() - 1
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
            reader.pos() - 1
        )
    }
    panic!("Error when reading param list.")
}


pub fn arglist(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: arglist: {}", reader.sym()));

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
        dprint(format!("Parse: block loop at: {}, token: {}", reader.pos(), reader.sym()));

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
                    // Two names in a row here indicates a typed variable.

                    let typed_var = Node::new(NodeType::TypedVar(s.to_string(), name.to_string()));

                    reader.next();
                    reader.next();
                    reader.skip("=", ctx);

                    let right_node = expression(reader, ctx);

                    let mut ass_node = Node::new(NodeType::Assign);
                    ass_node.children.push(typed_var);
                    ass_node.children.push(right_node);
                    return ass_node;
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
                                reader.pos()
                            );
                        }

                    }
                }

                _ => {
                    dart_parseerror(
                        "Expected identifier.",
                        ctx,
                        &reader.tokens(),
                        reader.pos()
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
