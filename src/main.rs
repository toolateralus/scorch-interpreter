pub mod interpreter;
pub mod standard_functions;
pub mod typechecker;
pub mod types;
pub mod context;
pub mod cli;

use ::std::collections::HashMap;
use ::std::env;
use interpreter::*;
use scorch_parser::lexer::*;
use cli::*;

#[cfg(test)]
pub mod test;

fn main() -> () {
    let flags_map = parse_cmd_line_args();
    
    let flags = cli::Flags::new(flags_map);

    let file = format!("{}/{}", flags.proj_root, "scorch_src/main.scorch");

    if flags.cli {
        run_repl();
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