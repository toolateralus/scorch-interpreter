use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    println!("Command-line arguments: {:?}", args);
    
    let mut tokenizer = create_tokenizer();
    
    let mut file = File::open("proto.type").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    tokenizer.tokenize(contents.as_str());
    
    dbg!(tokenizer.tokens);
}

fn create_tokenizer() -> Tokenizer {
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
    operators.insert(String::from(":"), TokenKind::Colon);
    operators.insert(String::from("."), TokenKind::Period);
    
    let tokenizer = Tokenizer {
        operators,
        keywords,
        tokens : Vec::<Token>::new(),
        source : String::new(),
        index : 0,
        line : 0,
        column : 0,
        length : 0,
    };
    tokenizer
}
#[derive(Debug)]
pub enum TokenFamily {
    Undefined = 0,
    Value,
    Identifier, 
    Operatior,
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
    Function,
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
    tokens : Vec<Token>,
    source : String,
    index : usize,
    line : usize,
    column : usize,
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
    fn tokenize(&mut self, input : &str) -> () {
        self.length = input.len();
        self.source = String::from(input);
        while self.index < self.source.len() {
            let mut current = self.source.chars().nth(self.index).unwrap();
            if current.is_whitespace() {
                self.index += 1;
                continue;
            }
            if current.is_digit(10) {
                let mut digit : String = String::new(); 
                while self.index < self.length && current.is_digit(10) { // is decimal
                    digit.push(current);
                    if !self.try_next(&mut current) {
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
            if current.is_ascii_punctuation() {
                let mut punctuation : String = String::new();
                while self.index < self.length && current.is_ascii_punctuation() {
                    punctuation.push(current);
                    if !self.try_next(&mut current) {
                        break;
                    }  
                } 
                if self.operators.contains_key(&punctuation) {
                    let kind = self.operators.get(&punctuation);
                    let token = Token {
                        family : TokenFamily::Operatior,
                        kind : *kind.unwrap(),
                        value : punctuation,
                    };
                    self.tokens.push(token);
                    continue;
                }
            }
            if current.is_alphabetic() {
                let mut identifier : String = String::new();
                while self.index < self.length && current.is_alphabetic() {
                    identifier.push(current);
                    if !self.try_next(&mut current) {
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