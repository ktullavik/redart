use token::Token;
use utils::dart_parseerror;
use state::State;

pub struct Reader {
    pos: usize,
    tokens: Vec<Token>
}


impl Reader {

    pub fn new(tokens: Vec<Token>) -> Reader {
        Reader {
            pos: 0,
            tokens
        }
    }

    pub fn expect(&self, sym: &str, state: &State) {

        let t = &self.tokens[self.pos];

        if format!("{}", t) != sym {
            dart_parseerror(
                format!("Expected: '{}'. Got: '{}'.", sym, t),
                state,
                self.sym()
            );
        }
    }

    pub fn skip(&mut self, sym: &str, state: &State) {
        self.expect(sym, state);
        self.next();
    }

    pub fn sym(&self) -> Token {
        if self.pos >= self.tokens.len() {

        }
        return self.tokens[self.pos].clone();
    }

    pub fn next(&mut self) -> Token {
        self.pos += 1;
        return self.tokens[self.pos].clone();
    }

    pub fn peek(&self) -> Token {
        return self.tokens[self.pos + 1].clone();
    }

    pub fn tokens(&self) -> &Vec<Token> {
        return &self.tokens;
    }

    pub fn pos(&self) -> usize {
        return self.pos;
    }

    pub fn len(&self) -> usize {
        return self.tokens.len();
    }

    pub fn more(&self) -> bool {
        return self.len() > self.pos + 1;
    }
 }
