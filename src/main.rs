// tokenizer, parser.
pub mod frontend;

// interpreter, execution. types.
pub mod runtime;

// cli repl, load & run file, etc.
pub mod util;

// unit test module, see /test.rs
#[cfg(test)]
pub mod test;

use runtime::interpreter::*;
use runtime::types::Context;

use frontend::tokens::*;
use std::{collections::HashMap, env};
use util::*;

fn main() -> () {
    let flags_map = parse_cmd_line_args();

    let flags = util::Flags::new(flags_map);

    let file = format!("{}/{}", flags.proj_root, "scorch_src/main.scorch");

    if flags.cli {
        run_cli();
    } else if flags.dump {
        execute_file_then_dump(String::from(file));
    } else {
        execute_from_file(String::from(file));
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
