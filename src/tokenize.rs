use types::{TokenType, Position, Token};

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

fn map_punctuator_tokens(c: u8) -> Option<TokenType> {
  match c {
    b'{' => Some(TokenType::LeftBrace),
    b'}' => Some(TokenType::RightBrace),
    b'[' => Some(TokenType::LeftBracket),
    b']' => Some(TokenType::RightBracket),
    b':' => Some(TokenType::Colon),
    b',' => Some(TokenType::Comma),
    _ => None,
  }
}

fn map_escapes(c: u8) -> Option<Escapes> {
  match c {
    b'"' => Some(Escapes::Quotation),
    b'\\' => Some(Escapes::ReverseSolidus),
    b'/' => Some(Escapes::Solidus),
    b'b' => Some(Escapes::Backspace),
    b'f' => Some(Escapes::FormFeed),
    b'n' => Some(Escapes::NewLine),
    b'r' => Some(Escapes::CarriageReturn),
    b't' => Some(Escapes::HorizontalTab),
    b'u' => Some(Escapes::HexadecimalDigits),
    _ => None,
  }
}

// Parsers

fn parse_whitespace(input: &str, index: usize, line: usize, column: usize) -> Option<Position> {
  let c = input.as_bytes()[index];

  match c {
    // CR (Unix)
    b'\r' => {
      if input.as_bytes()[index] == b'\n' {
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
    b'\n' => Some(Position {
      index: index + 1,
      line: line + 1,
      column: 1,
    }),
    b'\t' | b' ' => Some(Position {
      index: index + 1,
      line,
      column: column + 1,
    }),
    _ => None,
  }
}

fn parse_char(input: &str, index: usize, line: usize, column: usize) -> Option<Match> {
  let c = input.as_bytes()[index];

  match map_punctuator_tokens(c) {
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
  let rest = &input[index..];
  if rest.starts_with("true") {
    Some(Match {
      kind: TokenType::True,
      line,
      column: column + 4,
      index: index + 4,
      value: Some("true".to_string()),
    })
  } else if rest.starts_with("null") {
    Some(Match {
      kind: TokenType::Null,
      line,
      column: column + 4,
      index: index + 4,
      value: Some("null".to_string()),
    })
  } else if rest.starts_with("false") {
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
  let mut buffer = Vec::new();
  let mut state = StringStates::Start;

  while i < input.len() {
    let c = input.as_bytes()[i];
    match state {
      StringStates::Start => match c {
        b'"' => {
          i += 1;
          state = StringStates::StartQuoteOrChar;
        }
        _ => return None,
      },
      StringStates::StartQuoteOrChar => match c {
        b'\\' => {
          buffer.push(c);
          i += 1;
          state = StringStates::Escape;
        }
        b'"' => {
          i += 1;
          return Some(Match {
            kind: TokenType::String,
            line: line,
            column: column + i - index,
            index: i,
            value: Some(String::from_utf8(buffer).unwrap()),
          });
        }
        _ => {
          i += 1;
          buffer.push(c);
        }
      },
      StringStates::Escape => match map_escapes(c) {
        Some(Escapes::HexadecimalDigits) => {
          buffer.push(c);
          i += 1;
          for _ in 0..4 {
            let ch = input.as_bytes()[i];
            if ch.is_ascii_hexdigit() {
              buffer.push(ch);
              i += 1;
            } else {
              return None;
            }
          }
          state = StringStates::StartQuoteOrChar;
        }
        Some(_t) => {
          buffer.push(c);
          i += 1;
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
    let c = input.as_bytes()[i];
    match state {
      NumberStates::Start => match c {
        b'-' => state = NumberStates::Minus,
        b'0' => {
          passed_value_index = i + 1;
          state = NumberStates::Zero;
        }
        b'1'...b'9' => {
          passed_value_index = i + 1;
          state = NumberStates::Digit;
        }
        _ => return None,
      },

      NumberStates::Minus => match c {
        b'0' => {
          passed_value_index = i + 1;
          state = NumberStates::Zero;
        }
        b'1'...b'9' => {
          passed_value_index = i + 1;
          state = NumberStates::Digit;
        }
        _ => return None,
      },

      NumberStates::Zero => match c {
        b'.' => state = NumberStates::Point,
        b'e' | b'E' => state = NumberStates::Exp,
        _ => break,
      },

      NumberStates::Digit => match c {
        b'0'...b'9' => passed_value_index = i + 1,
        b'.' => state = NumberStates::Point,
        b'e' | b'E' => state = NumberStates::Exp,
        _ => break,
      },

      NumberStates::Point => match c {
        b'0'...b'9' => {
          passed_value_index = i + 1;
          state = NumberStates::DigitFraction;
        }
        _ => break,
      },

      NumberStates::DigitFraction => match c {
        b'0'...b'9' => passed_value_index = i + 1,
        b'e' | b'E' => state = NumberStates::Exp,
        _ => break,
      },

      NumberStates::Exp => match c {
        b'+' | b'-' => state = NumberStates::ExpDigitOrSign,
        b'0'...b'9' => {
          passed_value_index = i + 1;
          state = NumberStates::ExpDigitOrSign;
        }
        _ => break,
      },

      NumberStates::ExpDigitOrSign => match c {
        b'0'...b'9' => passed_value_index = i + 1,
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
