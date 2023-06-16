use token::Token;
use reader::Reader;
use utils::dprint;


/// Not applicable for first char in name, where only letters are allowed
fn is_legal_namechar(c: char) -> bool {
    c.is_alphabetic() || c.is_digit(10) || (c == '_')
}


fn read_word(tokens: &mut Vec<Token>, chars: &[char], start: usize, linenum: usize, symnum: usize) -> usize {
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

    if &sym == "import" {
        tokens.push(Token::Import(linenum, symnum));
    }
    else if &sym == "true" {
        tokens.push(Token::Bool(true, linenum, symnum));
    }
    else if &sym == "false" {
        tokens.push(Token::Bool(false, linenum, symnum));
    }
    else if &sym == "if" {
        tokens.push(Token::If(linenum, symnum));
    }
    else if &sym == "else" {
        tokens.push(Token::Else(linenum, symnum));
    }
    else if &sym == "while" {
        tokens.push(Token::While(linenum, symnum));
    }
    else if &sym == "do" {
        tokens.push(Token::Do(linenum, symnum));
    }
    else if &sym == "return" {
        tokens.push(Token::Return(linenum, symnum));
    }
    else if &sym == "class" {
        tokens.push(Token::Class(linenum, symnum));
    }
    else {
        tokens.push(Token::Name(sym, linenum, symnum));
    }
    return len;
}


pub fn lex(input: &str) -> Reader {
    let (tokens, pos) = lex_real(input, 0, 0, 1, 1);
    let reader = Reader::new(tokens);
    assert_eq!(pos, input.chars().count(), "Lexer with leftover input.");
    return reader;
}


