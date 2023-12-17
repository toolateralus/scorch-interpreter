pub mod tokens;
pub mod ast;

use std::env;
use std::fs::File;
use std::io::Read;

use tokens::*;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    println!("Command-line arguments: {:?}", args);
    
    let mut tokenizer = tokens::create_tokenizer();
    
    let mut file = File::open("proto.type").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    tokenizer.tokenize(contents.as_str());
    
        
}

