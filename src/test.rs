use crate::runtime::types::Value;
use std::{collections::HashMap, rc::Rc};

#[test]
fn if_else_statements() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/if_else.scorch"));
    dbg!(ctx);
}

#[test]
fn fields_vars_literals() {
    let ctx = super::execute_from_file(String::from(
        "scorch_src/unit_tests/fields_vars_literals.scorch",
    ));
    dbg!(ctx);
}

#[test]
fn loops() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/loops.scorch"));
    
    let value = ctx.borrow_mut().find_variable("result");
    if let super::runtime::types::Value::Bool(v) = value.to_owned().unwrap().value {
        assert_eq!(v, true, "test failed: {} {}", "loop one", v);
    }
    
    let value = ctx.borrow_mut().find_variable("result1").clone();
    if let super::runtime::types::Value::Bool(v) = value.to_owned().unwrap().value {
        assert_eq!(v, true, "test failed: {} {}", "loop two", v);
    }
    
    let value = ctx.borrow_mut().find_variable("result2").clone();
    if let super::runtime::types::Value::Bool(v) = value.to_owned().unwrap().value {
        assert_eq!(v, true, "test failed: {} {}", "loop two", v);
    }
}

#[test]
fn structs() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/structs.scorch"));
    let status = Rc::clone(&ctx.borrow_mut().variables["status"]);
    if let Value::String(str_status) = &status.value {
        assert!(str_status == "passed", "test failed: {}", str_status);
    }
}

#[test]
fn functions() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/functions.scorch"));
    let status = Rc::clone(&ctx.borrow_mut().variables["status"]);
    if let Value::String(str_status) = &status.value {
        assert!(str_status == "passed", "test failed: {}", str_status);
    }
}

#[test]
fn relationals() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/relationals.scorch"));
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
        let value = ctx.borrow_mut().variables[*&variable].clone();

        if let super::runtime::types::Value::Bool(v) = value.value {
            assert_eq!(v, expected_result, "test failed: {}", &variable);
        }
    }
}

