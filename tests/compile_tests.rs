#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_tests/*.fail.rs");
    t.pass("tests/compile_tests/*.pass.rs");
}
