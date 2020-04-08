use patpat::interpreter::VariableValue;
use patpat::test;

#[test]
fn hello_world() {
    test::init_testenv();
    let src = test::load("test/basic/hello_world.patpat");
    assert_eq!(
        VariableValue::String("Hello, world!".to_string()),
        test::execute(test::compile(&src))
    );
}

#[test]
fn variables() {
    test::init_testenv();
    let src = test::load("test/basic/variables.patpat");
    assert_eq!(
        VariableValue::Tuple(vec![VariableValue::Number(4.0), VariableValue::Number(2.0)]),
        test::execute(test::compile(&src))
    );
}

#[test]
fn blocks() {
    test::init_testenv();
    let src = test::load("test/basic/blocks.patpat");
    assert_eq!(
        VariableValue::Number(3.0),
        test::execute(test::compile(&src))
    );
}
