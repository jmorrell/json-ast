use types::{ArrayStatus, Identifier, Node, Parsed, Property, PropertyStatus, Token, TokenType};

use std::iter::{Iterator, Peekable};

#[derive(Debug)]
enum ObjectStates {
    Start,
    OpenObject,
    Property,
    Comma,
    TrailingComma,
}

#[derive(Debug)]
enum ArrayStates {
    Start,
    OpenArray,
    Node,
    Comma,
}

#[derive(Debug)]
enum PropertyStates {
    Start,
    Key,
    Colon,
}

fn parse_property<'a, It>(tokens: &mut Peekable<It>) -> Option<Property>
where
    It: Iterator<Item = &'a Token>,
{
    let mut state = PropertyStates::Start;
    let start = tokens.peek().unwrap().start;
    let mut key = Identifier {
        raw: "".to_string(),
        start,
        end: start,
    };

    while let Some(&token) = tokens.peek() {
        match state {
            PropertyStates::Start => {
                if let TokenType::String = token.kind {
                    key = Identifier {
                        raw: token.clone().value.unwrap(),
                        start: token.start,
                        end: token.end,
                    };
                    tokens.next();
                    state = PropertyStates::Colon;
                } else {
                    return None;
                }
            }
            PropertyStates::Colon => {
                if let TokenType::Colon = token.kind {
                    tokens.next();
                    state = PropertyStates::Key;
                } else {
                    // property String was not followed by a Colon
                    panic!("not implemented yet");
                    // invalid property
                }
            }
            PropertyStates::Key => {
                if let Some(value) = inner_parse_value(tokens) {
                    let end = match value {
                        Node::Object { end, .. } => end,
                        Node::Array { end, .. } => end,
                        Node::String { end, .. } => end,
                        Node::Number { end, .. } => end,
                        Node::Boolean { end, .. } => end,
                        Node::Null { end, .. } => end,
                    };
                    return Some(Property {
                        status: PropertyStatus::Valid,
                        key,
                        value,
                        start,
                        end,
                    });
                } else {
                    return None;
                }
            }
        }
    }

    // Reached the end of tokens
    panic!("Not implemented yet");
}

fn parse_object<'a, It>(tokens: &mut Peekable<It>) -> Option<Node>
where
    It: Iterator<Item = &'a Token>,
{
    let mut state = ObjectStates::Start;
    let mut children: Vec<Property> = vec![];
    let start = tokens.peek().unwrap().start;

    while let Some(&token) = tokens.peek() {
        match state {
            ObjectStates::Start => {
                if let TokenType::LeftBrace = token.kind {
                    tokens.next();
                    state = ObjectStates::OpenObject;
                } else {
                    // We validate this before calling this object, so it would be a bug
                    return None;
                }
            }
            ObjectStates::OpenObject => {
                if let TokenType::RightBrace = token.kind {
                    tokens.next();
                    return Some(Node::Object {
                        children,
                        start,
                        end: token.end,
                    });
                } else {
                    let val = parse_property(tokens);
                    // TODO: remove unwrap
                    children.push(val.unwrap());
                    state = ObjectStates::Property;
                }
            }
            ObjectStates::Property => match token.kind {
                // Comma follows property
                TokenType::Comma => {
                    tokens.next();
                    // If the next token is a right brace, then we have a trailing comma
                    if let Some(&next_token) = tokens.peek() {
                        if let TokenType::RightBrace = next_token.kind {
                            state = ObjectStates::TrailingComma;
                        } else {
                            state = ObjectStates::Comma;
                        }
                    } else {
                        state = ObjectStates::Comma;
                    }
                }
                // Closed object
                TokenType::RightBrace => {
                    tokens.next();
                    return Some(Node::Object {
                        children,
                        start,
                        end: token.end,
                    });
                }
                // Missing comma between properties
                TokenType::String => {
                    let child = children.pop().unwrap();
                    children.push(Property {
                        status: PropertyStatus::MissingComma,
                        key: child.key,
                        value: child.value,
                        start: child.start,
                        end: child.end,
                    });
                    state = ObjectStates::Comma;
                }
                _ => panic!("not implemented"),
            },
            ObjectStates::Comma => {
                let val = parse_property(tokens);
                if let Some(value) = val {
                    children.push(value);
                    state = ObjectStates::Property;
                } else {
                    panic!("not implemented")
                }
            }
            ObjectStates::TrailingComma => {
                // We should only end up here if the next token is a Right Brace
                if let TokenType::RightBrace = token.kind {
                    // replace the last parsed child with an invalid property
                    let child = children.pop().unwrap();
                    children.push(Property {
                        status: PropertyStatus::TrailingComma,
                        key: child.key,
                        value: child.value,
                        start: child.start,
                        end: child.end,
                    });
                    tokens.next();
                    return Some(Node::Object {
                        children,
                        start,
                        end: token.end,
                    });
                } else {
                    panic!("Expected Right brace");
                }
            }
        }
    }

    // Reached the end of tokens
    panic!("Not implemented yet");
    // invalid object
}

