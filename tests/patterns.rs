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

#[test]
fn test_log() {
    test::init_testenv();
    let src = test::load("test/patterns/test_log.patpat");
    test::execute(test::compile(&src));
    assert_eq!(
        concat!(
            "Number(1.0)\n",
            "String(\"Hello, world\")\n",
            "Nil\n",
            "Boolean(true)\n",
            "Tuple([Boolean(false), Number(4.2)])\n",
            "Boolean(false), Number(4.2)\n",
        ),
        test::get_logs()
    );
}

#[test]
fn lhs() {
    test::init_testenv();
    let src = test::load("test/patterns/lhs.patpat");
    assert_eq!(
        VariableValue::Number(4.0),
        test::execute(test::compile(&src))
    );
}

#[test]
#[should_panic(expected = "Mismatching number of arguments: expected 2, got 1.")]
fn nargs_panic() {
    test::init_testenv();
    let src = test::load("test/patterns/nargs_panic.patpat");
    test::execute(test::compile(&src));
}