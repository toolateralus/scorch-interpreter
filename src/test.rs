use crate::cli::execute_from_file;
#[test]
fn if_else_statements() {
    execute_from_file(String::from("scorch_src/unit_tests/if_else.scorch"));
}
#[test]
fn fields_vars_literals() {
    execute_from_file(String::from(
        "scorch_src/unit_tests/fields_vars_literals.scorch",
    ));
}
#[test]
fn loops() {
    execute_from_file(String::from("scorch_src/unit_tests/loops.scorch"));
}
#[test]
fn structs() {
    execute_from_file(String::from("scorch_src/unit_tests/structs.scorch"));
}
#[test]
fn functions() {
    execute_from_file(String::from("scorch_src/unit_tests/functions.scorch"));
}
#[test]
fn relationals() {
    execute_from_file(String::from("scorch_src/unit_tests/relationals.scorch"));
}
#[test]
fn arithmetic() {
    execute_from_file(String::from("scorch_src/unit_tests/arithmetic.scorch"));
}
#[test]
fn arrays() {
    execute_from_file(String::from("scorch_src/unit_tests/arrays.scorch"));
}
