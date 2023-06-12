use std::fmt;


#[derive(PartialEq)]
  pub enum Token {
  // Arithmetic
  Add,
  Sub,
  Mul,
  Div,
  Increment,
  Decrement,
  // Logic
  Not,
  LogOr,
  LogAnd,
  BitOr,
  BitXor,
  BitAnd,
  // Relation
  LessThan,
  GreaterThan,
  LessOrEq,
  GreaterOrEq,
  Equal,
  // Primitive
  Int(String),
  Double(String),
  Str(String, Vec<Vec<Token>>),
  Bool(bool),
  Name(String),
  // Structure
  Class,
  If,
  Else,
  Paren1,
  Paren2,
  Block1,
  Block2,
  Brack1,
  Brack2,
  Comma,
  // Other
  Assign,
  Access,
  Return,
  Import,
  EndSt,
  End
}


impl fmt::Display for Token {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      // Arithmetic
      Token::Add => write!(f, "+"),
      Token::Sub => write!(f, "-"),
      Token::Mul => write!(f, "*"),
      Token::Div => write!(f, "/"),
      Token::Increment => write!(f, "++"),
      Token::Decrement => write!(f, "--"),
      // Logic
      Token::Not => write!(f, "!"),
      Token::LogOr => write!(f, "||"),
      Token::LogAnd => write!(f, "&&"),
      Token::BitOr => write!(f, "|"),
      Token::BitXor => write!(f, "^"),
      Token::BitAnd => write!(f, "&"),
      // Relation
      Token::LessThan    => write!(f, "<"),
      Token::GreaterThan => write!(f, ">"),
      Token::LessOrEq    => write!(f, "<="),
      Token::GreaterOrEq => write!(f, ">="),
      Token::Equal => write!(f, "=="),
      // Primitive
      Token::Int(s)     => write!(f, "{}", s),
      Token::Double(s)     => write!(f, "{}", s),
      Token::Str(s, interpols)  => {
        if interpols.len() > 0 {
          write!(f, "\"{}\"", s).ok();
          write!(f, "( ").ok();
          for itp in interpols {
            for t in itp {
              write!(f, "{} ", t).ok();
            }
          }
          write!(f, ")")
        }
        else {
          write!(f, "\"{}\"", s)
        }
      },
      Token::Bool(v)     => write!(f, "{}", v),
      Token::Name(s)    => write!(f, "{}", s),
      // Structure
      Token::Class => write!(f, "class"),
      Token::If => write!(f, "if"),
      Token::Else => write!(f, "else"),
      Token::Paren1 => write!(f, "("),
      Token::Paren2 => write!(f, ")"),
      Token::Block1 => write!(f, "{{"),
      Token::Block2 => write!(f, "}}"),
      Token::Brack1 => write!(f, "["),
      Token::Brack2 => write!(f, "]"),
      Token::Comma => write!(f, ","),
      // Other
      Token::Assign => write!(f, "="),
      Token::Access => write!(f, "."),
      Token::Return => write!(f, "return"),
      Token::Import => write!(f, "import"),
      Token::EndSt => write!(f, "ENDST"),
      Token::End => write!(f, "END"),
    }
  }
}

