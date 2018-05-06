use types::{Identifier, Node, Parsed, Property, Token, TokenType};

use std::iter::{Iterator, Peekable};

#[derive(Debug)]
enum ObjectStates {
    Start,
    OpenObject,
    Property,
    Comma,
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
                TokenType::Comma => {
                    tokens.next();
                    state = ObjectStates::Comma;
                }
                TokenType::RightBrace => {
                    tokens.next();
                    return Some(Node::Object {
                        children,
                        start,
                        end: token.end,
                    });
                }
                // invalid object, expected , or }
                _ => panic!("not implemented"),
            },
            ObjectStates::Comma => {
                let val = parse_property(tokens);
                if let Some(value) = val {
                    children.push(value);
                    state = ObjectStates::Property;
                } else {
                    // trailing commas end up here
                    panic!("not implemented")
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
            ArrayStates::Comma => {
                let val = inner_parse_value(tokens);
                children.push(val.unwrap());
                state = ArrayStates::Node;
            }
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
            tree: None,
            errors: vec![],
        },
    }
}