#[test]
fn arithmetic() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/arithmetic.scorch"));
    let expected_results = HashMap::from([
        ("ff_addition", 5.3 + 6.2),
        ("ff_subtraction", 5.3 - 6.2),
        ("ff_multiplcation", 5.3 * 6.2),
        ("ff_division", 5.3 / 6.2),
        ("ff_parenthesis_1", (5.3 + 6.2) * 2.5),
        ("ff_parenthesis_2", 5.3 - (6.2 * 3.1)),
        ("ff_complex_1", (5.3 + 6.2) / (3.1 - 2.0)),
        ("ff_complex_2", 5.3 + (6.2 * 3.1) / 2.5),
        ("ff_complex_3", (5.3 - 6.2) * 2.5 / 3.1),
        ("ff_complex_4", 5.3 / (6.2 + 3.1) * 2.5),
        ("ff_complex_5", 5.3 + 6.2 - 3.1 * 2.0 / 1.5),
        ("ff_complex_6", ((5.3 * 2.5) - 6.2) / 3.1 + 1.0),
    ]);
    for (name, expected_val) in expected_results {
        let value = ctx.borrow_mut().variables[name].clone();
        if let super::runtime::types::Value::Double(v) = value.value {
            assert_eq!(v, expected_val, "test failed: {}", &name);
        }
    }
}
#[test]
fn arrays() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/arrays.scorch"));
    
    let _test_code_string = "empty_implicit 			:= []
    empty_explicit 			:= Array = []
    single_float_implicit 	:= [1.0]
    single_float_explicit 	:= Array = [1.0]
    plural_float_implicit  	:= [1.0, 2.0]
    plural_float_explicit 	:= Array = [1.0, 2.0]
    assignment 				:= single_float_implicit
    single_element_access 	:= single_float_implicit[0]
    first_element_access 	:= plural_float_implicit[0]
    second_element_access 	:= plural_float_implicit[1]
    accessor_assignment		:= [1.0, 2.0]
    accessor_assignment[0]	= 3.0";

    let t0 = ctx.borrow_mut().variables["empty_implicit"].clone();
    match &t0.value {
        Value::Array(_, elements) => assert_eq!(elements.len(), 0, "test failed: empty_implicit"),
        _ => panic!("test failed: empty_implicit"),
    }
    
    let t1 = ctx.borrow_mut().variables["empty_explicit"].clone();
    match &t1.value {
        Value::Array(_, elements) => assert_eq!(elements.len(), 0, "test failed: empty_explicit"),
        _ => panic!("test failed: empty_explicit"),
    }
    
    let t2 = ctx.borrow_mut().variables["single_float_implicit"].clone();
    match &t2.value {
        Value::Array(_, elements) => {
            assert_eq!(elements.len(), 1, "test failed: single_float_implicit");
            if let Value::Double(inner_val) = &elements[0].value {
                assert_eq!(
                    *inner_val, 1.0,
                    "test failed: single_float_implicit inner value"
                );
            } else {
                panic!("test failed: single_float_implicit inner value");
            }
        }
        _ => panic!("test failed: single_float_implicit"),
    }
    
    let t3 = ctx.borrow_mut().variables["single_float_explicit"].clone();
    match &t3.value {
        Value::Array(_, elements) => {
            assert_eq!(elements.len(), 1, "test failed: single_float_explicit");
            if let Value::Double(inner_val) = &elements[0].value {
                assert_eq!(
                    *inner_val, 1.0,
                    "test failed: single_float_explicit inner value"
                );
            } else {
                panic!("test failed: single_float_explicit inner value");
            }
        }
        _ => panic!("test failed: single_float_explicit"),
    }

    let t4 = ctx.borrow_mut().variables["plural_float_implicit"].clone();
    match &t4.value {
        Value::Array(_, elements) => {
            assert_eq!(elements.len(), 2, "test failed: plural_float_implicit");
            if let Value::Double(inner_val) = &elements[0].value {
                assert_eq!(
                    *inner_val, 1.0,
                    "test failed: plural_float_implicit inner value"
                );
            } else {
                panic!("test failed: plural_float_implicit inner value");
            }
            if let Value::Double(inner_val) = &elements[1].value {
                assert_eq!(
                    *inner_val, 2.0,
                    "test failed: plural_float_implicit inner value"
                );
            } else {
                panic!("test failed: plural_float_implicit inner value");
            }
        }
        _ => panic!("test failed: plural_float_implicit"),
    }

    let t5 = ctx.borrow_mut().variables["plural_float_explicit"].clone();
    match &t5.value {
        Value::Array(_, elements) => {
            assert_eq!(elements.len(), 2, "test failed: plural_float_explicit");
            if let Value::Double(inner_val) = &elements[0].value {
                assert_eq!(
                    *inner_val, 1.0,
                    "test failed: plural_float_explicit inner value"
                );
            } else {
                panic!("test failed: plural_float_explicit inner value");
            }
            if let Value::Double(inner_val) = &elements[1].value {
                assert_eq!(
                    *inner_val, 2.0,
                    "test failed: plural_float_explicit inner value"
                );
            } else {
                panic!("test failed: plural_float_explicit inner value");
            }
        }
        _ => panic!("test failed: plural_float_explicit"),
    }

    let t6 = ctx.borrow_mut().variables["assignment"].clone();
    match &t6.value {
        Value::Array(_, elements) => {
            assert_eq!(elements.len(), 1, "test failed: assignment");
            if let Value::Double(inner_val) = &elements[0].value {
                assert_eq!(*inner_val, 1.0, "test failed: assignment inner value");
            } else {
                panic!("test failed: assignment inner value");
            }
        }
        _ => panic!("test failed: assignment"),
    }

    let t7 = ctx.borrow_mut().variables["accessor_assignment"].clone();
    match &t7.value {
        Value::Array(_, elements) => {
            assert_eq!(elements.len(), 2, "test failed: accessor_assignment");
            if let Value::Double(inner_val) = &elements[0].value {
                assert_eq!(
                    *inner_val, 3.0,
                    "test failed: accessor_assignment inner value"
                );
            } else {
                panic!("test failed: accessor_assignment inner value");
            }
            if let Value::Double(inner_val) = &elements[1].value {
                assert_eq!(
                    *inner_val, 2.0,
                    "test failed: accessor_assignment inner value"
                );
            } else {
                panic!("test failed: accessor_assignment inner value");
            }
        }
        _ => panic!("test failed: accessor_assignment"),
    }
}
