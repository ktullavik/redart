use queues::*;
use crate::state::State;
use crate::reader::Reader;
use crate::token::Token;
use crate::node::{NodeType, Node};
use crate::parser::arglist;
use crate::error::parseerror;


pub fn expression(reader: &mut Reader, ctx: &State) -> Node {
    disjunction(reader, ctx)
}


// The following are operators as given by
// https://www.tutorialandexample.com/dart-operators-precedence-and-associativity
// Ordered from loose to tight.


// Various docs for many languages, including dart, specifies
// the || and && operators as left associative. However,
// since disjunction and conjunction are associative and have
// distinct precedence levels, it should not matter for result.
// This is also the case for bit-or, bit-xor and bit-and.
// Use the simpler right-tree parsing until proven stupid.

fn disjunction(reader: &mut Reader, state: &State) -> Node {

    let left = conjunction(reader, state);

    if !reader.more() {
        return left;
    }

    match reader.tok() {

        Token::LogOr(linenum, symnum) => {
            reader.next();
            let right = disjunction(reader, state);
            let mut disnode = Node::new(NodeType::LogOr(linenum, symnum));
            disnode.children.push(left);
            disnode.children.push(right);
            disnode
        }

        _ => left
    }
}


fn conjunction(reader: &mut Reader, state: &State) -> Node {

    let left = equality(reader, state);

    if !reader.more() {
        return left;
    }

    match reader.tok() {

        Token::LogAnd(linenum, symnum) => {
            reader.next();
            let right = conjunction(reader, state);
            let mut connode = Node::new(NodeType::LogAnd(linenum, symnum));
            connode.children.push(left);
            connode.children.push(right);
            connode
        }

        _ => left
    }
}


fn equality(reader: &mut Reader, state: &State) -> Node {

    let left = comparison(reader, state);

    if !reader.more() {
        return left;
    }

    match reader.tok() {

        Token::Equal(linenum, symnum) => {
            reader.next();
            let right = comparison(reader, state);
            let mut eqnode = Node::new(NodeType::Equal(linenum, symnum));
            eqnode.children.push(left);
            eqnode.children.push(right);
            eqnode
        }

        _ => left
    }
}


fn comparison(reader: &mut Reader, state: &State) -> Node {

    let left = bit_or(reader, state);

    if !reader.more() {
        return left;
    }

    match reader.tok() {

        Token::LessThan(linenum, symnum) => {
            reader.next();
            let right = bit_or(reader, state);
            let mut connode = Node::new(NodeType::LessThan(linenum, symnum));
            connode.children.push(left);
            connode.children.push(right);
            connode
        }
        Token::GreaterThan(linenum, symnum) => {
            reader.next();
            let right = bit_or(reader, state);
            let mut connode = Node::new(NodeType::GreaterThan(linenum, symnum));
            connode.children.push(left);
            connode.children.push(right);
            connode
        }
        Token::LessOrEq(linenum, symnum) => {
            reader.next();
            let right = bit_or(reader, state);
            let mut connode = Node::new(NodeType::LessOrEq(linenum, symnum));
            connode.children.push(left);
            connode.children.push(right);
            connode
        }
        Token::GreaterOrEq(linenum, symnum) => {
            reader.next();
            let right= bit_or(reader, state);
            let mut connode = Node::new(NodeType::GreaterOrEq(linenum, symnum));
            connode.children.push(left);
            connode.children.push(right);
            connode
        }
        _ => left
    }
}


fn bit_or(reader: &mut Reader, state: &State) -> Node {

    let left = bit_xor(reader, state);

    if !reader.more() {
        return left;
    }

    match reader.tok() {
        Token::BitOr(linenum, symnum) => {

            let mut node = Node::new(NodeType::BitOr(linenum, symnum));
            node.children.push(left);
            reader.next();
            let right = bit_or(reader, state);
            node.children.push(right);
            node
        }
        _ => left
    }
}


