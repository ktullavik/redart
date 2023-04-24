use parser;



/// Not applicable for first char in name, where only letters are allowed
fn is_legal_namechar(c: char) -> bool {
    c.is_alphabetic() || c.is_digit(10) || (c == '_')
}

//
//fn read_keyword(tokens: &mut Vec<parser::Token>, input: &str, i: usize) -> usize {
//    let keywords: [&str; 6] = [
//        "true",
//        "false",
//        "if",
//        "else",
//        "class",
//        "get"
//    ];
//
//    for k in keywords.iter() {
//        let maybe = input.get(i..(i+k.len()));
//
//        if i + k.len() >= input.len() {
//            return 0;
//        }
//
//        let part = maybe.expect("Lexer slice out of range.");
//
//        let mut no: bool = false;
//
//
//        for c in (**k).chars() {
//            for c2 in part.chars() {
//                if c != c2 {
//                    no = true;
//                    break;
//                }
//            }
//        }
//
//        if !no {
//            if input.len() > i + k.len() {
//                let abort_check = input.get(i + k.len()..(i+k.len() + 1));
//                let abort = abort_check.expect("Lexer slice out of range 2.");
//                let next_char_maybe: Option<char> = abort.chars().next();
//                let next_char = next_char_maybe.expect("Lexer slice out of range 3.");
//                if is_legal_namechar(next_char) {
//                    return 0;
//                }
//            }
//
//            tokens.push(parser::Token::KEYWORD(String::from(*k)));
//            return (*k).len();
//        }
//    }
//    return 0;
//}


fn is_keyword(s: &str) -> bool {
    match s {
        "true" |
        "false" |
        "if" |
        "else" |
        "class" |
        "get" => true,
        _ => false
    }
}


//fn read_name(tokens: &mut Vec<parser::Token>, input: &str, start: usize) -> usize {
fn read_word(tokens: &mut Vec<parser::Token>, chars: &[char], start: usize) -> usize {
    let mut len: usize = 0;
    let mut sym = String::from("");

    while start + len < chars.len() {
        let nc: char = chars[start + len];
        if is_legal_namechar(nc) {
            sym.push(nc);
            len += 1;
            continue;
        }
        break;

//        let nc_slice = input.get((i + nl)..(i + nl + 1)).unwrap();
//        let nc = nc_slice.chars().next().expect("Lexer out of range when reading name 2.");
//        if is_legal_namechar(nc) {
//            nl += 1;
//            sym.push(nc);
//            continue;
//        }
//        break;
    }

    if is_keyword(&sym) {
        tokens.push(parser::Token::KEYWORD(sym));
    }
    else {
        tokens.push(parser::Token::NAME(sym));
    }
    return len;
}



pub fn lex(input: &str) -> Vec<parser::Token> {

    println!(" ");
    println!("LEX");
    println!(" ");

    let mut tokens: Vec<parser::Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let inp_length = chars.len();
    let mut i: usize = 0;
    let mut c: char;


    while i < inp_length {

        c = chars[i];

        match c {

            ' ' | '\n' => {}

            '"' => {
                let mut k = 1;
                let mut s = String::new();
                let mut closed = false;

                while i+k < inp_length {

                    let nc = input.get(i+k .. i+k+1).unwrap();

                    if nc == "\"" {
                        closed = true;
                        k += 1;
                        break;
                    }

                    s.push((input.get(i + k..i + k + 1)).unwrap().chars().next().unwrap());
                    k += 1;
                }
                if !closed {
                    panic!("Unclosed quote!");
                }
                tokens.push(parser::Token::STRING(s));
                i += k;
                continue;
            }


            '\'' => {
                let mut k = 1;
                let mut s = String::new();
                let mut closed = false;

                while i+k < inp_length {

                    let nc = input.get(i+k .. i+k+1).unwrap();

                    if nc == "'" {
                        closed = true;
                        k += 1;
                        break;
                    }

                    s.push((input.get(i + k..i + k + 1)).unwrap().chars().next().unwrap());
                    k += 1;
                }
                if !closed {
                    panic!("Unclosed quote!");
                }
                tokens.push(parser::Token::STRING(s));
                i += k;
                continue;
            }

            '(' => {
                tokens.push(parser::Token::PAREN1);
            }

            ')' => {
                tokens.push(parser::Token::PAREN2);
            }

            '{' => {
                tokens.push(parser::Token::BLOCK1);
            }

            '}' => {
                tokens.push(parser::Token::BLOCK2);
            }

            '[' => {
                tokens.push(parser::Token::BRACK1);
            }

            ']' => {
                tokens.push(parser::Token::BRACK2);
            }

            '.' => {
                tokens.push(parser::Token::ACCESS);
            }

            ',' => {
                tokens.push(parser::Token::COMMA);
            }

            ';' => {
                tokens.push(parser::Token::ENDST);
            }

            '=' => {
                tokens.push(parser::Token::ASSIGN);
            }

            '+' => {
                tokens.push(parser::Token::ADD);
            }

            '-' => {
                tokens.push(parser::Token::SUB);
            }

            '*' => {
                tokens.push(parser::Token::MUL);
            }

            x if x.is_digit(10) => {
                let mut nl = 1;
                let mut nc: char;
                let mut is_int: bool = true;

                while i + nl < inp_length {
                    nc = input.get(i + nl .. i + nl + 1).unwrap().chars().next().unwrap();
                    if nc.is_digit(10) {
                        nl += 1;
                        continue;
                    }
                    else if nc == '.' {
                        if input.get(i+nl-1 .. i+nl) == Some(".") {
                            panic!("Unexpected symbol: \".\"");
                        }
                        is_int = false;
                        nl += 1;
                        continue;
                    }
                    break;
                }

                let val: &str = input.get(i .. i + nl).unwrap();
                tokens.push(parser::Token::NUM(String::from(val)));
                i += nl;
                continue;
            }

            x if x.is_alphabetic() => {
//                let keylen: usize = read_keyword(&mut tokens, input, i);
//                let keylen: usize = read_keyword(&mut tokens, &chars, i);
                let word_len: usize = read_word(&mut tokens, &chars, i);
                if word_len > 0 {
                    i += word_len;
                    continue;
                }
//                i += read_name(&mut tokens, &chars, i);
//                continue;
            }

            _ => {
                panic!("Unrecognized symbol: {}", c);
            }
        }

        i += 1;
    }

    tokens.push(parser::Token::END);
    tokens
}
