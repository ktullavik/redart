use state::State;
use reader::Reader;
use token::Token;
use node::{NodeType, Node};
use expression::expression;
use utils::{dprint, dart_parseerror};
use object::{ParamObj, Object};
use objsys::Class;
use crate::{expression::access_help, utils::dart_evalerror};


fn autoincludes() -> Vec<String> {
    vec![
        "auto:list.dart".to_string()
    ]
}


pub fn parse(reader: &mut Reader, ctx: &mut State) -> Vec<String> {

    dprint(" ");
    dprint("PARSE");
    dprint(" ");

    let imports = directives(reader, ctx);

    while reader.more() {
        decl(reader, ctx);
    }
    assert_eq!(reader.pos(), reader.len() - 1, "Unexpected index at end of parse: {} out of {}", reader.pos(), reader.len());

    return imports;
}


fn directives(reader: &mut Reader, ctx: &State) -> Vec<String> {
    dprint(format!("Parse: directives: {}", reader.sym()));

    let mut imports = autoincludes();

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


fn decl(reader: &mut Reader, state: &mut State) {
    dprint(format!("Parse: decl: {}", reader.sym()));

    match reader.sym() {

        // The type of a top level declaration.
        Token::Name(typ, _, _) => {

            match reader.next() {

                // Name of top level declaration.
                Token::Name(name, _, _) => {

                    match reader.next() {

                        Token::Paren1(_, _) => {
                            // Top level function
                            let mut node = Node::new(NodeType::FunDef(name.to_string(), state.filepath.clone()));
                            let params = paramlist(reader, state);
                            node.children.push(params);

                            reader.skip("{", state);
                            let body = block(reader, state);
                            node.children.push(body);
                            state.globals.push(node.clone());
                            return;
                        }

                        Token::Assign(_, _) => {
                            // Top level variable.
                            // These are lazy initialized (by first access) in dart.
                            // So we put the node tree into globals and then the
                            // interpreter will replace the TopVarLazy with a TopVar
                            // upon execution.
                            reader.next();
                            let mut node = Node::new(NodeType::TopVarLazy(typ, name));
                            let val = expression(reader, state);
                            node.children.push(val);
                            state.globals.push(node);
                            reader.skip(";", state);
                            return;
                        }

                        _ => dart_parseerror(format!("Unexpected token: {}",
                                reader.sym()),
                                state,
                                reader.tokens(),
                                reader.pos()
                            )
                    }
                }

                _ => {
                    panic!("Expected function name.")
                }
            }
        }

        Token::Const(_, _) => {

            println!("Found const");

            match reader.next() {

                Token::Name(type_or_name, _, _) => {

                    match reader.next() {

                        Token::Name(name, _, _) => {
                            reader.next();
                            reader.skip("=", state);
                            let val = expression(reader, state);
                            reader.skip(";", state);
                            let mut node = Node::new(
                                NodeType::ConstTopLazy(type_or_name, name));
                            node.children.push(val);
                            state.globals.push(node);
                            return;
                        }

                        Token::Assign(_, _) => {
                            reader.next();
                            let val = expression(reader, state);
                            reader.skip(";", state);
                            let mut node = Node::new(
                                NodeType::ConstTopLazy(String::from("dynamic"), type_or_name.clone()));
                            node.children.push(val);
                            state.globals.push(node);
                            println!("Inserted const: {}", type_or_name);
                            return;
                        }

                        x => dart_parseerror(
                            format!("Unexpected token: {}", x),
                            state,
                            reader.tokens(),
                            reader.pos()
                        )
                    }
                }

                _ => dart_parseerror(
                        "Expected name after 'const'.",
                        state,
                        reader.tokens(),
                        reader.pos()
                    )
            }
        }

        Token::Class(_, _) => {
            class(reader, state);
        }

        Token::Import(_, _) => {
            // As Dart.
            dart_parseerror(
                "Directives must appear before any declarations.",
                state,
                &reader.tokens(),
                reader.pos()
            );
        }

        x => {
            panic!("Expected top level declaration. Got {}", x);
        }
    }
}


fn class(reader: &mut Reader, state: &mut State) {
    dprint(format!("Parse: class: {}", reader.sym()));

    match reader.next() {
        Token::Name(classname, _, _) => {

            let mut class = Class::new(classname.clone());

            match reader.next() {

                Token::Extends(_, _) => {

                    match reader.next() {
                        
                        Token::Name(parentname, _, _) => {
                            class.parent = parentname;

                            match reader.next() {

                                Token::Block1(_, _) => {
                                    reader.next();
                                    readmembers(&mut class, reader, state);
                                    reader.skip("}", state);
                                }
                
                                x => dart_parseerror(
                                    format!("Unexpected token: {}", x),
                                    state,
                                    reader.tokens(),
                                    reader.pos()
                                )
                            }
                        }
                        
                        x => dart_evalerror(
                            format!("Expected parent class name. Got: {}", x),
                            state
                        )
                    }
                }

                Token::Block1(_, _) => {
                    reader.next();
                    readmembers(&mut class, reader, state);
                    reader.skip("}", state);
                }

                x => dart_parseerror(
                    format!("Unexpected token: {}", x),
                    state,
                    reader.tokens(),
                    reader.pos()
                )
            }



            // reader.skip("{", state);
            // readmembers(&mut class, reader, state);
            // reader.skip("}", state);
            state.objsys.register_class(class);
        }

        x => {
            panic!("Error: Expected class name. Got: {}", x)
        }
    }
}


// Expecting member declaration - field or method, or constructor.
fn readmembers(class: &mut Class, reader: &mut Reader, state: &mut State) {
    dprint(format!("Parse: readmembers: {}", reader.sym()));

    let mut got_contructor = false;

    while reader.more() {

        match reader.sym() {

            Token::Name(mtype, _, _) => {

                if *mtype == class.name {
                    // Constructor

                    reader.next();

                    let mut constructor_node = Node::new(NodeType::Constructor(class.name.clone(), state.filepath.clone()));
                    let params = constructor_paramlist(reader, state);
                    constructor_node.children.push(params);

                    match reader.sym() {

                        Token::Block1(_, _) => {
                            reader.next();
                            let body  = block(reader, state);
                            constructor_node.children.push(body);
                        }

                        Token::EndSt(_, _) => {
                            reader.next();
                            constructor_node.children.push(Node::new(NodeType::Null));
                        }

                        x => {
                            dart_parseerror(
                                format!("Expected constructor body, got: {}", x),
                                state,
                                reader.tokens(),
                                reader.pos()
                            )
                        }
                    }

                    got_contructor = true;
                    state.globals.push(constructor_node);
                    continue;
                }

                match reader.next() {

                    Token::Name(fieldname, _, _) => {

                        match reader.next() {

                            Token::Paren1(_, _) => {
                                // Method

                                let param_node = paramlist(reader, state);

                                reader.skip("{", state);

                                let body = block(reader, state);

                                let mut args: Vec<ParamObj> = Vec::new();

                                for i in 0..param_node.children.len() {
                                    let p = &param_node.children[i];
                                    match &p.nodetype {
                                        NodeType::Name(s) => {
                                            args.push(ParamObj{typ: String::from("var"), name: s.clone(), fieldinit: false});
                                        }
                                        NodeType::TypedVar(t, s) => {
                                            args.push(ParamObj{typ: t.to_string(), name: s.clone(), fieldinit: false});
                                        }
                                        x => panic!("Invalid parameter: {}", x)
                                    }
                                }

                                let methodobj = Object::Function(fieldname.to_string(), state.filepath.clone(), body, args);
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

                                let val = expression(reader, state);

                                reader.skip(";", state);

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

    if !got_contructor {
        // Class without constructor. Add an implicit one.
        let mut constructor_node = Node::new(NodeType::Constructor(class.name.clone(), state.filepath.clone()));
        constructor_node.children.push(Node::new(NodeType::ParamList));
        constructor_node.children.push(Node::new(NodeType::Null));
        state.globals.push(constructor_node);
    }

}


fn constructor_paramlist(reader: &mut Reader, state: &State) -> Node {
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
                    reader.skip(".", state);

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
                                state,
                                reader.tokens(),
                                reader.pos()
                            );
                        }
                    }
                }

                Token::Comma(_, _) => {
                    if !expect_comma {
                        // As dart.
                        dart_parseerror(
                            "Expected an identifier, but got ','.",
                            state, reader.tokens(),
                            reader.pos()
                        );
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
            state,
            reader.tokens(),
            reader.pos() - 1
        )
    }
    panic!("Error when reading param list.")
}


fn paramlist(reader: &mut Reader, state: &State) -> Node {
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

                    if let Token::Name(s2, _, _) = reader.peek() {

                        reader.next();

                        let param= Node::new(NodeType::TypedVar(s.to_string(), s2.to_string()));
                        node.children.push(param);
                        expect_comma = true;
                        reader.next();
                    }
                    else {
                        let param= Node::new(NodeType::Name(s.to_string()));
                        node.children.push(param);
                        expect_comma = true;
                        reader.next();
                    }
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
            state,
            reader.tokens(),
            reader.pos() - 1
        )
    }
    panic!("Error when reading param list.")
}


pub fn arglist(reader: &mut Reader, state: &State) -> Node {
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
                    let arg = expression(reader, state);
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
fn block(reader: &mut Reader, state: &State) -> Node {
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
                let snode = statement(reader, state);
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

                    // Why? Should'nt it always be EndSt or Block2?
                    _ => continue
                }
            }
        }
    }

    return node
}


