#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_tests/forgot_to_import.rs");
    t.compile_fail("tests/compile_tests/forgot_to_add_derive.rs");
    t.compile_fail("tests/compile_tests/output_struct_with_exceeding_fields.rs");
    t.pass("tests/compile_tests/casual_test.rs");
    t.pass("tests/compile_tests/empty_struct_to_empty_struct.rs");
    t.pass("tests/compile_tests/from_with_tuple.rs");
    t.pass("tests/compile_tests/from_ignores_duplicates.rs");
    t.pass("tests/compile_tests/from_occurring_more_than_once.rs");
    t.pass("tests/compile_tests/input_struct_with_exceeding_fields.rs");
    t.pass("tests/compile_tests/structs_with_into_fields.rs");
    t.pass("tests/compile_tests/structs_with_into_option_fields.rs");
    t.pass("tests/compile_tests/structs_with_into_vec_fields.rs");
    t.pass("tests/compile_tests/structs_with_plain_fields.rs");
    t.pass("tests/compile_tests/structs_with_macrotransform_plain_fields.rs");
    t.pass("tests/compile_tests/structs_with_multiple_annotations_on_plain_fields.rs");
    t.pass("tests/compile_tests/structs_with_transform_plain_fields.rs");
}
