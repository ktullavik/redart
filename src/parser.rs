use crate::state::State;
use crate::reader::Reader;
use crate::token::Token;
use crate::node::{NodeType, Node};
use crate::expression::expression;
use crate::error::parseerror;
use crate::object::{ParamObj, Object};
use crate::objsys::Class;
use crate::expression::access_help;


fn autoincludes() -> Vec<String> {
    vec![
        "auto:list.dart".to_string()
    ]
}


pub fn parse(reader: &mut Reader, ctx: &mut State) -> Vec<String> {

    let imports = directives(reader, ctx);

    while reader.more() {
        decl(reader, ctx);
    }
    assert_eq!(reader.pos(), reader.len() - 1, "Unexpected index at end of parse: {} out of {}", reader.pos(), reader.len());

    return imports;
}


fn directives(reader: &mut Reader, ctx: &State) -> Vec<String> {

    let mut imports = autoincludes();

    while reader.more() {

        match reader.tok() {
            Token::Import(_, _) => {

                reader.next();
                if let Token::Str(s, _, _, _) = reader.tok() {
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

    match reader.tok() {

        // The type of a top level declaration.
        Token::Name(typ, _, _) => {

            match reader.next() {

                // Name of top level declaration.
                Token::Name(name, linenum, symnum) => {

                    match reader.next() {

                        Token::Paren1(_, _) => {
                            // Top level function
                            let mut node = Node::new(
                                NodeType::FunDef(
                                    name.to_string(),
                                    state.filepath.clone(),
                                    linenum,
                                    symnum
                                ));
                            let params = paramlist(reader, state, false);
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
                            let mut node = Node::new(
                                NodeType::TopVarLazy(
                                    typ,
                                    name,
                                    linenum,
                                    symnum
                                ));
                            let val = expression(reader, state);
                            node.children.push(val);
                            state.globals.push(node);
                            reader.skip(";", state);
                            return;
                        }

                        _ => parseerror(
                                format!("Unexpected token: {}", reader.tok()),
                                state,
                                reader.tok()
                            )
                    }
                }

                _ => {
                    panic!("Expected function name.")
                }
            }
        }

        Token::Const(_, _) => {

            match reader.next() {

                Token::Name(type_or_name, linenum1, symnum1) => {

                    match reader.next() {

                        Token::Name(name, linenum2, symnum2) => {
                            reader.next();
                            reader.skip("=", state);
                            let val = expression(reader, state);
                            reader.skip(";", state);
                            let mut node = Node::new(
                                NodeType::ConstTopLazy(
                                    type_or_name,
                                    name,
                                    linenum2,
                                    symnum2
                                ));
                            node.children.push(val);
                            state.globals.push(node);
                            return;
                        }

                        Token::Assign(_, _) => {
                            reader.next();
                            let val = expression(reader, state);
                            reader.skip(";", state);
                            let mut node = Node::new(
                                NodeType::ConstTopLazy(
                                    String::from("dynamic"),
                                    type_or_name.clone(),
                                    linenum1,
                                    symnum1
                                ));
                            node.children.push(val);
                            state.globals.push(node);
                            return;
                        }

                        x => parseerror(
                            format!("Unexpected token: {}", x),
                            state,
                            reader.tok()
                        )
                    }
                }

                _ => parseerror(
                        "Expected name after 'const'.",
                        state,
                        reader.tok()
                    )
            }
        }

        Token::Class(_, _) => {
            class(reader, state);
        }

        Token::Import(_, _) => {
            // As Dart.
            parseerror(
                "Directives must appear before any declarations.",
                state,
                reader.tok()
            );
        }

        x => {
            parseerror(
                format!("Expected top level declaration. Got {}", x),
                state,
                x
        );
        }
    }
}


fn class(reader: &mut Reader, state: &mut State) {

    if let Token::Name(classname, _, _) = reader.next() {

        let mut class = Class::new(classname.clone());

        if let Token::Extends(_, _) = reader.next() {
            if let Token::Name(parentname, _, _) = reader.next() {
                class.parent = parentname;
                reader.next();
            }
            else {
                parseerror(
                    "Expected parent class name",
                    state,
                    reader.tok()
                );
            }
        }
        reader.skip("{", state);
        readmembers(&mut class, reader, state);
        reader.skip("}", state);
        state.objsys.register_class(class);
        return;
    }
    parseerror(
        "Expected class name",
        state,
        reader.tok()
    );
}


// Expecting member declaration - field or method, or constructor.
fn readmembers(class: &mut Class, reader: &mut Reader, state: &mut State) {

    let mut got_contructor = false;

    while reader.more() {

        match reader.tok() {

            Token::Name(mtype, linenum, symnum) => {

                if *mtype == class.name {
                    // Constructor

                    reader.next();

                    let mut body = Node::new(NodeType::Null(reader.linenum(), reader.symnum()));
                    let mut initlist = Node::new(NodeType::Null(reader.linenum(), reader.symnum()));
                    let params = paramlist(reader, state, true);

                    match reader.tok() {

                        Token::Colon(_, _) => {
                            initlist = initializer_list(reader, state);

                            if let Token::Block1(_, _) = reader.tok() {
                                reader.next();
                                body  = block(reader, state);
                            }    
                        }

                        Token::Block1(_, _) => {
                            reader.next();
                            body  = block(reader, state);
                        }

                        Token::EndSt(_, _) => {
                            reader.next();
                        }

                        x =>
                            parseerror(
                                format!("Unexpected token when parsing constructor: {}", reader.tok()),
                                state,
                                x
                            )
                    }

                    let constructor_node = Node::new(
                        NodeType::Constructor(
                            class.name.clone(),
                            Box::new(params),
                            Box::new(initlist),
                            Box::new(body),
                            state.filepath.clone(),
                            linenum,
                            symnum
                    ));

                    got_contructor = true;
                    state.globals.push(constructor_node);
                    continue;
                }

                match reader.next() {

                    Token::Name(fieldname, linenum, symnum) => {

                        match reader.next() {

                            Token::Paren1(_, _) => {
                                // Method

                                // FIXME, why can't param_node be used directly?
                                // Why do we need ParamObj which is not event a Node?
                                let param_node = paramlist(reader, state, false);

                                reader.skip("{", state);

                                let body = block(reader, state);

                                let mut args: Vec<ParamObj> = Vec::new();

                                for i in 0..param_node.children.len() {
                                    let p = &param_node.children[i];
                                    match &p.nodetype {
                                        NodeType::Name(s, _, _) => {
                                            args.push(ParamObj{typ: String::from("var"), name: s.clone(), fieldinit: false});
                                        }
                                        NodeType::TypedVar(t, s, _, _) => {
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
                                class.add_field(mtype, fieldname, Node::new(NodeType::Null(linenum, symnum)));
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

                    Token::Get(_, _) => {

                        match reader.next() {

                            Token::Name(gettername, _, _) => {
                                reader.next();
                                reader.skip("{", state);

                                let body = block(reader, state);

                                let getterfunc = Object::Function(gettername.to_string(), state.filepath.clone(), body, Vec::new());
                                class.add_getter(gettername.to_string(), getterfunc);
                            }

                            x => {
                                parseerror(
                                    "Expected name after 'get'",
                                    state,
                                    x
                                );
                            }
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
        let mut constructor_node = Node::new(
            NodeType::Constructor(
                class.name.clone(),
                Box::new(Node::new(NodeType::Null(reader.linenum(), reader.symnum()))),
                Box::new(Node::new(NodeType::Null(reader.linenum(), reader.symnum()))),
                Box::new(Node::new(NodeType::Null(reader.linenum(), reader.symnum()))),
                state.filepath.clone(),
                0,
                0
        ));
        constructor_node.children.push(
            Node::new(NodeType::ParamList(0, 0)));
        constructor_node.children.push(
            Node::new(NodeType::Null(0, 0)));
        state.globals.push(constructor_node);
    }

}


fn paramlist(reader: &mut Reader, state: &State, is_constructor: bool) -> Node {

    if let Token::Paren1(linenum, symnum) = reader.tok() {

        let mut node = Node::new(NodeType::ParamList(linenum, symnum));
        let mut expect_comma = false;
        reader.next();

        while reader.more() {

            match reader.tok() {

                Token::Paren2(_, _) => {
                    reader.next();
                    return node;
                }

                Token::This(_, _) => {

                    if !is_constructor {
                        // As dart.
                        parseerror(
                            "Initializing formal parameters can only be used in constructors",
                            state,
                            reader.tok()
                        );
                    }

                    reader.next();
                    reader.skip(".", state);

                    match reader.tok() {

                        Token::Name(s, linenum, symnum) => {
                            let this_fieldinit = Node::new(
                                NodeType::ThisFieldInit(s, linenum, symnum)
                            );
                            node.children.push(this_fieldinit);
                            expect_comma = true;
                            reader.next();
                        }

                        x => {
                            parseerror(
                                format!("Expected identifier. Got {}", x),
                                state,
                                reader.tok()
                            );
                        }
                    }
                }

                Token::Comma(_, _) => {
                    if !expect_comma {
                        // As dart.
                        parseerror(
                            "Expected an identifier, but got ','.",
                            state,
                            reader.tok() 
                        );
                    }
                    reader.next();
                    expect_comma = false;
                }

                Token::Name(s, linenum, symnum) => {

                    if let Token::Name(s2, linenum, symnum) = reader.peek() {

                        reader.next();

                        let param= Node::new(
                            NodeType::TypedVar(
                                s.to_string(),
                                s2.to_string(),
                                linenum,
                                symnum
                        ));
                        node.children.push(param);
                        expect_comma = true;
                        reader.next();
                    }
                    else {
                        let paramnode= Node::new(
                            NodeType::Name(
                                s.to_string(),
                                linenum,
                                symnum
                        ));
                        node.children.push(paramnode);
                        expect_comma = true;
                        reader.next();
                    }
                }

                _ => {
                    panic!("Unexpected token when reading parameters: {}", reader.tok())
                }
            }
        }
    }
    else {
        parseerror(
            "Expected parameter list after constructor declaration.",
            state,
            reader.tok()
        )
    }
    panic!("Error when reading param list.")
}


pub fn arglist(reader: &mut Reader, state: &State) -> Node {

    if let Token::Paren1(linenum, symnum) = reader.tok() {

        let mut node = Node::new(
            NodeType::ArgList(linenum, symnum)
        );
        let mut expect_comma = false;
        reader.next();

        while reader.more() {

            match reader.tok() {

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
        panic!("Error: Expected start of arglist: '('. Found: {}", reader.tok())
    }
    panic!("Error when reading arg list.")
}


fn initializer_list(reader: &mut Reader, state: &State) -> Node {

    let mut initlist = Node::new(NodeType::InitList(reader.linenum(), reader.symnum()));
    let mut got_super = false;
    let mut expect_comma = false;
    let start = reader.tok().clone();

    reader.next();

    while reader.more() {

        match reader.tok() {

            Token::Name(name, _, _) => {

                if got_super {
                    parseerror(
                        // As dart
                        "The superconstructor call must be last in the initializer list",
                        state,
                        reader.tok()
                    );
                }
                if expect_comma {
                    parseerror("Expected separator, got: '{}'", state, reader.tok());
                }

                let name_node = Node::new(NodeType::Name(name, reader.linenum(), reader.pos()));
                reader.next();
                reader.skip("=", state);

                let val_node = expression(reader, state);

                let mut init = Node::new(NodeType::Initializer(reader.linenum(), reader.symnum()));
                init.children.push(name_node);
                init.children.push(val_node);
                initlist.children.push(init);
                expect_comma = true;
            }

            Token::Super(_, _) => {

                if got_super {
                    parseerror(
                        // As dart
                        "Unexpected superconstructor call",
                        state,
                        reader.tok()
                    );
                }
                if expect_comma {
                    parseerror("Expected separator, got: '{}'", state, reader.tok());
                }

                got_super = true;
                let mut super_node = Node::new(NodeType::Super(reader.linenum(), reader.symnum()));
                reader.next();
                super_node.children.push(arglist(reader, state));
                initlist.children.push(super_node);
            }

            Token::Comma(_, _) => {
                if !expect_comma {
                    parseerror("Unexpected symbol: ','", state, reader.tok());
                }
                reader.next();
                expect_comma = false;
            }

            Token::EndSt(_, _) => {
                reader.next();
                break;
            }

            x => {
                parseerror(format!("Unexpected token: {}", x), state, x)
            }
        }
    }

    if initlist.children.len() == 0 {
        parseerror("Expected an initializer", state, start);
    }
    initlist
}


/// Parse a series of statements.
///
/// Expects first token after block started by {.
/// Consumes the end-block token }.
fn block(reader: &mut Reader, state: &State) -> Node {

    let mut node = Node::new(
        NodeType::Block(reader.linenum(), reader.symnum())
    );

    while reader.more() {

        match reader.tok() {

            Token::Block2(_, _) => {
                reader.next();
                break;
            }

            Token::End(_, _) => {
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

                match reader.tok() {

                    Token::Block2(_, _) => {
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
    let linenum = reader.linenum();
    let symnum = reader.symnum();
    reader.next();
    let right_node = expression(reader, state);
    let mut ass_node = Node::new(NodeType::Assign(linenum, symnum));
    ass_node.children.push(left_node);
    ass_node.children.push(right_node);
    return ass_node;
}


fn statement(reader: &mut Reader, state: &State) -> Node {

    match reader.tok() {

        Token::Name(s, name_linenum1, name_symnum1) => {

            match reader.peek() {

                Token::Name(name, name_linenum2, name_symnum2) => {
                    // Two names in a row here indicates a typed variable or nested function declaration.

                    let typed_var = Node::new(
                        NodeType::TypedVar(
                            s.to_string(),
                            name.to_string(),
                            name_linenum2,
                            name_symnum2
                    ));
                    reader.next();
                    reader.next();

                    match reader.tok() {

                        Token::Assign(_, _) => {
                            assign_help(typed_var, reader, state)
                        }

                        Token::Paren1(_, _) => {
                            // Nested function declaration.

                            let params = paramlist(reader, state, false);
                            reader.skip("{", state);
                            let body = block(reader, state);

                            let mut funcnode = Node::new(
                                NodeType::FunDef(
                                    name.clone(),
                                    state.filepath.clone(),
                                    name_linenum2,
                                    name_symnum2
                            ));
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
                    let owner = Node::new(
                        NodeType::Name(s, name_linenum1, name_symnum1)
                    );
                    let left_node = access_help(reader, owner, state);

                    match reader.tok() {
                        Token::Assign(_, _) => {
                            assign_help(left_node, reader, state)
                        }
                        _ => left_node
                    }
                }

                Token::Brack1(_, _) => {
                    reader.next();
                    let owner = Node::new(
                        NodeType::Name(s, name_linenum1, name_symnum1)
                    );
                    let left_node = access_help(reader, owner, state);

                    match reader.tok() {
                        Token::Assign(_, _) => {
                            assign_help(left_node, reader, state)
                        }
                        _ => left_node
                    }
                }

                Token::Assign(_, _) => {
                    reader.next();
                    let left_node = Node::new(
                        NodeType::Name(s.to_string(), name_linenum1, name_symnum1)
                    );
                    assign_help(left_node, reader, state)
                }

                _ => {
                    expression(reader, state)
                }
            }
        }

        Token::If(linenum, symnum) => {

            let mut condnode = Node::new(
                NodeType::Conditional(linenum, symnum)
            );

            let condpart = conditional(reader, state);
            condnode.children.push(condpart);

            loop {
                let next_token = reader.tok();

                match next_token {

                    Token::Else(_, _) => {
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

        Token::While(linenum, symnum) => {

            reader.next();
            reader.skip("(", state);

            let boolexpr = expression(reader, state);

            reader.skip(")", state);
            reader.skip("{", state);

            let blocknode = block(reader, state);

            let mut node = Node::new(NodeType::While(linenum, symnum));
            node.children.push(boolexpr);
            node.children.push(blocknode);
            return node;
        }

        Token::Do(linenum, symnum) => {

            reader.next();
            reader.skip("{", state);

            let blocknode = block(reader, state);

            reader.skip("while", state);
            reader.skip("(", state);

            let boolexpr = expression(reader, state);

            reader.skip(")", state);

            let mut node = Node::new(NodeType::DoWhile(linenum, symnum));
            node.children.push(blocknode);
            node.children.push(boolexpr);
            return node;
        }

        Token::For(for_linenum, for_symnum) => {

            reader.next();
            reader.skip("(", state);

            match reader.tok() {

                Token::Name(n1, name_linenum, name_symnum) => {
                    reader.next();

                    match reader.tok() {
                        Token::Name(n2, name_linenum2, name_symnum2) => {
                            reader.next();

                            let typvar = Node::new(
                                NodeType::TypedVar(
                                    n1, n2, name_linenum2, name_symnum2)
                            );

                            match reader.tok() {

                                Token::Assign(assign_linenum, assign_symnum) => {
                                    reader.skip("=", state);

                                    let initexpr = expression(reader, state);
        
                                    let mut assign = Node::new(
                                        NodeType::Assign(
                                            assign_linenum,
                                            assign_symnum
                                        ));
                                    assign.children.push(typvar);
                                    assign.children.push(initexpr);
        
                                    reader.skip(";", state);
        
                                    let condexpr = expression(reader, state);
        
                                    reader.skip(";", state);
        
                                    let mutexpr = expression(reader, state);
        
                                    reader.skip(")", state);
                                    reader.skip("{", state);
        
                                    let body = block(reader, state);
        
                                    let mut forloop = Node::new(
                                        NodeType::For(for_linenum, for_symnum
                                    ));
                                    forloop.children.extend([assign, condexpr, mutexpr, body]);
                                    return forloop;
                                }

                                Token::In(_, _) => {
                                    reader.next();

                                    let iterable = expression(reader, state);
                                    reader.skip(")", state);
                                    reader.skip("{", state);
                                    let body = block(reader, state);

                                    let mut forloop = Node::new(
                                        NodeType::For(for_linenum, for_symnum
                                    ));
                                    // let mut in_node = Node::new(NodeType::In);
                                    forloop.children.push(typvar);
                                    forloop.children.push(iterable);
                                    // forloop.children.push(in_node);
                                    forloop.children.push(body);
                                    return forloop;
                                }

                                x => parseerror(
                                    format!("Unexpected token in for-loop: {}", x),
                                    state,
                                    reader.tok()
                                )
                            }
                        }

                        // Without declaration
                        // for (i=x; ...
                        Token::Assign(assign_linenum, assign_symnum) => {

                            reader.next();
                            let initexpr = expression(reader, state);

                            let mut assign = Node::new(
                                NodeType::Assign(assign_linenum, assign_symnum
                                ));
                            let namenode = Node::new(
                                NodeType::Name(n1, name_linenum, name_symnum
                            ));

                            assign.children.push(namenode);
                            assign.children.push(initexpr);

                            reader.skip(";", state);

                            let condexpr = expression(reader, state);

                            reader.skip(";", state);

                            let mutexpr = expression(reader, state);

                            reader.skip(")", state);
                            reader.skip("{", state);

                            let body = block(reader, state);

                            let mut forloop = Node::new(
                                NodeType::For(for_linenum, for_symnum
                            ));
                            forloop.children.extend([assign, condexpr, mutexpr, body]);

                            return forloop;
                        }

                        x => {
                            parseerror(
                                format!("Expected identifier or assignment. Got: {}", x),
                                state,
                                reader.tok()
                            );
                        }
                    }
                }

                _ => {
                    parseerror(
                        "Expected identifier.",
                        state,
                        reader.tok()
                    );
                }
            }
        }

        Token::Return(linenum, symnum) => {
            reader.next();
            let val = expression(reader, state);
            let mut ret = Node::new(NodeType::Return(linenum, symnum));
            ret.children.push(val);
            return ret;
        }

        _ => {
            return expression(reader, state);
        }
    }
}


fn conditional(reader: &mut Reader, ctx: &State) -> Node {

    match reader.tok() {

        Token::If(linenum, symnum) => {
            reader.next();
            reader.skip("(", ctx);

            let boolnode = expression(reader, ctx);

            reader.skip(")", ctx);
            reader.skip("{", ctx);

            let bodynode = block(reader, ctx);

            let mut ifnode = Node::new(NodeType::If(linenum, symnum));
            ifnode.children.push(boolnode);
            ifnode.children.push(bodynode);
            return ifnode;
        }
        Token::Else(linenum, symnum) => {

            match reader.next() {

                Token::If(linenum, symnum) => {

                    reader.next();
                    reader.skip("(", ctx);

                    let boolnode = expression(reader, ctx);

                    reader.skip(")", ctx);
                    reader.skip("{", ctx);

                    let bodynode = block(reader, ctx);

                    let mut elseifnode = Node::new(NodeType::ElseIf(linenum, symnum));
                    elseifnode.children.push(boolnode);
                    elseifnode.children.push(bodynode);
                    return elseifnode;
                }

                Token::Block1(_, _) => {

                    reader.next();

                    let bodynode = block(reader, ctx);

                    let mut elsenode = Node::new(NodeType::Else(linenum, symnum));
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
