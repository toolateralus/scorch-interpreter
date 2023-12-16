use std::collections::HashMap;

pub fn create_tokenizer() -> Tokenizer {
    let mut operators : HashMap<String, TokenKind> = HashMap::new();
    let mut keywords : HashMap<String, TokenKind> = HashMap::new();

    keywords.insert(String::from("make"), TokenKind::Make);
    keywords.insert(String::from("yield"), TokenKind::Yield);
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
    operators.insert(String::from("."), TokenKind::Period);

    operators.insert(String::from("<="), TokenKind::Insert);
    operators.insert(String::from("=>"), TokenKind::Extract);

    operators.insert(String::from("+"), TokenKind::Add);
    operators.insert(String::from("-"), TokenKind::Subtract);
    operators.insert(String::from("*"), TokenKind::Multiply);
    operators.insert(String::from("/"), TokenKind::Divide);
    operators.insert(String::from("%"), TokenKind::Modulo);

    let tokenizer = Tokenizer {
        operators,
        keywords,
        tokens : Vec::<Token>::new(),
        source : String::new(),
        index : 0,
        length : 0,
    };
    tokenizer
}

#[derive(Debug)]
pub enum TokenFamily {
    Undefined = 0,
    Value,
    Identifier, 
    Operator,
    Punctuation,
    Keyword,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Undefined = 0,
    // values
    Number,
    String,
    Boolean,
    // identifiers
    Variable,
    // operators
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    // punctuation
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
    
    // keywords.
    Make,
    Yield,
    Break, 
    Typedef,
    
    // special operators
    Insert, // <=
    Extract, // =>
    DubColon, // ::
}

#[derive(Debug)]
pub struct Token {
    family : TokenFamily,
    kind : TokenKind,
    value : String,
}
pub trait TokenProcessor {
    fn tokenize(&mut self, input : &str) -> ();
    fn try_next(&mut self, current: &mut char) -> bool; 
}
pub struct Tokenizer {
    pub tokens : Vec<Token>,
    source : String,
    index : usize,
    length : usize,
    keywords : HashMap<String, TokenKind>,
    operators : HashMap<String, TokenKind>,
}
impl TokenProcessor for Tokenizer {
fn try_next(&mut self, current: &mut char) -> bool
    {
        self.index += 1;
        if self.index < self.length {
            *current = self.source.chars().nth(self.index).unwrap();
                return true;
        }
        false
    }
    fn tokenize(&mut self, input : &str) {
        self.length = input.len();
        self.source = String::from(input);
        while self.index < self.length {
            let mut current = self.source.chars().nth(self.index).unwrap();
            if current.is_whitespace() {
                self.index += 1;
                continue;
            }
            if current.is_digit(10) {
                let mut digit : String = String::new(); 
                loop {
                    digit.push(current);
                    if !self.try_next(&mut current) || !current.is_digit(10) {
                        break;
                    }
                }
                let token = Token {
                    family : TokenFamily::Value,
                    kind : TokenKind::Number,
                    value : digit,
                };
                self.tokens.push(token);
                continue;
            }            
            if current == ':' || current.is_ascii_punctuation() {
                let mut punctuation : String = String::new();
                let mut matches : Vec<String> = Vec::new();
                loop {
                    punctuation.push(current);
                    if self.operators.contains_key(&punctuation) {
                        matches.push(punctuation.clone());
                    }
                    if !self.try_next(&mut current) || !(current.is_ascii_punctuation() || current == ':') {
                        break;
                    }
                }
                if !matches.is_empty() {
                    // sort for longest matching operator.
                    matches.sort_by(|a, b| b.len().cmp(&a.len()));
                    let match_ = matches[0].clone();
                    let kind = self.operators.get(&match_);
                    let token = Token {
                        family : TokenFamily::Operator,
                        kind : *kind.unwrap(),
                        value : match_,
                    };
                    self.tokens.push(token);
                }
            }
            if current.is_alphabetic() {
                let mut identifier : String = String::new();
                loop {
                    identifier.push(current);
                    if !self.try_next(&mut current) || !current.is_alphabetic() {
                        break;
                    }
                }
                
                if self.keywords.contains_key(&identifier) {
                    let kind = self.keywords.get(&identifier);
                    let token = Token {
                        family : TokenFamily::Keyword,
                        kind : *kind.unwrap(),
                        value : identifier,
                    };
                    self.tokens.push(token);
                    continue;
                }
                
                // todo: implement const-first rule;
                // variables are explicit.
                
                let token = Token {
                    family : TokenFamily::Identifier,
                    kind : TokenKind::Variable,
                    value : identifier,  
                };
                self.tokens.push(token);
            }
        }
    }
}