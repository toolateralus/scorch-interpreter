use std::collections::HashMap;
use crate::runtime::types::Value;

// todo!()
pub fn if_else_statements() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/test_if_else.scorch"));
    dbg!(ctx);
}
// todo!()
pub fn fields_vars_literal() {
    let ctx = super::execute_from_file(String::from(
        "scorch_src/unit_tests/test_fields_vars_literal.scorch",
    ));
    dbg!(ctx);
}

#[test]
fn test_if_else_statements() {
    if_else_statements();
}

#[test]
fn test_fields_vars_literal() {
    fields_vars_literal();
}

#[test]
fn test_functions() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/test_functions.scorch"));
    let status = ctx.variables["status"].clone();
    if let Value::String(str_status) = status.value.clone() {
        assert!(str_status == "passed", "test failed: {}", str_status);
    }
}

#[test]
fn test_rel_expr() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/test_rel_expr.scorch"));
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
        let value = &*ctx.variables[*&variable].clone();
        
        if let super::runtime::types::Value::Bool(v) = value.value {
            assert_eq!(v, expected_result, "test failed: {}", &variable);
        }
    }
}

#[test]
fn test_arithmetic() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/test_arithmetic.scorch"));
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
        let value = ctx.variables[name].clone();
        if let super::runtime::types::Value::Float(v) = value.value {
            assert_eq!(v, expected_val, "test failed: {}", &name);
        }
    }
}
//#[test]
// TODO: implemet this
fn test_arrays() {
    let ctx = super::execute_from_file(String::from("scorch_src/unit_tests/arrays.scorch"));
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
        let value = ctx.variables[name].clone();
        if let super::runtime::types::Value::Float(v) = value.value {
            assert_eq!(v, expected_val, "test failed: {}", &name);
        }
    }
}
