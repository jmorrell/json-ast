use types::{TokenType, Token, Node, Property, Identifier, Parsed};

enum ObjectStates {
    Start,
    OpenObject,
    Property,
    Comma,
}

enum ArrayStates {
    Start,
    OpenArray,
    Node,
    Comma,
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
            let value = inner_parse_value(_input, tokens, index).unwrap();
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

fn parse_object(_input: &str, tokens: &Vec<Token>, index: &mut usize) -> Option<Node> {
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
                    return Some(Node::Object {
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
                    return Some(Node::Object {
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

fn parse_array(_input: &str, tokens: &Vec<Token>, index: &mut usize) -> Option<Node> {
    let mut state = ArrayStates::Start;
    let mut children: Vec<Node> = vec![];
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
                    return Some(Node::Array {
                        children,
                        start: start_token.start,
                        end: token.end,
                    });
                } else {
                    let val = inner_parse_value(_input, tokens, index);
                    children.push(val.unwrap());
                    state = ArrayStates::Node;
                }
            }
            ArrayStates::Node => match token.kind {
                TokenType::RightBracket => {
                    let start_token = &tokens[start_index];
                    *index += 1;
                    return Some(Node::Array {
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
                let val = inner_parse_value(_input, tokens, index);
                children.push(val.unwrap());
                state = ArrayStates::Node;
            }
        }
    }

    None
}

fn parse_literal(_input: &str, tokens: &Vec<Token>, index: &mut usize) -> Option<Node> {
    let token = &tokens[*index];

    match token.kind {
        TokenType::String => {
            *index += 1;
            Some(Node::String {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            })
        }
        TokenType::Number => {
            *index += 1;
            Some(Node::Number {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            })
        }
        TokenType::True | TokenType::False => {
            *index += 1;
            Some(Node::Boolean {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            })
        }
        TokenType::Null => {
            *index += 1;
            Some(Node::Null {
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

fn inner_parse_value(input: &str, tokens: &Vec<Token>, index: &mut usize) -> Option<Node> {
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

pub fn parse_value(input: &str, tokens: &Vec<Token>, index: &mut usize) -> Parsed {
    if let Some(val) = parse_literal(input, tokens, index) {
        Parsed::Success { tree: val }
    } else if let Some(val) = parse_array(input, tokens, index) {
        Parsed::Success { tree: val }
    } else if let Some(val) = parse_object(input, tokens, index) {
        Parsed::Success { tree: val }
    } else {
        Parsed::Failure {
            tree: None,
            errors: vec!(),
        }
    }
}
