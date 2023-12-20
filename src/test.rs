// todo!()
pub fn if_else_statements() {
    let ctx = super::execute_return_global_ctx(String::from("test_if_else.scorch"));
    dbg!(ctx);
}
// todo!()
pub fn fields_vars_literal() {
    let ctx = super::execute_return_global_ctx(String::from("test_fields_vars_literal.scorch"));
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
    let ctx = super::execute_return_global_ctx(String::from("test_functions.scorch"));
    let status = ctx.variables["status"].clone();
    if let super::types::ValueType::String(str_status) = *status {
        assert!(str_status == "success", "test failed: {}", str_status);
    }
}

#[test]
fn test_rel_expr() {
    let ctx = super::execute_return_global_ctx(String::from("test_rel_expr.scorch"));
    let variables = [
        "rel_t1", "rel_t2", "rel_t3", "rel_t4", "rel_t5", "rel_t6", "rel_t7", "rel_t8",
        "rel_t9", "rel_t10", "rel_t11", "rel_t12",
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
       
        if let super::types::ValueType::Bool(v) = value {
            assert_eq!(v, expected_result, "test failed: {}", variable);
        }
    }
}