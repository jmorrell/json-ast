#[derive(Copy, Clone, Debug)]
pub enum TokenType {
  LeftBrace,    // {
  RightBrace,   // }
  LeftBracket,  // [
  RightBracket, // ]
  Colon,        // :
  Comma,        // ,
  String,       //
  Number,       //
  True,         // true
  False,        // false
  Null,         // null
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
  pub line: usize,
  pub column: usize,
  pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Token {
  pub kind: TokenType,
  pub value: Option<String>,
  pub start: Position,
  pub end: Position,
}

enum Escapes {
  Quotation,
  ReverseSolidus,
  Solidus,
  Backspace,
  FormFeed,
  NewLine,
  CarriageReturn,
  HorizontalTab,
  HexadecimalDigits,
}

enum StringStates {
  Start,
  StartQuoteOrChar,
  Escape,
}

enum NumberStates {
  Start,
  Minus,
  Zero,
  Digit,
  Point,
  DigitFraction,
  Exp,
  ExpDigitOrSign,
}

#[derive(Clone, Debug)]
struct Match {
  kind: TokenType,
  line: usize,
  column: usize,
  index: usize,
  value: Option<String>,
}

fn map_punctuator_tokens(c: char) -> Option<TokenType> {
  match c {
    '{' => Some(TokenType::LeftBrace),
    '}' => Some(TokenType::RightBrace),
    '[' => Some(TokenType::LeftBracket),
    ']' => Some(TokenType::RightBracket),
    ':' => Some(TokenType::Colon),
    ',' => Some(TokenType::Comma),
    _ => None,
  }
}

fn map_escapes(c: char) -> Option<Escapes> {
  match c {
    '"' => Some(Escapes::Quotation),
    '\\' => Some(Escapes::ReverseSolidus),
    '/' => Some(Escapes::Solidus),
    'b' => Some(Escapes::Backspace),
    'f' => Some(Escapes::FormFeed),
    'n' => Some(Escapes::NewLine),
    'r' => Some(Escapes::CarriageReturn),
    't' => Some(Escapes::HorizontalTab),
    'u' => Some(Escapes::HexadecimalDigits),
    _ => None,
  }
}

// Parsers

fn parse_whitespace(input: &str, index: usize, line: usize, column: usize) -> Option<Position> {
  let c = input.chars().nth(index);

  match c {
    // CR (Unix)
    Some('\r') => {
      if input.chars().nth(index) == Some('\n') {
        Some(Position {
          index: index + 2,
          line: line + 1,
          column: column + 1,
        })
      } else {
        Some(Position {
          index: index + 1,
          line: line + 1,
          column: column + 1,
        })
      }
    }
    // LF (MacOS)
    Some('\n') => Some(Position {
      index: index + 1,
      line: line + 1,
      column: 1,
    }),
    Some('\t') | Some(' ') => Some(Position {
      index: index + 1,
      line,
      column: column + 1,
    }),
    _ => None,
  }
}

fn parse_char(input: &str, index: usize, line: usize, column: usize) -> Option<Match> {
  let c = input.chars().nth(index);

  match map_punctuator_tokens(c.unwrap()) {
    Some(t) => Some(Match {
      kind: t,
      line,
      column: column + 1,
      index: index + 1,
      value: None,
    }),
    None => None,
  }
}

fn parse_keyword(input: &str, index: usize, line: usize, column: usize) -> Option<Match> {
  let len = input.len();
  if len >= index + 4 && &input[index..index + 4] == "true" {
    Some(Match {
      kind: TokenType::True,
      line,
      column: column + 4,
      index: index + 4,
      value: Some("true".to_string()),
    })
  } else if len >= index + 4 && &input[index..index + 4] == "null" {
    Some(Match {
      kind: TokenType::Null,
      line,
      column: column + 4,
      index: index + 4,
      value: Some("null".to_string()),
    })
  } else if len >= index + 5 && &input[index..index + 5] == "false" {
    Some(Match {
      kind: TokenType::False,
      line,
      column: column + 5,
      index: index + 5,
      value: Some("false".to_string()),
    })
  } else {
    None
  }
}

fn parse_string(input: &str, index: usize, line: usize, column: usize) -> Option<Match> {
  let mut i = index;
  // let mut passed_value_index = index;
  let mut buffer = String::new();
  let mut state = StringStates::Start;

  while i < input.len() {
    let c = input.chars().nth(i).unwrap();
    match state {
      StringStates::Start => match c {
        '"' => {
          i += 1;
          state = StringStates::StartQuoteOrChar;
        }
        _ => return None,
      },
      StringStates::StartQuoteOrChar => match c {
        '\\' => {
          buffer.push_str(&c.to_string());
          i += 1;
          // passed_value_index = i;
          state = StringStates::Escape;
        }
        '"' => {
          i += 1;
          // passed_value_index = i;
          return Some(Match {
            kind: TokenType::String,
            line: line,
            column: column + i - index,
            index: i,
            // TODO: isn't passed_value_index just always i here?
            value: Some(input[index..i].to_string()),
          });
        }
        _ => {
          i += 1;
          // passed_value_index = i;
          buffer.push_str(&c.to_string());
        }
      },
      StringStates::Escape => match map_escapes(c) {
        Some(Escapes::HexadecimalDigits) => {
          buffer.push_str(&c.to_string());
          i += 1;
          // passed_value_index = i;
          for _ in 0..4 {
            if let Some(ch) = input.chars().nth(i) {
              if ch.is_ascii_hexdigit() {
                buffer.push_str(&ch.to_string());
                i += 1;
              } else {
                return None;
              }
            } else {
              return None;
            }
          }
          state = StringStates::StartQuoteOrChar;
        }
        Some(_t) => {
          buffer.push_str(&c.to_string());
          i += 1;
          // passed_value_index = i;
          state = StringStates::StartQuoteOrChar;
        }
        _ => {
          return None;
        }
      },
    }
  }
  None
}

fn parse_number(input: &str, index: usize, line: usize, column: usize) -> Option<Match> {
  let start_index = index;
  let mut passed_value_index = index;
  let mut i = index;
  let mut state = NumberStates::Start;

  while i < input.len() {
    let c = input.chars().nth(i).unwrap();
    match state {
      NumberStates::Start => match c {
        '-' => state = NumberStates::Minus,
        '0' => {
          passed_value_index = i + 1;
          state = NumberStates::Zero;
        }
        '1'...'9' => {
          passed_value_index = i + 1;
          state = NumberStates::Digit;
        }
        _ => return None,
      },

      NumberStates::Minus => match c {
        '0' => {
          passed_value_index = i + 1;
          state = NumberStates::Zero;
        }
        '1'...'9' => {
          passed_value_index = i + 1;
          state = NumberStates::Digit;
        }
        _ => return None,
      },

      NumberStates::Zero => match c {
        '.' => state = NumberStates::Point,
        'e' | 'E' => state = NumberStates::Exp,
        _ => break,
      },

      NumberStates::Digit => match c {
        '0'...'9' => passed_value_index = i + 1,
        '.' => state = NumberStates::Point,
        'e' | 'E' => state = NumberStates::Exp,
        _ => break,
      },

      NumberStates::Point => match c {
        '0'...'9' => {
          passed_value_index = i + 1;
          state = NumberStates::DigitFraction;
        }
        _ => break,
      },

      NumberStates::DigitFraction => match c {
        '0'...'9' => passed_value_index = i + 1,
        'e' | 'E' => state = NumberStates::Exp,
        _ => break,
      },

      NumberStates::Exp => match c {
        '+' | '-' => state = NumberStates::ExpDigitOrSign,
        '0'...'9' => {
          passed_value_index = i + 1;
          state = NumberStates::ExpDigitOrSign;
        }
        _ => break,
      },

      NumberStates::ExpDigitOrSign => match c {
        '0'...'9' => passed_value_index = i + 1,
        _ => break,
      },
    }
    i += 1;
  }

  if passed_value_index > 0 {
    Some(Match {
      kind: TokenType::Number,
      line: line,
      column: column + passed_value_index - start_index,
      index: passed_value_index,
      value: Some(input[start_index..passed_value_index].to_string()),
    })
  } else {
    None
  }
}

pub fn tokenize(input: &str) -> Vec<Token> {
  let mut line: usize = 1;
  let mut column: usize = 1;
  let mut index: usize = 0;
  let mut tokens: Vec<Token> = Vec::new();

  while index < input.len() {
    if let Some(pos) = parse_whitespace(input, index, line, column) {
      line = pos.line;
      column = pos.column;
      index = pos.index;
    } else if let Some(pos) = parse_char(input, index, line, column) {
      tokens.push(Token {
        kind: pos.kind,
        value: pos.value,
        start: Position {
          line,
          column,
          index,
        },
        end: Position {
          line: pos.line,
          column: pos.column,
          index: pos.index,
        },
      });
      line = pos.line;
      column = pos.column;
      index = pos.index;
    } else if let Some(pos) = parse_keyword(input, index, line, column) {
      tokens.push(Token {
        kind: pos.kind,
        value: pos.value,
        start: Position {
          line,
          column,
          index,
        },
        end: Position {
          line: pos.line,
          column: pos.column,
          index: pos.index,
        },
      });
      line = pos.line;
      column = pos.column;
      index = pos.index;
    } else if let Some(pos) = parse_string(input, index, line, column) {
      tokens.push(Token {
        kind: pos.kind,
        value: pos.value,
        start: Position {
          line,
          column,
          index,
        },
        end: Position {
          line: pos.line,
          column: pos.column,
          index: pos.index,
        },
      });
      line = pos.line;
      column = pos.column;
      index = pos.index;
    } else if let Some(pos) = parse_number(input, index, line, column) {
      tokens.push(Token {
        kind: pos.kind,
        value: pos.value,
        start: Position {
          line,
          column,
          index,
        },
        end: Position {
          line: pos.line,
          column: pos.column,
          index: pos.index,
        },
      });
      line = pos.line;
      column = pos.column;
      index = pos.index;
    } else {
      panic!("TODO: Errors end up here");
    }
  }

  tokens
}
