pub mod expression;
pub mod interpreter;
pub mod std_builtins;
pub mod typechecker;
pub mod types;
pub mod context;


// cli repl, load & run file, etc.
pub mod util;

// unit test module, see /test.rs
#[cfg(test)]
pub mod test;
use ::std::collections::HashMap;
use ::std::env;

use interpreter::*;

use scorch_parser::lexer::*;

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