fn bit_xor(reader: &mut Reader, state: &State) -> Node {

    let left = bit_and(reader, state);

    if !reader.more() {
        return left;
    }

    match reader.tok() {
        Token::BitXor(linenum, symnum) => {

            let mut node = Node::new(NodeType::BitXor(linenum, symnum));
            node.children.push(left);
            reader.next();
            let right = bit_xor(reader, state);
            node.children.push(right);
            node
        }
        _ => left
    }
}


fn bit_and(reader: &mut Reader, state: &State) -> Node {

    let left= sum(reader, state);

    if !reader.more() {
        return left;
    }

    match reader.tok() {
        Token::BitAnd(linenum, symnum) => {

            let mut node = Node::new(NodeType::BitAnd(linenum, symnum));
            node.children.push(left);
            reader.next();
            let right = bit_and(reader, state);
            node.children.push(right);
            node
        }
        _ => left
    }
}


fn sum(reader: &mut Reader, state: &State) -> Node {
    sum_help(reader, &mut queue![], &mut queue![], state)
}


fn sum_help(reader: &mut Reader, righties: &mut Queue<Node>, ops: &mut Queue<Node>, state: &State) -> Node {

    let n = product(reader, state);
    righties.add(n).ok();

    if !reader.more() {
        return righties.remove().unwrap();
    }

    match reader.tok() {

        Token::Add(linenum, symnum) => {
            ops.add(Node::new(NodeType::Add(linenum, symnum))).ok();
            reader.next();
            let deeper = sum_help(reader, righties, ops, state);
            let mut node = ops.remove().unwrap();
            node.children.push(deeper);
            node.children.push(righties.remove().unwrap());
            node
        }
        Token::Sub(linenum, symnum) => {
            ops.add(Node::new(NodeType::Sub(linenum, symnum))).ok();
            reader.next();
            let deeper = sum_help(reader, righties, ops, state);
            let mut node = ops.remove().unwrap();
            node.children.push(deeper);
            node.children.push(righties.remove().unwrap());
            node
        }
        _ => {
            righties.remove().unwrap()
        }
    }
}


fn product(reader: &mut Reader, ctx: &State) -> Node {
    product_help(reader, &mut queue![], &mut queue![], ctx)
}


fn product_help(reader: &mut Reader, righties: &mut Queue<Node>, ops: &mut Queue<Node>, ctx: &State) -> Node {

    let n = postop(reader, ctx);

    righties.add(n).ok();

    if !reader.more() {
        return righties.remove().unwrap();
    }

    match reader.tok() {

        Token::Mul(linenum, symnum) => {
            ops.add(Node::new(NodeType::Mul(linenum, symnum))).ok();
            reader.next();
            let deeper = product_help(reader, righties, ops, ctx);
            let mut node = ops.remove().unwrap();
            node.children.push(deeper);
            node.children.push(righties.remove().unwrap());
            node
        }
        Token::Div(linenum, symnum) => {
            ops.add(Node::new(NodeType::Div(linenum, symnum))).ok();
            reader.next();
            let deeper = product_help(reader, righties, ops, ctx);
            let mut node = ops.remove().unwrap();
            node.children.push(deeper);
            node.children.push(righties.remove().unwrap());
            node
        }
        _ => {
            righties.remove().unwrap()
        }
    }
}