fn parse_array<'a, It>(tokens: &mut Peekable<It>) -> Option<Node>
where
    It: Iterator<Item = &'a Token>,
{
    let mut state = ArrayStates::Start;
    let mut children: Vec<Node> = vec![];
    let start = tokens.peek().unwrap().start;

    while let Some(&token) = tokens.peek() {
        match state {
            ArrayStates::Start => {
                if let TokenType::LeftBracket = token.kind {
                    tokens.next();
                    state = ArrayStates::OpenArray;
                } else {
                    // We validate this before calling this object, so it would be a bug
                    return None;
                }
            }
            ArrayStates::OpenArray => {
                if let TokenType::RightBracket = token.kind {
                    tokens.next();
                    return Some(Node::Array {
                        status: ArrayStatus::Valid,
                        children,
                        start,
                        end: token.end,
                    });
                } else {
                    let val = inner_parse_value(tokens);
                    // TODO: remove unwrap
                    children.push(val.unwrap());
                    state = ArrayStates::Node;
                }
            }
            ArrayStates::Node => match token.kind {
                TokenType::RightBracket => {
                    tokens.next();
                    return Some(Node::Array {
                        status: ArrayStatus::Valid,
                        children,
                        start,
                        end: token.end,
                    });
                }
                TokenType::Comma => {
                    tokens.next();
                    state = ArrayStates::Comma;
                }
                _ => {
                    panic!("Not implemented yet");
                }
            },
            ArrayStates::Comma => match token.kind {
                TokenType::RightBracket => {
                    tokens.next();
                    return Some(Node::Array {
                        status: ArrayStatus::TrailingComma,
                        children,
                        start,
                        end: token.end,
                    });
                }
                _ => {
                    let val = inner_parse_value(tokens);
                    children.push(val.unwrap());
                    state = ArrayStates::Node;
                }
            },
        }
    }

    // Reached the end of tokens
    panic!("Not implemented yet");
}

fn parse_literal<'a, It>(tokens: &mut Peekable<It>) -> Option<Node>
where
    It: Iterator<Item = &'a Token>,
{
    match tokens.next() {
        Some(token) => match token.kind {
            TokenType::String => Some(Node::String {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            }),
            TokenType::Number => Some(Node::Number {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            }),
            TokenType::True | TokenType::False => Some(Node::Boolean {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            }),
            TokenType::Null => Some(Node::Null {
                raw: token.clone().value.unwrap(),
                start: token.start,
                end: token.end,
            }),
            TokenType::LeftBrace
            | TokenType::RightBrace
            | TokenType::LeftBracket
            | TokenType::RightBracket
            | TokenType::Colon
            | TokenType::Comma => None,
        },
        None => {
            panic!("expected a literal");
        }
    }
}

fn inner_parse_value<'a, It>(tokens: &mut Peekable<It>) -> Option<Node>
where
    It: Iterator<Item = &'a Token>,
{
    match tokens.peek() {
        Some(&t) => match t.kind {
            TokenType::LeftBrace => parse_object(tokens),
            TokenType::LeftBracket => parse_array(tokens),
            TokenType::String
            | TokenType::Number
            | TokenType::True
            | TokenType::False
            | TokenType::Null => parse_literal(tokens),
            TokenType::RightBrace
            | TokenType::RightBracket
            | TokenType::Colon
            | TokenType::Comma => None,
        },
        None => None,
    }
}

pub fn parse_value(tokens: &Vec<Token>) -> Parsed {
    let mut iter = tokens.iter().peekable();
    let val = inner_parse_value(&mut iter);

    match val {
        Some(v) => Parsed::Success { tree: v },
        None => Parsed::Failure {
            tokens: tokens.to_vec(),
            tree: None,
            errors: vec![],
        },
    }
}
