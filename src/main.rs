pub mod ast;
pub mod runtime;
pub mod tokens;

use std::error;
use std::fs::File;
use std::io::Read;
use std::{collections::HashMap, env};

use ast::{Node, Visitor};
use runtime::{Context, Interpreter, ValueType};
use tokens::*;

fn main() -> () {
    test_fields_vars_literal();
    test_rel_expr(); //
    test_arithmetic();
}

fn test_fields_vars_literal() {
    let ctx = execute_return_global_ctx(String::from("test_fields_vars_literal.scorch"));
    dbg!(ctx);
}

fn test_rel_expr() {
    let ctx = execute_return_global_ctx(String::from("test_rel_expr.scorch"));
    let variables = [
        "rel_t1", "rel_t2", "rel_t3", "rel_t4", "rel_t5", "rel_t6", "rel_t7", "rel_t8", "rel_t9",
        "rel_t10", "rel_t11", "rel_t12",
    ];
    let expected_results = [
        true,  // rel_t1 := 5 < 10
        false, // rel_t2 := 5 > 10
        true,  // rel_t3 := 5 <= 10
        false, // rel_t4 := 5 >= 10
        false, // rel_t5 := 5 == 10
        true,  // rel_t6 := 5 != 10
        true,  // rel_t7 := 5 == 5
        false, // rel_t8 := 5 != 5
        true,  // rel_t9  := 5 <= 5
        true,  // rel_t10 := 5 >= 5
        false, // rel_t11 := 5 < 5
        false, // rel_t12 := 5 > 5
    ];
    for i in 0..11 {
        let variable = variables[i];
        let expected_result = expected_results[i];
        let value = *ctx.variables[*&variable].clone();
        if let ValueType::Bool(v) = value {
            if v == expected_result {
                println!("test passed: {}", variable);
            } else {
                panic!("failed test: bool value");
            }
        } else {
            dbg!(variables);
            dbg!(variable);
            panic!("failed test: bool value");
        }
    }
}

fn test_arithmetic() {
    let ctx = execute_return_global_ctx(String::from("test_arithmetic.scorch"));
    let expected_results = HashMap::from([
        ("f_arith_t02", 5.3 - 6.2),
        ("f_arith_t03", 5.3 * 6.2),
        ("f_arith_t04", 5.3 / 6.2),
        ("f_arith_t05", (5.3 + 6.2) * 2.5),
        ("f_arith_t06", 5.3 - (6.2 * 3.1)),
        ("f_arith_t07", (5.3 + 6.2) / (3.1 - 2.0)),
        ("f_arith_t08", 5.3 + (6.2 * 3.1) / 2.5),
        ("f_arith_t09", (5.3 - 6.2) * 2.5 / 3.1),
        ("f_arith_t10", 5.3 / (6.2 + 3.1) * 2.5),
        ("f_arith_t11", 5.3 + 6.2 - 3.1 * 2.0 / 1.5),
        ("f_arith_t12", ((5.3 * 2.5) - 6.2) / 3.1 + 1.0),
    ]);
    for (name, expected_val) in expected_results {
        let value = *ctx.variables[name].clone();
        if let ValueType::Float(v) = value {
            if v == expected_val {
                println!("test passed: {}", name);
            } else {
                println!(
                    "failed test: {}\nvalue: {}\nexpected: {}",
                    name, v, expected_val
                );
            }
        } else {
            println!(
                "failed test: {}\nvalue: {:?}\nexpected ValueType::Float",
                name, value
            );
        }
    }
}

fn execute_return_global_ctx(filename: String) -> Box<Context> {
    let mut tokenizer = tokens::create_tokenizer();
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    tokenizer.tokenize(&contents.as_str());

    let tokens = tokenizer.tokens;
    let ast_root = ast::parse_program(&tokens);

    let mut interpreter = Interpreter {
        context: runtime::Context::new(),
    };
    ast_root.accept(&mut interpreter);

    let ctx = interpreter.context;

    return Box::new(ctx);
}