fn postop(reader: &mut Reader, ctx: &State) -> Node {

    let node = access_chain(reader, ctx);

    match reader.tok() {

        Token::Decrement(linenum, symnum) => {

            reader.next();

            match node.nodetype {
                NodeType::Name(s, name_linenum, name_symnum) => {
                    let mut decnode = Node::new(
                        NodeType::PostDecrement(linenum, symnum)
                    );
                    let n = Node::new(
                        NodeType::Name(s.clone(), name_linenum, name_symnum)
                    );
                    decnode.children.push(n);
                    decnode
                }
                x => parseerror(
                    format!("Invalid node for decrement: {}", x),
                    ctx,
                    reader.tok())
            }
        }

        Token::Increment(linenum, symnum) => {

            reader.next();

            match node.nodetype {
                NodeType::Name(s, name_linenum, name_symnum) => {
                    let mut decnode = Node::new(
                        NodeType::PostIncrement(linenum, symnum)
                    );
                    let n = Node::new(
                        NodeType::Name(s.clone(), name_linenum, name_symnum)
                    );
                    decnode.children.push(n);
                    decnode
                }
                x => parseerror(
                    format!("Invalid node for increment: {}", x),
                    ctx,
                    reader.tok())
            }        
        }

        _ => node
    }
}


fn access_chain(reader: &mut Reader, ctx: &State) -> Node {

    let n = term(reader, ctx);

    match reader.tok() {
        Token::Access(_, _) |
        Token::Brack1(_, _) => {
            access_help(reader, n, ctx)
        }
        _ => n
    }
}


pub fn access_help(reader: &mut Reader, owner: Node, ctx: &State) -> Node {

    match reader.tok() {

        Token::Access(_, _) => {

            match reader.next() {

                Token::Name(name, linenum, symnum) => {

                    if !reader.more() {
                        // This path is taken when we have string interpolation with dots.
                        // Eg: print("${a.b}");
                        let mut node = Node::new(
                            NodeType::Name(name.clone(), linenum, symnum)
                        );
                        node.children.push(owner);
                        return node;
                    }

                    match reader.next() {

                        Token::Paren1(_, _) => {
                            let node = arglist(reader, ctx);
                            let mut funcall_node = Node::new(
                                NodeType::MethodCall(
                                    name.to_string(),
                                    Box::new(owner),
                                    ctx.filepath.clone(),
                                    linenum,
                                    symnum
                            ));
                            funcall_node.children.push(node);
                            access_help(reader, funcall_node, ctx)
                        }

                        // FIXME, the next two are identical

                        Token::Brack1(_, _) => {
                            let mut node = Node::new(
                                NodeType::Name(
                                    name.clone(),
                                    linenum,
                                    symnum
                            ));
                            node.children.push(owner);
                            access_help(reader, node, ctx)
                        }

                        Token::Access(_, _) => {
                            let mut node = Node::new(
                                NodeType::Name(
                                    name.clone(),
                                    linenum,
                                    symnum
                            ));
                            node.children.push(owner);
                            access_help(reader, node, ctx)
                        }

                        _ => {
                            let mut node = Node::new(
                                NodeType::Name(name.clone(), linenum, symnum)
                            );
                            node.children.push(owner);
                            node
                        }
                    }
                }

                x => {
                    panic!("Expected name after accessor, got: {}", x)
                }
            }
        }
        Token::Brack1(linenum, symnum) => {
            reader.next();
            let index_node = expression(reader, ctx);
            reader.skip("]", ctx);
            let mut collaccess = Node::new(
                NodeType::CollAccess(linenum, symnum)
            );
            collaccess.children.push(owner);
            collaccess.children.push(index_node);
            if let Token::Brack1(_, _) = reader.tok() {
                return access_help(reader, collaccess, ctx);
            }
            if let Token::Access(_, _) = reader.tok() {
                return access_help(reader, collaccess, ctx);
            }
            collaccess
        }
        _ => owner
    }
}


