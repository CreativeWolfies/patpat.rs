use patpat::interpreter::VariableValue;
use patpat::test;

#[test]
fn call() {
    test::init_testenv();
    let src = test::load("test/patterns/call.patpat");
    assert_eq!(
        VariableValue::Number(4.0),
        test::execute(test::compile(&src))
    );
}
