use patpat::interpreter::VariableValue;
use patpat::test;

#[test]
fn call() {
  test::init_testenv();
  let src = test::load("test/tuples/access.patpat");
  assert_eq!(
      VariableValue::Tuple(vec![
        VariableValue::Tuple(vec![VariableValue::Number(2.0), VariableValue::Number(1.0)]),
        VariableValue::Number(1.0),
        VariableValue::Number(3.0)
      ]),
      test::execute(test::compile(&src))
  );
}
