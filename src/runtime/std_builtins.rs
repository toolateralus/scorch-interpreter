use std::collections::HashMap;

use super::types::{Value, BuiltInFunction};


pub fn print_ln(args: Vec<Value>) -> Value {
    for arg in args {
        match arg {
            Value::Float(val) => print!("{}\n", val),
            Value::Bool(val) => print!("{}\n", val),
            Value::String(val) => print!("{}\n", val),
            Value::None() => print!("{:?}", Value::None()),
            Value::Function(_) => {
                let newargs = Vec::from([arg.clone()]);
                return tostr(newargs);
            }
            Value::Array(mutable, elements) => {
                let mutable_str = if mutable { "mutable" } else { "immutable" };

                println!("{} array, length {}", mutable_str, elements.len());

                for element in elements.iter() {
                    print_ln(Vec::from([element.value.clone()]));
                }
            }
            Value::List(elements) => {
                for element in elements.try_borrow().unwrap().iter() {
                    print_ln(Vec::from([element.value.clone()]));
                }
            }
            Value::Struct {
                name: _,
                context: _,
            } => todo!(),
            Value::Return(_) => panic!("Cannot print return value"),
        }
    }
    Value::None()
}
pub fn wait(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("sleep expected 1 argument :: ms sleep duration");
    }
    if let Value::Float(val) = args[0] {
        std::thread::sleep(std::time::Duration::from_millis(val as u64));
    } else {
        panic!("sleep expected a <num>");
    }
    Value::None()
}
pub fn readln(args: Vec<Value>) -> Value {
    if args.len() != 0 {
        panic!("readln expected 0 arguments");
    }
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    Value::String(input.replace("\n", ""))
}
pub fn tostr(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("tostr expected 1 argument");
    }
    let arg = &args[0];
    let result = match arg {
        Value::Float(val) => val.to_string(),
        Value::Bool(val) => val.to_string(),
        Value::String(val) => val.clone(),
        Value::None() => String::from("None"),
        Value::Function(func) => {
            let params: Vec<String> = func
                .params
                .iter()
                .map(|param| format!("{}: {}", param.name, param.typename))
                .collect();
            let stri = String::from(format!(
                "{}({}) -> {}",
                func.name,
                params.join(", "),
                func.return_type
            ));
            println!("{}", stri);
            stri
        }
        Value::Array(mutable, elements) => {
            let mutable_str = if *mutable { "mutable" } else { "immutable" };
            format!("array : {} , length : {}", mutable_str, elements.len())
        }
        _ => {
            panic!("Cannot convert value to string");
        }
    };
    Value::String(result)
}

pub fn get_builtin_functions() -> HashMap<String, BuiltInFunction> {
    HashMap::from([
        (
            String::from("println"),
            BuiltInFunction::new(Box::new(print_ln)),
        ),
        (
            String::from("readln"),
            BuiltInFunction::new(Box::new(readln)),
        ),
        (   
            String::from("wait"), 
            BuiltInFunction::new(Box::new(wait))
        ),
        (   
            String::from("tostr"), 
            BuiltInFunction::new(Box::new(tostr))
        ),
    ])
}
