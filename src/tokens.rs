use regex::Regex;
use std::collections::HashMap;

pub fn create_tokenizer() -> Tokenizer {
    let mut operators: HashMap<String, TokenKind> = HashMap::new();
    let mut keywords: HashMap<String, TokenKind> = HashMap::new();

    keywords.insert(String::from("for"), TokenKind::For);
    keywords.insert(String::from("loop"), TokenKind::Loop);
    
    keywords.insert(String::from("break"), TokenKind::Break);
    keywords.insert(String::from("typedef"), TokenKind::Typedef);


    operators.insert(String::from("("), TokenKind::OpenParenthesis);
    operators.insert(String::from(")"), TokenKind::CloseParenthesis);
    operators.insert(String::from("{"), TokenKind::OpenBrace);
    operators.insert(String::from("}"), TokenKind::CloseBrace);
    operators.insert(String::from("["), TokenKind::OpenBracket);
    operators.insert(String::from("]"), TokenKind::CloseBracket);
    operators.insert(String::from(","), TokenKind::Comma);
    operators.insert(String::from(";"), TokenKind::Semicolon);
    
    operators.insert(String::from("::"), TokenKind::DubColon);
    operators.insert(String::from(":"), TokenKind::Colon);
    operators.insert(String::from(":="), TokenKind::ColonEquals);
    operators.insert(String::from("."), TokenKind::Period);

    operators.insert(String::from("="), TokenKind::Assignment);

    operators.insert(String::from("<="), TokenKind::ReverseLambda);
    operators.insert(String::from("=>"), TokenKind::Lambda);

    operators.insert(String::from("+"), TokenKind::Add);
    operators.insert(String::from("-"), TokenKind::Subtract);
    operators.insert(String::from("*"), TokenKind::Multiply);
    operators.insert(String::from("/"), TokenKind::Divide);
    operators.insert(String::from("%"), TokenKind::Modulo);
    operators.insert(String::from("!"), TokenKind::Not);

    let tokenizer = Tokenizer {
        operators,
        keywords,
        tokens: Vec::<Token>::new(),
        source: String::new(),
        index: 0,
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

    // punctuation
    Newline,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Comma,
    Semicolon,
    Colon,
    Period,

    For,
    Loop,
    Break,
    Typedef,

    // special operators
    ReverseLambda, // <=, Pack In.
    Lambda,        // =>, Extract out.
    DubColon,
    ColonEquals, 
    Assignment,
    Not,
    Bool, // ::
}
#[derive(Debug, Clone)]
pub struct Token {
    pub family: TokenFamily,
    pub kind: TokenKind,
    pub value: String,
}
pub trait TokenProcessor {
    fn tokenize(&mut self, input: &str) -> ();
    fn consume(&mut self, current: &mut char) -> bool;
}
pub struct Tokenizer {
    pub tokens: Vec<Token>,
    source: String,
    index: usize,
    length: usize,
    keywords: HashMap<String, TokenKind>,
    operators: HashMap<String, TokenKind>,
}
impl TokenProcessor for Tokenizer {
    fn consume(&mut self, current: &mut char) -> bool {
        self.index += 1;
        if self.index < self.length {
            *current = self.source.chars().nth(self.index).unwrap();
            return true;
        }
        false
    }
    fn tokenize(&mut self, original_input: &str) {
        let comment_regex = Regex::new(r"(//.*\n)|(/\*.*?\*/)").unwrap();
        let input = comment_regex.replace_all(original_input, "");

        self.length = input.len();
        self.source = String::from(input);
        while self.index < self.length {
            let mut current = self.source.chars().nth(self.index).unwrap();

            if current == '\'' || current == '\"' {
                let mut string: String = String::new();
                loop {
                    if !self.consume(&mut current) {
                        panic!("Expected end of string.");
                    }

                    if current == '\'' || current == '\"' {
                        self.index += 1;
                        break;
                    }
                    string.push(current);
                }
                let token = Token {
                    family: TokenFamily::Value,
                    kind: TokenKind::String,
                    value: string,
                };
                self.tokens.push(token);
                continue;
            }
            if current == '\n' || current == '\r' {
                let token = Token {
                    family: TokenFamily::Operator,
                    kind: TokenKind::Newline,
                    value: String::from("\n"),
                };
                self.tokens.push(token);
                self.index += 1;
                continue;
            }
            if current.is_whitespace() {
                self.index += 1;
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
                let token = Token {
                    family: TokenFamily::Value,
                    kind: TokenKind::Number,
                    value: digit,
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
                        || !(current.is_ascii_punctuation() || current == ':')
                    {
                        break;
                    }
                }
                if !matches.is_empty() {
                    // sort for longest matching operator.
                    matches.sort_by(|a, b| b.len().cmp(&a.len()));
                    let match_ = matches[0].clone();
                    let kind = self.operators.get(&match_);
                    let token = Token {
                        family: TokenFamily::Operator,
                        kind: *kind.unwrap(),
                        value: match_,
                    };
                    self.tokens.push(token);
                }
            }
            if current.is_alphabetic() || current == '_' || current == '-' {
                let mut identifier: String = String::new();
                loop {
                    identifier.push(current);
                    if !self.consume(&mut current)
                        || (!current.is_alphanumeric() && current != '_' && current != '-')
                    {
                        break;
                    }
                }

                if identifier == "true" || identifier == "false"
                {
                    let token = Token {
                        family: TokenFamily::Value,
                        kind: TokenKind::Bool,
                        value: identifier,
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
                };
                self.tokens.push(token);
            }
        }
    }
}
