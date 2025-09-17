use trybuild::TestCases;

#[test]
fn from_attribute_compile_failures() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/from/*.rs");
}
