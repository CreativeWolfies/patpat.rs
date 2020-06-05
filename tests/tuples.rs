use patpat::interpreter::VariableValue;
use patpat::test;

#[test]
fn access() {
  test::init_testenv();
  let src = test::load("test/tuples/access.patpat");
  assert_eq!(
      VariableValue::Tuple(vec![
          VariableValue::Tuple(vec![VariableValue::Number(2.0), VariableValue::Number(1.0)]),
          VariableValue::Number(1.0),
          VariableValue::Number(3.0),
          VariableValue::Number(1.0),
      ]),
      test::execute(test::compile(&src))
  );
}

#[test]
fn push() {
  test::init_testenv();
  let src = test::load("test/tuples/push.patpat");
  assert_eq!(
      VariableValue::Tuple(vec![
          VariableValue::Number(1.0),
          VariableValue::Number(2.0),
          VariableValue::Number(3.0),
      ]),
      test::execute(test::compile(&src))
  );
}

#[test]
fn pop() {
  test::init_testenv();
  let src = test::load("test/tuples/pop.patpat");
  assert_eq!(
      VariableValue::Tuple(vec![
          VariableValue::Number(3.0),
          VariableValue::Tuple(vec![VariableValue::Number(1.0), VariableValue::Number(2.0)]),
      ]),
      test::execute(test::compile(&src))
  );
}