fn assign_help(left_node: Node, reader: &mut Reader, state: &State) -> Node {
    reader.next();
    let right_node = expression(reader, state);
    let mut ass_node = Node::new(NodeType::Assign);
    ass_node.children.push(left_node);
    ass_node.children.push(right_node);
    return ass_node;
}


fn statement(reader: &mut Reader, state: &State) -> Node {
    dprint(format!("Parse: statement: {}", reader.sym()));

    match reader.sym() {

        Token::Name(s, _, _) => {

            match reader.peek() {

                Token::Name(name, _, _) => {
                    // Two names in a row here indicates a typed variable or nested function declaration.

                    let typed_var = Node::new(NodeType::TypedVar(s.to_string(), name.to_string()));
                    reader.next();
                    reader.next();

                    match reader.sym() {

                        Token::Assign(_, _) => {
                            assign_help(typed_var, reader, state)
                        }

                        Token::Paren1(_, _) => {
                            // Nested function declaration.

                            let params = paramlist(reader, state);
                            reader.skip("{", state);
                            let body = block(reader, state);

                            let mut funcnode = Node::new(NodeType::FunDef(name.clone(), state.filepath.clone()));
                            funcnode.children.push(params);
                            funcnode.children.push(body);
                            funcnode
                        }

                        x => {
                            panic!("Unexpected token: {}", x)
                        }
                    }
                }

                Token::Access(_, _) => {
                    reader.next();
                    let owner = Node::new(NodeType::Name(s));
                    let left_node = access_help(reader, owner, state);

                    match reader.sym() {
                        Token::Assign(_, _) => {
                            assign_help(left_node, reader, state)
                        }
                        _ => left_node
                    }
                }

                Token::Brack1(_, _) => {
                    reader.next();
                    let owner = Node::new(NodeType::Name(s));
                    let left_node = access_help(reader, owner, state);

                    match reader.sym() {
                        Token::Assign(_, _) => {
                            assign_help(left_node, reader, state)
                        }
                        _ => left_node
                    }
                }


                Token::Assign(_, _) => {
                    reader.next();
                    let left_node = Node::new(NodeType::Name(s.to_string()));
                    assign_help(left_node, reader, state)
                }

                _ => {
                    expression(reader, state)
                }
            }
        }

        Token::If(_, _) => {
            dprint("Parse: if");

            let mut condnode = Node::new(NodeType::Conditional);

            let condpart = conditional(reader, state);
            condnode.children.push(condpart);


            loop {

                let next_token = reader.sym();

                match next_token {

                    Token::Else(_, _) => {
                        dprint("Parse: if-else");

                        let lastcond = conditional(reader, state);
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
            reader.skip("(", state);

            let boolexpr = expression(reader, state);

            reader.skip(")", state);
            reader.skip("{", state);

            let blocknode = block(reader, state);

            let mut node = Node::new(NodeType::While);
            node.children.push(boolexpr);
            node.children.push(blocknode);
            return node;
        }

        Token::Do(_, _) => {

            reader.next();
            reader.skip("{", state);

            let blocknode = block(reader, state);

            reader.skip("while", state);
            reader.skip("(", state);

            let boolexpr = expression(reader, state);

            reader.skip(")", state);

            let mut node = Node::new(NodeType::DoWhile);
            node.children.push(blocknode);
            node.children.push(boolexpr);
            return node;
        }

        Token::For(_, _) => {

            reader.next();
            reader.skip("(", state);

            match reader.sym() {

                Token::Name(n1, _, _) => {
                    reader.next();

                    match reader.sym() {
                        Token::Name(n2, _, _) => {
                            reader.next();

                            let typvar = Node::new(NodeType::TypedVar(n1, n2));

                            match reader.sym() {

                                Token::Assign(_, _) => {
                                    reader.skip("=", state);

                                    let initexpr = expression(reader, state);
        
                                    let mut assign = Node::new(NodeType::Assign);
                                    assign.children.push(typvar);
                                    assign.children.push(initexpr);
        
                                    reader.skip(";", state);
        
                                    let condexpr = expression(reader, state);
        
                                    reader.skip(";", state);
        
                                    let mutexpr = expression(reader, state);
        
                                    reader.skip(")", state);
                                    reader.skip("{", state);
        
                                    let body = block(reader, state);
        
                                    let mut forloop = Node::new(NodeType::For);
                                    forloop.children.extend([assign, condexpr, mutexpr, body]);
                                    return forloop;
                                }

                                Token::In(_, _) => {
                                    reader.next();

                                    let iterable = expression(reader, state);
                                    reader.skip(")", state);
                                    reader.skip("{", state);
                                    let body = block(reader, state);

                                    let mut forloop = Node::new(NodeType::For);
                                    // let mut in_node = Node::new(NodeType::In);
                                    forloop.children.push(typvar);
                                    forloop.children.push(iterable);
                                    // forloop.children.push(in_node);
                                    forloop.children.push(body);
                                    return forloop;
                                }

                                x => dart_parseerror(
                                    format!("Unexpected token in for-loop: {}", x),
                                    state,
                                    &reader.tokens(),
                                    reader.pos()
                                )
                            }
 
                        }

                        // Without declaration
                        // for (i=x; ...
                        Token::Assign(_, _) => {

                            reader.next();
                            let initexpr = expression(reader, state);

                            let mut assign = Node::new(NodeType::Assign);
                            let namenode = Node::new(NodeType::Name(n1));

                            assign.children.push(namenode);
                            assign.children.push(initexpr);

                            reader.skip(";", state);

                            let condexpr = expression(reader, state);

                            reader.skip(";", state);

                            let mutexpr = expression(reader, state);

                            reader.skip(")", state);
                            reader.skip("{", state);

                            let body = block(reader, state);

                            let mut forloop = Node::new(NodeType::For);
                            forloop.children.extend([assign, condexpr, mutexpr, body]);

                            return forloop;
                        }

                        x => {
                            dart_parseerror(
                                format!("Expected identifier or assignment. Got: {}", x),
                                state,
                                &reader.tokens(),
                                reader.pos()
                            );
                        }

                    }
                }

                _ => {
                    dart_parseerror(
                        "Expected identifier.",
                        state,
                        &reader.tokens(),
                        reader.pos()
                    );
                }
            }
        }

        Token::Return(_, _) => {
            reader.next();
            let val = expression(reader, state);
            let mut ret = Node::new(NodeType::Return);
            ret.children.push(val);
            return ret;
        }

        _ => {
            return expression(reader, state);
        }
    }
}


fn conditional(reader: &mut Reader, ctx: &State) -> Node {
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
