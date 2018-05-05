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

#[derive(Clone, Debug)]
pub enum Node {
    Object {
        children: Vec<Property>,
        start: Position,
        end: Position,
    },
    Array {
        children: Vec<Node>,
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
    pub value: Node,
    pub start: Position,
    pub end: Position,
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub raw: String,
    pub start: Position,
    pub end: Position,
}

#[derive(Clone, Debug)]
pub enum ParseErrorType {
    TrailingComma,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct ParseError {
    pub position: Position,
    pub error_type: ParseErrorType,
}

#[derive(Clone, Debug)]
pub enum Parsed {
    Success {
        tree: Node,
    },
    Failure {
        tree: Option<Node>,
        errors: Vec<ParseError>,
    },
}