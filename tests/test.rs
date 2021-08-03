#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_tests/forgot_to_import.rs");
    t.compile_fail("tests/compile_tests/forgot_to_add_derive.rs");
    t.pass("tests/compile_tests/casual_test.rs");
    t.pass("tests/compile_tests/empty_struct_to_empty_struct.rs");
    t.pass("tests/compile_tests/input_struct_with_exceeding_fields.rs");
    t.compile_fail("tests/compile_tests/output_struct_with_exceeding_fields.rs");
    t.pass("tests/compile_tests/structs_with_plain_fields.rs");
}