fn lex_real(input: &str, startpos: usize, interpol: usize, mut linenum: usize, mut symnum: usize) -> (Vec<Token>, usize) {

    dprint(" ");
    dprint("LEX");
    dprint(" ");

    let mut tokens: Vec<Token> = Vec::new();

    let chars: Vec<char> = input.chars().collect();
    let inp_length = chars.len();
    let mut i: usize = startpos;
    let mut c: char;


    while i < inp_length {

        c = chars[i];

        match c {

            ' ' => {}

            '\n' => {
                assert_eq!(interpol, 0, "Unexpected newline in interpolation.");
                linenum += 1;
                symnum = 0;
            }

            '"' => {
                let mut s = String::new();
                let mut closed = false;
                let mut subs: Vec<Vec<Token>> = Vec::new();
                i += 1;
                symnum += 1;

                while i < inp_length {

                    let nc : char = chars[i];

                    if nc == '"' {
                        closed = true;
                        i += 1;
                        symnum += 1;
                        break;
                    }

                    s.push(nc);
                    i += 1;
                    symnum += 1;

                    if nc == '$' && chars[i] == '{' {
                        let (sublex, new_pos) = lex_real(input, i + 1, interpol + 1, linenum, symnum);
                        subs.push(sublex);
                        // Assuming string interpol does not cross lines.
                        symnum += new_pos - i;
                        i = new_pos;
                    }
                }
                if !closed {
                    panic!("Unclosed quote!");
                }
                tokens.push(Token::Str(s, subs, linenum, symnum));
                continue;
            }

            '\'' => {
                let mut s = String::new();
                let mut closed = false;
                let mut subs: Vec<Vec<Token>> = Vec::new();
                i += 1;
                symnum += 1;

                while i < inp_length {

                    let nc : char = chars[i];

                    if nc == '\'' {
                        closed = true;
                        i += 1;
                        symnum += 1;
                        break;
                    }

                    s.push(nc);
                    i += 1;
                    symnum += 1;

                    if nc == '$' && chars[i] == '{' {
                        let (sublex, new_pos) = lex_real(input, i + 1, interpol + 1, linenum, symnum);
                        subs.push(sublex);
                        // Assuming string interpol does not cross lines.
                        symnum += new_pos - i;
                        i = new_pos;
                    }
                }
                if !closed {
                    panic!("Unclosed quote!");
                }
                tokens.push(Token::Str(s, subs, linenum, symnum));
                continue;
            }

            '/' => {
                i += 1;
                symnum += 1;
                if inp_length > i {
                    if chars[i] == '/' {
                        i += 1;
                        symnum += 1;
                        while i < inp_length  {
                            if chars[i] == '\n' {
                                i += 1;
                                linenum += 1;
                                symnum = 0;
                                break;
                            }
                            i += 1;
                            symnum += 1;
                        }
                    }
                    else {
                        tokens.push(Token::Div(linenum, symnum));
                    }
                }
                else {
                    panic!("Unexpected end of input: '/'");
                }
                continue;
            }

            '(' => {
                tokens.push(Token::Paren1(linenum, symnum));
            }

            ')' => {
                tokens.push(Token::Paren2(linenum, symnum));
            }

            '{' => {
                tokens.push(Token::Block1(linenum, symnum));
            }

            '}' => {
                if interpol > 0 {
                    return (tokens, i+1);
                }
                tokens.push(Token::Block2(linenum, symnum));
            }

            '[' => {
                tokens.push(Token::Brack1(linenum, symnum));
            }

            ']' => {
                tokens.push(Token::Brack2(linenum, symnum));
            }

            '.' => {
                tokens.push(Token::Access(linenum, symnum));
            }

            ',' => {
                tokens.push(Token::Comma(linenum, symnum));
            }

            ';' => {
                tokens.push(Token::EndSt(linenum, symnum));
            }

            '=' => {
                if chars[i+1] == '=' {
                    tokens.push(Token::Equal(linenum, symnum));
                    i += 2;
                    symnum += 2;
                    continue;
                }
                tokens.push(Token::Assign(linenum, symnum));
            }

            '+' => {
                if chars[i+1] == '+' {
                    tokens.push(Token::Increment(linenum, symnum));
                    i += 2;
                    symnum += 2;
                    continue;
                }
                tokens.push(Token::Add(linenum, symnum));
            }

            '-' => {
                if chars[i+1] == '-' {
                    tokens.push(Token::Decrement(linenum, symnum));
                    i += 2;
                    symnum += 2;
                    continue;
                }
                tokens.push(Token::Sub(linenum, symnum));
            }

            '*' => {
                tokens.push(Token::Mul(linenum, symnum));
            }

            '<' => {
                if chars[i+1] == '=' {
                    tokens.push(Token::LessOrEq(linenum, symnum));
                    i += 2;
                    symnum += 2;
                    continue;
                }
                tokens.push(Token::LessThan(linenum, symnum));
            }

            '>' => {
                if chars[i+1] == '=' {
                    tokens.push(Token::GreaterOrEq(linenum, symnum));
                    i += 2;
                    symnum += 2;
                    continue;
                }
                tokens.push(Token::GreaterThan(linenum, symnum));
            }

            '|' => {
                if chars[i+1] == '|' {
                    tokens.push(Token::LogOr(linenum, symnum));
                    i += 2;
                    symnum += 2;
                    continue;
                }
                tokens.push(Token::BitOr(linenum, symnum));
            }

            '&' => {
                if chars[i+1] == '&' {
                    tokens.push(Token::LogAnd(linenum, symnum));
                    i += 2;
                    symnum += 2;
                    continue;
                }
                tokens.push(Token::BitAnd(linenum, symnum));
            }

            '^' => {
                tokens.push(Token::BitXor(linenum, symnum));
            }

            '!' => {
                tokens.push(Token::Not(linenum, symnum));
            }

            x if x.is_digit(10) => {
                let mut nl = 1;
                let mut nc: char;
                let mut is_int: bool = true;

                while i + nl < inp_length {
                    nc = input.get(i + nl .. i + nl + 1).unwrap().chars().next().unwrap();
                    if nc.is_digit(10) {
                        nl += 1;
                        symnum += 1;
                        continue;
                    }
                    else if nc == '.' {
                        if input.get(i+nl-1 .. i+nl) == Some(".") {
                            panic!("Unexpected symbol: \".\"");
                        }
                        is_int = false;
                        nl += 1;
                        symnum += 1;
                        continue;
                    }
                    break;
                }

                let val: &str = input.get(i .. i + nl).unwrap();
                if is_int {
                    tokens.push(Token::Int(String::from(val), linenum, symnum));
                }
                else {
                    tokens.push(Token::Double(String::from(val), linenum, symnum));
                }
                i += nl;
                continue;
            }

            x if x.is_alphabetic() => {
                let word_len: usize = read_word(&mut tokens, &chars, i, linenum, symnum);
                if word_len > 0 {
                    i += word_len;
                    symnum += word_len;
                    continue;
                }
            }

            _ => {
                panic!("Unrecognized symbol: {}", c);
            }
        }

        i += 1;
        symnum += 1;
    }

    tokens.push(Token::End);
    (tokens, i)
}