fn term(reader: &mut Reader, state: &State) -> Node {

    match reader.tok() {

        Token::Int(val, linenum, symnum) => {
            reader.next();
            Node::new(NodeType::Int(val, linenum, symnum))
        }

        Token::Double(val, linenum, symnum) => {
            reader.next();
            Node::new(NodeType::Double(val, linenum, symnum))
        }

        Token::Add(_, _) => {
            // As Dart.
            parseerror(
                "'+' is not a prefix operator.",
                state,
                reader.tok(),
            );
        }

        Token::Sub(linenum, symnum) => {
            // This handles unary minus.
            reader.next();
            let mut unary = Node::new(NodeType::Sub(linenum, symnum));
            let next = term(reader, state);
            unary.children.push(next);
            unary
        }

        Token::Not(linenum, symnum) => {
            reader.next();
            let mut notnode = Node::new(NodeType::Not(linenum, symnum));
            let next = term(reader, state);
            notnode.children.push(next);
            notnode
        }

        Token::Str(ref s, interpols, linenum, symnum) => {

            let mut node = Node::new(NodeType::Str(s.clone(), linenum, symnum));

            if interpols.is_empty() {
                reader.next();
                return node;
            }

            for itp in interpols {
                let mut r = Reader::new(itp);
                let itpn = expression(&mut r, state);
                node.children.push(itpn);
            }
            // May be empty when inside interpol recursion.
            if reader.more() {
                reader.next();
            }
            node
        }

        Token::Bool(v, linenum, symnum) => {
            reader.next();
            Node::new(NodeType::Bool(v, linenum, symnum))
        }

        Token::Name(ref s, linenum, symnum) => {

            if reader.more() {
                match reader.next() {
                    Token::Paren1(_, _) => {
                        // Function call.
                        let args_node = arglist(reader, state);
                        let mut funcall_node = Node::new(
                            NodeType::FunCall(s.to_string(), linenum, symnum)
                        );
                        funcall_node.children.push(args_node);
                        return funcall_node;
                    }
                    Token::Brack1(_, _) => {
                        let node = Node::new(
                            NodeType::Name(s.clone(), linenum, symnum)
                        );
                        return access_help(reader, node, state);
                    }
                    _ => {}
                }
            }
            Node::new(NodeType::Name(s.clone(), linenum, symnum))
        }

        Token::Increment(linenum, symnum) => {

            match reader.next() {
                Token::Name(name, name_linenum, name_symnum) => {
                    reader.next();
                    let mut node = Node::new(
                        NodeType::PreIncrement(linenum, symnum)
                    );
                    node.children.push(Node::new(
                        NodeType::Name(name, name_linenum, name_symnum))
                    );
                    node
                }
                x => panic!("Invalid operand for increment: {}", x)
            }
        }

        Token::Decrement(linenum, symnum) => {

            match reader.next() {
                Token::Name(name, name_linenum, name_symnum) => {
                    reader.next();
                    let mut node = Node::new(
                        NodeType::PreDecrement(linenum, symnum)
                    );
                    node.children.push(Node::new(
                        NodeType::Name(name, name_linenum, name_symnum))
                    );
                    node
                }
                x => panic!("Invalid operand for decrement: {}", x)
            }
        }

        Token::Paren1(_, _) => {
            reader.next();
            let wnode = expression(reader, state);
            reader.skip(")", state);
            wnode
        }

        Token::Brack1(linenum, symnum) => {

            reader.next();
            let mut list_node = Node::new(NodeType::List(linenum, symnum));
            let mut expect_sep = false;

            match reader.tok() {

                Token::Brack2(_, _) => {
                    reader.next();
                    list_node
                }

                _ => {

                    while reader.pos() < reader.len() {

                        if expect_sep {
                            match reader.tok() {

                                Token::Comma(_, _) => {
                                    if !expect_sep {
                                        panic!("Expected an identifier, but got ','");
                                    }
                                    reader.next();
                                    expect_sep = false;
                                    continue;
                                }

                                Token::Brack2(_, _) => {
                                    reader.next();
                                    break;
                                }
                                _ => panic!("Unexpected token when parsing list: {}", reader.tok())
                            }
                        }
                        expect_sep = true;
                        let entry = expression(reader, state);
                        list_node.children.push(entry);
                    }
                    list_node
                }
            }
        }

        Token::This(linenum, symnum) => {
            reader.next();
            Node::new(NodeType::This(linenum, symnum))
        }

        x => {
            panic!("Unexpected token: {}", x)
        }
    }
}