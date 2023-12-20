use regex::Regex;
use std::collections::HashMap;

pub fn create_tokenizer() -> Tokenizer {
    let operators = HashMap::from([
        (String::from("("), TokenKind::OpenParenthesis),
        (String::from(")"), TokenKind::CloseParenthesis),
        (String::from("{"), TokenKind::OpenBrace),
        (String::from("}"), TokenKind::CloseBrace),
        (String::from("["), TokenKind::OpenBracket),
        (String::from("]"), TokenKind::CloseBracket),
        (String::from(","), TokenKind::Comma),
        (String::from("::"), TokenKind::DubColon),
        (String::from(":"), TokenKind::Colon),
        (String::from(":="), TokenKind::ColonEquals),
        (String::from("="), TokenKind::Assignment),
        (String::from("=="), TokenKind::Equals),
        (String::from("!="), TokenKind::NotEquals),
        (String::from("<="), TokenKind::LessThanEquals),
        (String::from(">="), TokenKind::GreaterThanEquals),
        (String::from("<"), TokenKind::LeftAngle),
        (String::from(">"), TokenKind::RightAngle),
        (String::from("&&"), TokenKind::LogicalAnd),
        (String::from("||"), TokenKind::LogicalOr),
        (String::from("=>"), TokenKind::Lambda),
        (String::from("+"), TokenKind::Add),
        (String::from("-"), TokenKind::Subtract),
        (String::from("*"), TokenKind::Multiply),
        (String::from("/"), TokenKind::Divide),
        (String::from("%"), TokenKind::Modulo),
        (String::from("!"), TokenKind::Not),
    ]);

    let keywords = HashMap::from([
        (String::from("if"), TokenKind::If),
        (String::from("else"), TokenKind::Else),
    ]);

    let tokenizer = Tokenizer {
        operators,
        keywords,
        tokens: Vec::<Token>::new(),
        source: String::new(),
        index: 0,
        line: 1,
        column: 1,
        length: 0,
    };
    tokenizer
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenFamily {
    Undefined = 0,
    Value,
    Identifier,
    Operator,
    Keyword,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Undefined = 0,
    // values
    Number,
    String,
    Boolean,
    // identifiers
    Identifier,
    // operators
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    LessThanEquals,
    GreaterThanEquals,
    LeftAngle,
    RightAngle,
    LogicalAnd,
    LogicalOr,
    Not,

    // punctuation
    Newline,

    OpenParenthesis,
    CloseParenthesis,

    OpenBrace,
    CloseBrace,

    OpenBracket,
    CloseBracket,

    Comma,
    Colon,

    If,
    Else,
    Return,
    Eof,

    // special operators
    Lambda, // =>, Extract out.
    DubColon,
    ColonEquals,
    Assignment,
    Bool, // ::
}
#[derive(Debug, Clone)]
pub struct Token {
    pub family: TokenFamily,
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
    pub column: usize,
}
pub trait TokenProcessor {
    fn tokenize(&mut self, input: &str) -> ();
    fn consume(&mut self, current: &mut char) -> bool;
}
pub struct Tokenizer {
    pub tokens: Vec<Token>,
    source: String,
    index: usize,
    line: usize,
    column: usize,
    length: usize,
    keywords: HashMap<String, TokenKind>,
    operators: HashMap<String, TokenKind>,
}
impl TokenProcessor for Tokenizer {
    fn consume(&mut self, current: &mut char) -> bool {
        self.index += 1;
        self.column += 1;
        if self.index < self.length {
            *current = self.source.chars().nth(self.index).unwrap();
            return true;
        }
        false
    }
    fn tokenize(&mut self, original_input: &str) {
        self.tokens.clear();
        self.index = 0;
        self.line = 1;
        self.column = 1;

        let comment_regex = Regex::new(r"(//.*\n)|(/\*.*?\*/)").unwrap();
        let input = comment_regex.replace_all(original_input, "");

        self.length = input.len();
        self.source = String::from(input);
        while self.index < self.length {
            let mut current = self.source.chars().nth(self.index).unwrap();
            let mut size_at_last_newline = 0;
            if current == '\'' || current == '\"' {
                let mut string = String::new();
                loop {
                    if current == '\n' || current == '\r' {
                        self.line += 1;
                        self.column = 1;
                        size_at_last_newline = string.len();
                    }
                    if !self.consume(&mut current) {
                        panic!("Expected end of string.");
                    }
                    if current == '\'' || current == '\"' {
                        self.index += 1;
                        self.column += 1;
                        break;
                    }
                    string.push(current);
                }
                let len = string.len();
                let token = Token {
                    family: TokenFamily::Value,
                    kind: TokenKind::String,
                    value: string,
                    line: self.line,
                    column: self.column + size_at_last_newline - len,
                };
                self.tokens.push(token);
                continue;
            }
            if current == '\n' || current == '\r' {
                let token = Token {
                    family: TokenFamily::Operator,
                    kind: TokenKind::Newline,
                    value: String::from("\n"),
                    line: self.line,
                    column: self.column - 1,
                };
                self.tokens.push(token);
                self.index += 1;
                self.line += 1;
                self.column = 1;
                continue;
            }
            if current.is_whitespace() {
                self.index += 1;
                self.column += 1;
                continue;
            }
            if current.is_numeric() {
                let mut digit: String = String::new();
                digit.push(current);
                while self.consume(&mut current) {
                    if current.is_digit(10) || current == '.' {
                        digit.push(current);
                    } else {
                        break;
                    }
                }
                let len = digit.len();
                let token = Token {
                    family: TokenFamily::Value,
                    kind: TokenKind::Number,
                    value: digit,
                    line: self.line,
                    column: self.column - len,
                };
                self.tokens.push(token);
                continue;
            }
            if current.is_ascii_punctuation() && !(current == '\'' || current == '\"') {
                let mut punctuation: String = String::new();
                let mut matches: Vec<String> = Vec::new();
                while !(current == '\'' || current == '\"') {
                    punctuation.push(current);
                    if self.operators.contains_key(&punctuation) {
                        matches.push(punctuation.clone());
                    }
                    if !self.consume(&mut current)
                        || !self
                            .operators
                            .contains_key(&(punctuation.clone() + &current.to_string()))
                    {
                        break;
                    }
                }
                if !matches.is_empty() {
                    // sort for longest matching operator.
                    matches.sort_by(|a, b| b.len().cmp(&a.len()));
                    let match_ = matches[0].clone();
                    let kind = self.operators.get(&match_);
                    let len = match_.len();
                    let token = Token {
                        family: TokenFamily::Operator,
                        kind: *kind.unwrap(),
                        value: match_,
                        line: self.line,
                        column: self.column - len,
                    };
                    self.tokens.push(token);
                }
            }
            if current.is_alphabetic() || current == '_' {
                let mut identifier: String = String::new();
                loop {
                    identifier.push(current);
                    if !self.consume(&mut current) || (!current.is_alphanumeric() && current != '_')
                    {
                        break;
                    }
                }

                let len = identifier.len();
                if identifier == "true" || identifier == "false" {
                    let token = Token {
                        family: TokenFamily::Value,
                        kind: TokenKind::Bool,
                        value: identifier,
                        line: self.line,
                        column: self.column - len,
                    };
                    self.tokens.push(token);
                    continue;
                }

                if self.keywords.contains_key(&identifier) {
                    let kind = self.keywords.get(&identifier);
                    let token = Token {
                        family: TokenFamily::Keyword,
                        kind: *kind.unwrap(),
                        value: identifier,
                        line: self.line,
                        column: self.column - len,
                    };
                    self.tokens.push(token);
                    continue;
                }

                // todo: implement const-first rule;
                // variables are explicit.
                let token = Token {
                    family: TokenFamily::Identifier,
                    kind: TokenKind::Identifier,
                    value: identifier,
                    line: self.line,
                    column: self.column - len,
                };
                self.tokens.push(token);
            }
        }
        let token = Token {
            family: TokenFamily::Undefined,
            kind: TokenKind::Eof,
            value: String::from(""),
            line: self.line,
            column: self.column,
        };
        self.tokens.push(token)
    }
}
