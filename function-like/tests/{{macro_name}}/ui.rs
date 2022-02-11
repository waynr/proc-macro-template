use trybuild;

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/{{macro_name}}/fail/*.rs");
    t.pass("tests/{{macro_name}}/pass/*.rs");
}
