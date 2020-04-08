use patpat::interpreter::VariableValue;
use patpat::test;

#[test]
fn add() {
    test::init_testenv();
    let src = test::load("test/expr/add.patpat");
    assert_eq!(
        VariableValue::Number(17.0),
        test::execute(test::compile(&src))
    );
}

#[test]
fn sub() {
    test::init_testenv();
    let src = test::load("test/expr/sub.patpat");
    assert_eq!(
        VariableValue::Number(-19.0),
        test::execute(test::compile(&src))
    );
}

#[test]
fn mul() {
    test::init_testenv();
    let src = test::load("test/expr/mul.patpat");
    assert_eq!(
        VariableValue::Number(-18.0),
        test::execute(test::compile(&src))
    );
}

#[test]
fn div() {
    test::init_testenv();
    let src = test::load("test/expr/div.patpat");
    assert_eq!(
        VariableValue::Number(-1.0 / 18.0),
        test::execute(test::compile(&src))
    );
}
