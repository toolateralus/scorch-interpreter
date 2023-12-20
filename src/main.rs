pub mod ast;
pub mod runtime;
pub mod tokens;
pub mod util;

#[cfg(test)]
pub mod test;

use runtime::{interpreter::*, types};
use runtime::types::Context;

use std::{collections::HashMap, env};
use tokens::*;
use util::*;

fn main() -> () {
    let flags_map = parse_cmd_line_args();
    
    let flags = util::Flags::new(flags_map);
    
    let file = format!("{}/{}", flags.proj_root, "scorch_src/main.scorch");
    
    if flags.dump {
        execute_then_dump(String::from(file));
    } else {
        execute(String::from(file));
    }
}

fn parse_cmd_line_args() -> HashMap<String, bool> {
    let mut flags = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].clone();
        flags.insert(arg, true);
        i += 1;
    }
    return flags;
}

