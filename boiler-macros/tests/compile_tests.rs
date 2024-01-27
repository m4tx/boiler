#[test]
fn test_action_meta() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/action_meta.rs");
    t.compile_fail("tests/ui/action_meta_fail.rs");
}
