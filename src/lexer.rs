use parser;
use utils;


/// Not applicable for first char in name, where only letters are allowed
fn is_legal_namechar(c: char) -> bool {
    c.is_alphabetic() || c.is_digit(10) || (c == '_')
}


fn is_keyword(s: &str) -> bool {
    match s {
        // "true" |
        // "false" |
        "if" |
        "else" |
        "class" |
        "get" => true,
        _ => false
    }
}


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
    }

    if &sym == "true" {
        tokens.push(parser::Token::BOOL(true));
    }
    else if &sym == "false" {
        tokens.push(parser::Token::BOOL(false));
    }
    else if is_keyword(&sym) {
        tokens.push(parser::Token::KEYWORD(sym));
    }
    else {
        tokens.push(parser::Token::NAME(sym));
    }
    return len;
}



pub fn lex(input: &str) -> Vec<parser::Token> {

    utils::dprint(" ");
    utils::dprint("LEX");
    utils::dprint(" ");

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

            '/' => {
                i = i + 1;
                if inp_length > i {
                    if chars[i] == '/' {
                        i += 1;
                        while i < inp_length  {
                            if chars[i] == '\n' {
                                i += 1;
                                break;
                            }
                            i += 1;
                        }
                    }
                    else {
                        tokens.push(parser::Token::DIV);
                    }
                }
                else {
                    panic!("Unexpected end of input: '/'");
                }
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
                if is_int {
                    tokens.push(parser::Token::INT(String::from(val)));
                }
                else {
                    tokens.push(parser::Token::DOUBLE(String::from(val)));
                }
                i += nl;
                continue;
            }

            x if x.is_alphabetic() => {
                let word_len: usize = read_word(&mut tokens, &chars, i);
                if word_len > 0 {
                    i += word_len;
                    continue;
                }
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
