use super::context::Context;
use super::typechecker::TypeChecker;
use super::types::{Instance, Value};
use std::cell::RefCell;
use std::process::Command;
use std::{collections::HashMap, rc::Rc};

pub struct StandardFunction {
    pub func: Box<dyn FnMut(&mut Context, &TypeChecker, Vec<Value>) -> Value>,
}
impl StandardFunction {
    pub fn new(func: Box<dyn FnMut(&mut Context, &TypeChecker, Vec<Value>) -> Value>) -> Self {
        StandardFunction { func }
    }
    pub fn call(
        &mut self,
        context: &mut Context,
        type_checker: &TypeChecker,
        args: Vec<Value>,
    ) -> Value {
        (self.func)(context, type_checker, args)
    }
}

pub fn get_builtin_functions() -> HashMap<String, StandardFunction> {
    HashMap::from([
        (
            String::from("clearscreen"),
            StandardFunction::new(Box::new(clear_screen)),
        ),
        (
            String::from("println"),
            StandardFunction::new(Box::new(print_ln)),
        ),
        (
            String::from("readln"),
            StandardFunction::new(Box::new(readln)),
        ),
        (String::from("wait"), StandardFunction::new(Box::new(wait))),
        (
            String::from("tostr"),
            StandardFunction::new(Box::new(tostr)),
        ),
        (String::from("time"), StandardFunction::new(Box::new(time))),
        (
            String::from("assert"),
            StandardFunction::new(Box::new(assert)),
        ),
        (
            String::from("assert_eq"),
            StandardFunction::new(Box::new(assert_eq)),
        ),
        (String::from("find"), StandardFunction::new(Box::new(find))),
        (String::from("len"), StandardFunction::new(Box::new(length))),
        (String::from("push"), StandardFunction::new(Box::new(push))),
        (String::from("pop"), StandardFunction::new(Box::new(pop))),
        (
            String::from("floor"),
            StandardFunction::new(Box::new(floor)),
        ),
        (String::from("abs"), StandardFunction::new(Box::new(abs))),
    ])
}

pub fn clear_screen(_: &mut Context, _: &TypeChecker, _: Vec<Value>) -> Value {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("clear").status();
    }
    Value::None()
}

// IO
pub fn print_ln(context: &mut Context, type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    for arg in args {
        match arg {
            Value::Reference(..) => todo!(),
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

                println!(
                    "{} Array<T> : length {}",
                    mutable_str,
                    elements.borrow_mut().len()
                );

                // for element in elements.iter() {
                //     print_ln(Vec::from([element.value.clone()]));
                // }
            }
            Value::StructInstance {
                typename: _name,
                context: _,
            } => {
                // println!("global::{}\n{:#?}", name, context);
                // for (k, member) in context.variables.iter() {
                //     println!("{} : {:?}", k, member.value)
                // }
            }
            Value::Return(_) => panic!("Cannot print return value"),
        }
    }
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
// System
pub fn time(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 0 {
        panic!("time expected 0 arguments");
    }

    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    Value::Int(time.as_millis() as i32)
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
// Vectors & Arrays
pub fn length(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("length takes one array argument");
    }
    let arg = &args[0];

    match arg {
        Value::Array(_, elements) => {
            return Value::Int(elements.borrow_mut().len() as i32);
        }
        _ => {
            dbg!(arg);
            panic!("Cannot get length of value");
        }
    }
}
pub fn push(_context: &mut Context, type_checker: &TypeChecker, mut args: Vec<Value>) -> Value {
    if args.len() < 2 {
        panic!("push expected 2 arguments");
    }
    let arg = args.remove(0);

    match arg {
        Value::Array(mutable, elements) => {
            if mutable {
                for value in args {
                    if let Some(t) = type_checker.from_value(&value) {
                        let var = Rc::new(RefCell::new(Instance::new(mutable, value, Rc::clone(&t))));
                        elements.borrow_mut().push(var);
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
pub fn find(_context: &mut Context, type_checker: &TypeChecker, mut args: Vec<Value>) -> Value {
    if args.len() < 2 {
        panic!("find expects at least 2 arguments: a string key and an array or struct instance to search in. got : {:#?}", args);
    }
    let key = args.remove(0);
    let search_target = args.remove(0);
    
    match search_target {
        Value::Array(_, elements) => {
            let key = key.as_string().unwrap();
            
            for element in elements.borrow().iter() {
                let element = element.borrow_mut();
                let Value::StructInstance {typename, context } = &element.value else {
                    panic!("find expects an array or struct instance as the search target, got : {:#?}", &element.value);
                };
                
                if let Some(member) = context.variables.get("key") {
                    let mem = member.borrow();
                    let string = mem.value.as_string();
                    let mut equals = false;
                    
                    if let Some(string) = string {
                        equals = *string == *key;
                    };
                    
                    if !equals {
                        continue;
                    }
                    
                    return Value::Reference(Rc::clone(&member));
                }
            }
        }
        _ => {
            panic!("find expects an array or struct instance as the search target, got {:#?}", search_target);
        }
    }
    
    Value::None()
}
pub fn pop(_context: &mut Context, _type_checker: &TypeChecker, mut args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("pop expected 1 argument");
    }
    let arg = args.remove(0);
    match arg {
        Value::Array(mutable, elements) => {
            let mut el = elements.borrow_mut();
            assert!(
                el.len() > (0 as usize),
                "stack underflow: cannot pop from an empty array."
            );
            assert!(mutable, "Cannot pop from immutable array");
            
            let val = el.pop();
            
            if let Some(val) = val {
                return Value::Reference(Rc::clone(&val))
            } else {
                panic!("Cannot pop from empty array");
            }
            
        }
        _ => {
            panic!("Cannot pop from non-array value");
        }
    }
}
// Testing
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
// Conversions
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
        Value::None() => String::from(scorch_parser::ast::NONE_TNAME),
        Value::Function(func) => {
            let stri = get_function_signature(func);
            stri
        }
        Value::Array(mutable, elements) => {
            let mutable_str = if *mutable { "mutable" } else { "immutable" };
            format!(
                "array : {} , length : {}",
                mutable_str,
                elements.borrow_mut().len()
            )
        }
        _ => {
            dbg!(arg);
            dbg!(args);
            panic!("Cannot convert value to string");
        }
    };
    Value::String(result)
}
pub fn get_function_signature<'ctx>(func: &'ctx Rc<super::types::Function>) -> String {
    let params: Vec<String> = func
        .params
        .iter()
        .map(|param| format!("{}: {}", param.name, param.m_type.borrow().name))
        .collect();
    format!(
        "{}({}) -> {}",
        func.name,
        params.join(", "),
        func.return_type.borrow().name
    )
}
// Math
// IO
pub fn abs(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("abs expected 1 argument");
    }
    let arg = &args[0];
    match arg {
        Value::Int(val) => Value::Int(val.abs()),
        Value::Double(val) => Value::Double(val.abs()),
        _ => panic!("Cannot apply abs function to non-numeric value"),
    }
}
pub fn floor(_context: &mut Context, _type_checker: &TypeChecker, args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("floor expected 1 argument");
    }
    let arg = &args[0];
    match arg {
        Value::Double(val) => Value::Double(val.floor()),
        _ => panic!("Cannot apply floor function to non-double value"),
    }
}
