use std::{collections::HashMap, rc::Rc};

use super::{
    typechecker::TypeChecker,
    types::{BuiltInFunction, Context, Value, Variable},
};

pub fn print_ln(context: &mut Context, type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    for arg in args {
        match arg {
            Value::Int(val) => print!("{}\n", val),
            Value::Double(val) => print!("{}\n", val),
            Value::Bool(val) => print!("{}\n", val),
            Value::String(val) => print!("{}\n", val),
            Value::None() => print!("{:?}", Value::None()),
            Value::Function(_) => {
                let newargs = Vec::from([arg]);
                return tostr(context, type_checker, newargs);
            }
            Value::Array(mutable, elements) => {
                let mutable_str = if mutable { "var" } else { "const" };

                println!("{} Array<T> : length {}", mutable_str, elements.len());

                // for element in elements.iter() {
                //     print_ln(Vec::from([element.value.clone()]));
                // }
            }
            Value::Struct {
                typename: name,
                context: _,
            } => {
                println!("global::{}", name);
                // for (k, member) in context.variables.iter() {
                //     println!("{} : {:?}", k, member.value)
                // }
            }
            Value::Return(_) => panic!("Cannot print return value"),
        }
    }
    Value::None()
}
pub fn wait(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("wait expected 1 argument :: ms wait duration");
    }
    if let Value::Double(val) = args[0] {
        std::thread::sleep(std::time::Duration::from_millis(val as u64));
    } else {
        panic!("wait expected a <num>");
    }
    Value::None()
}
pub fn length(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("length takes one array argument");
    }
    let arg = &args[0];

    match arg {
        Value::Array(_, elements) => {
            return Value::Int(elements.len() as i32);
        }
        _ => {
            dbg!(arg);
            panic!("Cannot get length of value");
        }
    }
}
pub fn time(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 0 {
        panic!("time expected 0 arguments");
    }

    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    Value::Double(time.as_secs_f64())
}
pub fn assert_eq(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 3 {
        panic!("assert expected 3 arguments");
    }
    let message = args[2].as_string().unwrap();

    let are_equal = match (&args[0], &args[1]) {
        (Value::None(), Value::None()) => true,
        (Value::Int(lhs_val), Value::Int(rhs_val)) => lhs_val == rhs_val,
        (Value::Double(lhs_val), Value::Double(rhs_val)) => lhs_val == rhs_val,
        (Value::Bool(lhs_val), Value::Bool(rhs_val)) => lhs_val == rhs_val,
        (Value::String(lhs_val), Value::String(rhs_val)) => lhs_val == rhs_val,
        _ => false,
    };
    assert!(are_equal, "{}", message);
    Value::None()
}
pub fn assert(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 2 {
        panic!("assert expected 2 or 3 arguments");
    }
    let condition = args[0].as_bool().unwrap();
    let message = args[1].as_string().unwrap();
    assert!(condition, "{}", message);
    Value::None()
}
pub fn readln(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 0 {
        panic!("readln expected 0 arguments");
    }
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    Value::String(input.replace("\n", ""))
}
pub fn tostr(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("tostr expected 1 argument");
    }
    let arg = &args[0];
    let result = match arg {
        Value::Int(val) => val.to_string(),
        Value::Double(val) => val.to_string(),
        Value::String(val) => val.clone(),
        Value::Bool(val) => val.to_string(),
        Value::None() => String::from("None"),
        Value::Function(func) => {
            let params: Vec<String> = func
                .params
                .iter()
                .map(|param| format!("{}: {}", param.name, param.m_type.name))
                .collect();
            let stri = String::from(format!(
                "{}({}) -> {}",
                func.name,
                params.join(", "),
                func.return_type.name
            ));
            println!("{}", stri);
            stri
        }
        Value::Array(mutable, elements) => {
            let mutable_str = if *mutable { "mutable" } else { "immutable" };
            format!("array : {} , length : {}", mutable_str, elements.len())
        }
        _ => {
            dbg!(arg);
            dbg!(args);
            panic!("Cannot convert value to string");
        }
    };
    Value::String(result)
}
pub fn push(_context: &mut Context, type_checker: &TypeChecker, mut args: Vec<Value>) -> Value {
    if args.len() < 2 {
        panic!("push expected 2 arguments");
    }
    let arg = args.remove(0);

    match arg {
        Value::Array(mutable, mut elements) => {
            if mutable {
                for value in args {
                    if let Some(t) = type_checker.from_value(&value) {
                        let var = Variable::new(mutable, value, Rc::clone(&t));
                        elements.push(var);
                    } else {
                        panic!("invalid type for array");
                    }
                }

                return Value::Array(mutable, elements);
            } else {
                panic!("Cannot push to immutable array");
            }
        }
        _ => {
            panic!("Cannot push to value");
        }
    }
}
pub fn pop(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("pop expected 1 argument");
    }
    let arg = &args[0];
    match &arg {
        Value::Array(mutable, elements) => {
            if *mutable {
                let mut new_array = elements.clone();
                let _popped_value = new_array.pop().expect("array is empty");
                return Value::Array(*mutable, new_array);
            } else {
                panic!("Cannot pop from immutable array");
            }
        }
        _ => {
            panic!("Cannot pop from non-array value");
        }
    }
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
        (String::from("wait"), BuiltInFunction::new(Box::new(wait))),
        (String::from("tostr"), BuiltInFunction::new(Box::new(tostr))),
        (String::from("time"), BuiltInFunction::new(Box::new(time))),
        (
            String::from("assert"),
            BuiltInFunction::new(Box::new(assert)),
        ),
        (
            String::from("assert_eq"),
            BuiltInFunction::new(Box::new(assert_eq)),
        ),
        (String::from("len"), BuiltInFunction::new(Box::new(length))),
        (String::from("push"), BuiltInFunction::new(Box::new(push))),
        (String::from("pop"), BuiltInFunction::new(Box::new(pop))),
    ])
}
