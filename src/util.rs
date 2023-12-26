use std::io::{self, Write};

use crate::frontend::*;
use crate::*;

pub struct Flags {
    pub proj_root: String,
    pub dump: bool,
    pub cli: bool,
    pub no_interpret: bool,
}
impl Flags {
    pub fn new(flags_map: HashMap<String, bool>) -> Flags {
        Flags {
            proj_root: get_project_root(),
            dump: flags_map.contains_key("dump"),
            cli: flags_map.contains_key("cli"),
            no_interpret: flags_map.contains_key("no-interpret"),
        }
    }
    pub fn qualify_from_root(&self, path: String) {
        let mut path = path;
        if path.starts_with("/") {
            path.remove(0);
        }
        path.insert_str(0, &self.proj_root);
    }
}
pub fn get_project_root() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let src_dir = current_dir.join("src");
    let parent_dir = src_dir.parent().expect("Failed to get project root");
    let project_root = parent_dir.to_path_buf();
    project_root.to_str().unwrap().to_string()
}

pub fn run_cli() {
    let tokenizer = tokens::create_tokenizer();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let tokens = &tokenizer.tokens;

        let _ast_root = parser::parse_program(&tokens);
        input.clear();
    }
}
