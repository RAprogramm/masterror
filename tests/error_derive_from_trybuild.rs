use trybuild::TestCases;

#[test]
fn from_attribute_compile_failures() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/from/*.rs");
}

#[test]
fn transparent_attribute_compile_failures() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/transparent/*.rs");
}

#[test]
fn backtrace_attribute_compile_failures() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/backtrace/*.rs");
}
