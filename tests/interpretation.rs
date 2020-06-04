use patpat::interpreter::VariableValue;
use patpat::test;

#[test]
fn basic() {
    test::init_testenv();
    let src = test::load("test/interpretation/basic.patpat");
    assert_eq!(
        VariableValue::Tuple(vec![
            VariableValue::Number(2.0),
            VariableValue::Number(4.0)
        ]),
        test::execute(test::compile(&src))
    );
}
