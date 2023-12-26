// tokenizer, parser.
pub mod frontend;
pub mod llvm;
pub mod util;

use ::std::collections::HashMap;
use ::std::env;

use util::*;

fn main() -> () {
    let flags_map = parse_cmd_line_args();

    let flags = util::Flags::new(flags_map);

    let _file = format!("{}/{}", flags.proj_root, "scorch_src/main.scorch");

    if flags.cli {
        run_cli();
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
