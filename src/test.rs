#[test]
fn if_else_statements() {
    super::execute_from_file(String::from("scorch_src/unit_tests/if_else.scorch"));
}

#[test]
fn fields_vars_literals() {
    super::execute_from_file(String::from(
        "scorch_src/unit_tests/fields_vars_literals.scorch",
    ));
}

#[test]
fn loops() {
    super::execute_from_file(String::from("scorch_src/unit_tests/loops.scorch"));
}

#[test]
fn structs() {
    super::execute_from_file(String::from("scorch_src/unit_tests/structs.scorch"));
}

#[test]
fn functions() {
    super::execute_from_file(String::from("scorch_src/unit_tests/functions.scorch"));
}

#[test]
fn relationals() {
    super::execute_from_file(String::from("scorch_src/unit_tests/relationals.scorch"));
}

#[test]
fn arithmetic() {
    super::execute_from_file(String::from("scorch_src/unit_tests/arithmetic.scorch"));
}
#[test]
fn arrays() {
    super::execute_from_file(String::from("scorch_src/unit_tests/arrays.scorch"));
}
