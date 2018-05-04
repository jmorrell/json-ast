use tokenize::{Position, Token, TokenType};

enum ObjectStates {
    Start,
    OpenObject,
    Property,
    Comma,
}

enum ArrayStates {
    Start,
    OpenArray,
    Value,
    Comma,
}

#[derive(Clone, Debug)]
pub enum Value {
    Object {
        children: Vec<Property>,
        start: Position,
        end: Position,
    },
    Array {
        children: Vec<Value>,
        start: Position,
        end: Position,
    },
    String {
        raw: String,
        start: Position,
        end: Position,
    },
    Number {
        raw: String,
        start: Position,
        end: Position,
    },
    Boolean {
        raw: String,
        start: Position,
        end: Position,
    },
    Null {
        raw: String,
        start: Position,
        end: Position,
    },
}

#[derive(Clone, Debug)]
pub struct Property {
    pub key: Identifier,
    pub value: Value,
    pub start: Position,
    pub end: Position,
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub raw: String,
    pub start: Position,
    pub end: Position,
}

fn parse_property(_input: &str, tokens: &Vec<Token>, index: &mut usize) -> Option<Property> {
    let token = &tokens[*index];

    // The different states of parsing for properties are sequential until the last
    // so there's no need to break this into a FSM
    if let TokenType::String = token.kind {
        if let TokenType::Colon = (&tokens[*index + 1]).kind {
            let key = Identifier {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            };

            *index += 2;
            let value = parse_value(_input, tokens, index).unwrap();
            let last_token = &tokens[*index-1];

            return Some(Property {
                key,
                value,
                start: token.start,
                end: last_token.end,
            });
        } else {
            // property String was not followed by a Colon
            panic!("not implemented yet");
        }
    } else {
        None
    }
}

fn parse_object(_input: &str, tokens: &Vec<Token>, index: &mut usize) -> Option<Value> {
    let mut state = ObjectStates::Start;
    let mut children: Vec<Property> = vec![];
    let start_index = *index;

    while *index < tokens.len() {
        let token = &tokens[*index];

        match state {
            ObjectStates::Start => {
                if let TokenType::LeftBrace = token.kind {
                    state = ObjectStates::OpenObject;
                    *index += 1;
                } else {
                    return None;
                }
            }
            ObjectStates::OpenObject => {
                if let TokenType::RightBrace = token.kind {
                    let start_token = &tokens[start_index];
                    *index += 1;
                    return Some(Value::Object {
                        children,
                        start: start_token.start,
                        end: token.end,
                    });
                } else {
                    let val = parse_property(_input, tokens, index);
                    children.push(val.unwrap());
                    state = ObjectStates::Property;
                }
            }
            ObjectStates::Property => match token.kind {
                TokenType::Comma => {
                    *index += 1;
                    state = ObjectStates::Comma;
                }
                TokenType::RightBrace => {
                    let start_token = &tokens[start_index];
                    *index += 1;
                    return Some(Value::Object {
                        children,
                        start: start_token.start,
                        end: token.end,
                    });
                }
                _ => panic!("not implemented"),
            },
            ObjectStates::Comma => {
                let val = parse_property(_input, tokens, index);
                if let Some(value) = val {
                    children.push(value);
                    state = ObjectStates::Property;
                } else {
                    panic!("not implemented")
                }
            }
        }
    }

    None
}

fn parse_array(_input: &str, tokens: &Vec<Token>, index: &mut usize) -> Option<Value> {
    let mut state = ArrayStates::Start;
    let mut children: Vec<Value> = vec![];
    let start_index = *index;

    while *index < tokens.len() {
        let token = &tokens[*index];

        match state {
            ArrayStates::Start => {
                if let TokenType::LeftBracket = token.kind {
                    state = ArrayStates::OpenArray;
                    *index += 1;
                } else {
                    return None;
                }
            }
            ArrayStates::OpenArray => {
                if let TokenType::RightBracket = token.kind {
                    let start_token = &tokens[start_index];
                    *index += 1;
                    return Some(Value::Array {
                        children,
                        start: start_token.start,
                        end: token.end,
                    });
                } else {
                    let val = parse_value(_input, tokens, index);
                    children.push(val.unwrap());
                    state = ArrayStates::Value;
                }
            }
            ArrayStates::Value => match token.kind {
                TokenType::RightBracket => {
                    let start_token = &tokens[start_index];
                    *index += 1;
                    return Some(Value::Array {
                        children,
                        start: start_token.start,
                        end: token.end,
                    });
                }
                TokenType::Comma => {
                    state = ArrayStates::Comma;
                    *index += 1;
                }
                _ => {
                    panic!("Not implemented yet");
                }
            },
            ArrayStates::Comma => {
                let val = parse_value(_input, tokens, index);
                children.push(val.unwrap());
                state = ArrayStates::Value;
            }
        }
    }

    None
}

fn parse_literal(_input: &str, tokens: &Vec<Token>, index: &mut usize) -> Option<Value> {
    let token = &tokens[*index];

    match token.kind {
        TokenType::String => {
            *index += 1;
            Some(Value::String {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            })
        }
        TokenType::Number => {
            *index += 1;
            Some(Value::Number {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            })
        }
        TokenType::True | TokenType::False => {
            *index += 1;
            Some(Value::Boolean {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            })
        }
        TokenType::Null => {
            *index += 1;
            Some(Value::Null {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            })
        }
        TokenType::LeftBrace
        | TokenType::RightBrace
        | TokenType::LeftBracket
        | TokenType::RightBracket
        | TokenType::Colon
        | TokenType::Comma => None,
    }
}

pub fn parse_value(input: &str, tokens: &Vec<Token>, index: &mut usize) -> Option<Value> {
    if let Some(val) = parse_literal(input, tokens, index) {
        Some(val)
    } else if let Some(val) = parse_array(input, tokens, index) {
        Some(val)
    } else if let Some(val) = parse_object(input, tokens, index) {
        Some(val)
    } else {
        None
    }
}

// fn parse(input: &str) {
//     let tokens = tokenize(input);
//     let mut index: usize = 0;

//     // println!("Tokens: {:?}", tokens);

//     if tokens.len() == 0 {
//         println!("this is an error case");
//     }

//     let value = parse_value(input, &tokens, &mut index);
//     println!("JSON:\n{}\nParsed:\n{:#?}", input, value.unwrap());
// }
