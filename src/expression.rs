use context::Ctx;
use reader::Reader;
use token::Token;
use node::{NodeType, Node};
use parser::arglist;
use utils::{dprint, dart_parseerror};
use queues::*;


pub fn expression(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: expression: {}", reader.sym()));

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

fn disjunction(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: disjunction: {}", reader.sym()));

    let left = conjunction(reader, ctx);

    if reader.len() <= reader.pos() {
        return left;
    }

    let t = reader.sym();

    return match t {
        Token::LogOr(_, _) => {

            reader.next();
            let right = disjunction(reader, ctx);

            let mut disnode = Node::new(NodeType::LogOr);
            disnode.children.push(left);
            disnode.children.push(right);

            disnode
        }

        _ => left
    }
}


fn conjunction(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: conjunction: {}", reader.sym()));

    let left = equality(reader, ctx);

    if reader.len() <= reader.pos() {
        return left;
    }

    let t = reader.sym();

    return match t {
        Token::LogAnd(_, _) => {

            reader.next();
            // let (right, i) = conjunction(tokens, next_pos + 1, ctx);
            // let (right, i) = conjunction(reader, ctx);
            let right = conjunction(reader, ctx);

            let mut connode = Node::new(NodeType::LogAnd);
            connode.children.push(left);
            connode.children.push(right);

            connode
        }

        _ => left
    }
}


// fn equality(reader: &mut Reader, ctx: &Ctx) -> (Node, usize) {
fn equality(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: equality: {}", reader.sym()));

    let left = comparison(reader, ctx);

    if reader.len() <= reader.pos() {
        return left;
    }

    let t = reader.sym();

    return match t {
        Token::Equal(_, _) => {

            reader.next();
            let right = comparison(reader, ctx);

            let mut eqnode = Node::new(NodeType::Equal);
            eqnode.children.push(left);
            eqnode.children.push(right);

            eqnode
        }

        _ => left
    }
}


// fn comparison(reader: &mut Reader, ctx: &Ctx) -> (Node, usize) {
fn comparison(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: comparison: {}", reader.sym()));

    let left = bit_or(reader, ctx);

    if reader.len() <= reader.pos() {
        return left;
    }

    let t = reader.sym();

    return match t {
        Token::LessThan(_, _) => {

            reader.next();
            let right = bit_or(reader, ctx);

            let mut connode = Node::new(NodeType::LessThan);
            connode.children.push(left);
            connode.children.push(right);

            connode
        }
        Token::GreaterThan(_, _) => {

            reader.next();
            let right = bit_or(reader, ctx);

            let mut connode = Node::new(NodeType::GreaterThan);
            connode.children.push(left);
            connode.children.push(right);

            connode
        }
        Token::LessOrEq(_, _) => {

            reader.next();
            let right = bit_or(reader, ctx);

            let mut connode = Node::new(NodeType::LessOrEq);
            connode.children.push(left);
            connode.children.push(right);

            connode
        }
        Token::GreaterOrEq(_, _) => {

            reader.next();
            let right= bit_or(reader, ctx);

            let mut connode = Node::new(NodeType::GreaterOrEq);
            connode.children.push(left);
            connode.children.push(right);

            connode
        }

        _ => left
    }
}


fn bit_or(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: bit_or: {}", reader.sym()));

    let left = bit_xor(reader, ctx);

    if reader.len() <= reader.pos() {
        return left;
    }

    let c = reader.sym();

    return match c {
        Token::BitOr(_, _) => {
            let mut node = Node::new(NodeType::BitOr);
            node.children.push(left);

            reader.next();
            let right = bit_or(reader, ctx);
            node.children.push(right);

            node
        }
        _ => left
    }
}


// fn bit_xor(reader: &mut Reader, ctx: &Ctx) -> (Node, usize) {
fn bit_xor(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: bit_xor: {}", reader.sym()));

    let left = bit_and(reader, ctx);

    if reader.len() <= reader.pos() {
        return left;
    }

    let c = reader.sym();

    return match c {
        Token::BitXor(_, _) => {
            let mut node = Node::new(NodeType::BitXor);
            node.children.push(left);

            reader.next();
            let right = bit_xor(reader, ctx);
            // let (right, i) = bit_xor(tokens, next_pos + 1, ctx);
            node.children.push(right);

            node
        }
        _ => left
    }
}


fn bit_and(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: bit_and: {}", reader.sym()));

    let left= sum(reader, ctx);

    if reader.len() <= reader.pos() {
        return left;
    }

    let c = reader.sym();

    return match c {
        Token::BitAnd(_, _) => {
            let mut node = Node::new(NodeType::BitAnd);
            node.children.push(left);

            reader.next();
            let right = bit_and(reader, ctx);
            node.children.push(right);

            node
        }
        _ => left
    }
}


fn sum(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: sum: {}", reader.sym()));

    sum_help(reader, &mut queue![], &mut queue![], ctx)
}


fn sum_help(reader: &mut Reader, righties: &mut Queue<Node>, ops: &mut Queue<Node>, ctx: &Ctx) -> Node {

    let n = product(reader, ctx);
    righties.add(n).ok();

    if reader.len() <= reader.pos() {
        return righties.remove().unwrap();
    }

    let c = reader.sym();

    return match c {

        Token::Add(_, _) => {

            ops.add(Node::new(NodeType::Add)).ok();

            reader.next();
            // let (deeper, i) = sum_help(tokens, next_pos + 1, righties, ops, ctx);
            let deeper = sum_help(reader, righties, ops, ctx);

            let mut node = ops.remove().unwrap();
            node.children.push(deeper);
            node.children.push(righties.remove().unwrap());
            // (node, i)
            node
        }
        Token::Sub(_, _) => {

            ops.add(Node::new(NodeType::Sub)).ok();

            reader.next();
            // let (deeper, i) = sum_help(tokens, next_pos + 1, righties, ops, ctx);
            let deeper = sum_help(reader, righties, ops, ctx);

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


fn product(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: product: {}", reader.sym()));

    product_help(reader, &mut queue![], &mut queue![], ctx)
}


fn product_help(reader: &mut Reader, righties: &mut Queue<Node>, ops: &mut Queue<Node>, ctx: &Ctx) -> Node {

    let n = access(reader, ctx);
    righties.add(n).ok();

    if reader.len() <= reader.pos() {
        return righties.remove().unwrap();
    }

    let c = reader.sym();

    return match c {

        Token::Mul(_, _) => {

            ops.add(Node::new(NodeType::Mul)).ok();

            reader.next();
            let deeper = product_help(reader, righties, ops, ctx);

            let mut node = ops.remove().unwrap();
            node.children.push(deeper);
            node.children.push(righties.remove().unwrap());

            node
        }
        Token::Div(_, _) => {

            ops.add(Node::new(NodeType::Div)).ok();

            // let (deeper, i) = product_help(tokens, next_pos + 1, righties, ops, ctx);
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


fn access(reader: &mut Reader, ctx: &Ctx) -> Node {

    let n = term(reader, ctx);
    let t = reader.sym();

    return match t {
        Token::Access(_, _) => {
            access_help(reader, n, ctx)
        }
        _ => { n }
    }
}


fn access_help(reader: &mut Reader, owner: Node, ctx: &Ctx) -> Node {

    match reader.sym() {

        Token::Access(_, _) => {

            match reader.next() {

                Token::Name(name, _, _) => {

                    match reader.next() {

                        Token::Paren1(_, _) => {
                            let args_node = arglist(reader, ctx);
                            let mut funcall_node = Node::new(NodeType::MethodCall(name.to_string(), Box::new(owner), ctx.filepath.clone()));
                            funcall_node.children.push(args_node);
                            return access_help(reader, funcall_node, ctx);
                        }

                        Token::Decrement(_, _) => {
                            let mut decnode = Node::new(NodeType::PostDecrement);
                            let node = Node::new(NodeType::Name(name.clone()));
                            decnode.children.push(node);
                            return decnode;
                        }

                        Token::Increment(_, _) => {
                            let mut incnode = Node::new(NodeType::PostIncrement);
                            let node = Node::new(NodeType::Name(name.clone()));
                            incnode.children.push(node);
                            return incnode;
                        }

                        _ => {
                            let mut node = Node::new(NodeType::Name(name.clone()));
                            node.children.push(owner);
                            return access_help(reader, node, ctx);
                        }
                    }
                }

                x => {
                    panic!("Expected name after accessor, got: {}", x)
                }
            }
        }
        _ => owner
    }
}


fn term(reader: &mut Reader, ctx: &Ctx) -> Node {
    dprint(format!("Parse: term: {}", reader.sym()));

    let t = reader.sym();

    match t {

        Token::Int(val, _, _) => {
            let node = Node::new(NodeType::Int(val));
            reader.next();
            return node;
        }

        Token::Double(val, _, _) => {
            let node = Node::new(NodeType::Double(val));
            reader.next();
            return node;
        }

        Token::Add(_, _) => {
            // As Dart.
            dart_parseerror(
                "'+' is not a prefix operator.",
                ctx,
                reader.tokens(),
                reader.pos()
            );
        }

        Token::Sub(_, _) => {
            // This handles unary minus.
            let mut unary = Node::new(NodeType::Sub);
            reader.next();
            let next = term(reader, ctx);
            unary.children.push(next);
            return unary;
        }

        Token::Not(_, _) => {
            let mut notnode = Node::new(NodeType::Not);
            reader.next();
            let next = term(reader, ctx);
            notnode.children.push(next);
            return notnode;
        }

        Token::Str(ref s, interpols, _, _) => {
            return if interpols.is_empty() {
                let node = Node::new(NodeType::Str(s.clone()));
                reader.next();
                node
            } else {
                let mut node = Node::new(NodeType::Str(s.clone()));

                for itp in interpols {

                    let mut r = Reader::new(itp);

                    let itpn = expression(&mut r, ctx);
                    node.children.push(itpn);
                }
                // May be empty when inside interpol recursion.
                if reader.len() > reader.pos() + 1 {
                    reader.next();
                }
                return node;
            }
        }

        Token::Bool(v, _, _) => {
            let node = Node::new(NodeType::Bool(v));
            reader.next();
            return node;
        }

        Token::Name(ref s, _, _) => {

            // Postfixed inc/dec should be bound tightly, so handle
            // it here rather than in expression.

            if reader.len() > reader.pos() + 1 {

                reader.next();

                if let Token::Increment(_, _) = reader.sym() {
                    let mut incnode = Node::new(NodeType::PostIncrement);
                    let node = Node::new(NodeType::Name(s.clone()));
                    incnode.children.push(node);
                    reader.next();
                    return incnode;
                }
                if let Token::Decrement(_, _) = reader.sym() {
                    let mut decnode = Node::new(NodeType::PostDecrement);
                    let node = Node::new(NodeType::Name(s.clone()));
                    decnode.children.push(node);
                    reader.next();
                    return decnode;
                }
                if let Token::Paren1(_, _) = reader.sym() {
                    // Function call.
                    let args_node = arglist(reader, ctx);
                    let mut funcall_node = Node::new(NodeType::FunCall(s.to_string()));
                    // FIXME, already set above
                    funcall_node.nodetype = NodeType::FunCall(s.to_string());
                    funcall_node.children.push(args_node);
                    return funcall_node;
                }
            }

            let node = Node::new(NodeType::Name(s.clone()));
            return node;
        }

        Token::Increment(_, _) => {

            return match reader.next() {
                Token::Name(name, _, _) => {
                    reader.next();
                    let mut node = Node::new(NodeType::PreIncrement);
                    node.children.push(Node::new(NodeType::Name(name)));
                    node
                }
                x => panic!("Invalid operand for increment: {}", x)
            }
        }

        Token::Decrement(_, _) => {

            return match reader.next() {
                Token::Name(name, _, _) => {
                    reader.next();
                    let mut node = Node::new(NodeType::PreDecrement);
                    node.children.push(Node::new(NodeType::Name(name)));
                    node
                }
                x => panic!("Invalid operand for decrement: {}", x)
            }
        }

        Token::Paren1(_, _) => {
            reader.next();
            let wnode = expression(reader, ctx);
            if let Token::Paren2(_, _) = reader.sym() {
                reader.next();
                return wnode;
            }
            else {
                panic!("Expected closing parenthesis at {} but found {}", reader.pos(), reader.sym())
            }
        }

        Token::Brack1(_, _) => {

            reader.next();
            let mut list_node = Node::new(NodeType::List);
            let mut expect_sep = false;

            return match reader.sym() {

                Token::Brack2(_, _) => {
                    reader.next();
                    list_node
                }

                _ => {

                    while reader.pos() < reader.len() {

                        if expect_sep {
                            match reader.sym() {

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
                                _ => panic!("Unexpected token when parsing list: {}", reader.sym())
                            }
                        }
                        expect_sep = true;
                        let entry = expression(reader, ctx);
                        list_node.children.push(entry);
                    }
                    return list_node;
                }
            }
        }


        _ => {
            panic!("Unexpected token {}.", t)
        }
    }
}