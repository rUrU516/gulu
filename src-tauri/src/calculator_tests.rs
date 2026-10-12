use crate::{calculator::Calculator, expression::evaluate};

fn type_keys(calculator: &mut Calculator, keys: &str) {
    for key in keys.chars() {
        calculator.press_key(&key.to_string()).unwrap();
    }
}

fn calculate(keys: &str) -> String {
    let mut calculator = Calculator::default();
    type_keys(&mut calculator, keys);
    let view = calculator.press_key("equals").unwrap();
    assert!(view.error.is_none(), "{:?}", view.error);
    view.display
}

#[test]
fn multiplication_and_division_take_priority() {
    assert_eq!(calculate("2+3*4"), "14");
    assert_eq!(calculate("18-12/3"), "14");
}

#[test]
fn parentheses_override_priority() {
    assert_eq!(calculate("2*(3+4)"), "14");
    assert_eq!(calculate("((2+3)*4)/(6-2)"), "5");
}

#[test]
fn subtraction_and_division_are_left_associative() {
    assert_eq!(calculate("10-3-2"), "5");
    assert_eq!(calculate("20/2/5"), "2");
}

#[test]
fn negative_numbers_and_decimals_work() {
    assert_eq!(calculate("-2.5*4"), "-10");
    assert_eq!(calculate("3*-2"), "-6");
    assert_eq!(calculate("-(2+3)"), "-5");
    assert_eq!(calculate(".5+.25"), "0.75");
    assert_eq!(calculate("-0"), "0");
}

#[test]
fn adjacent_numbers_and_parentheses_insert_multiplication() {
    assert_eq!(calculate("2(3+4)"), "14");
    assert_eq!(calculate("(2+3)4"), "20");
    assert_eq!(calculate("(2+3)(4+1)"), "25");
    assert_eq!(calculate("(2+3).5"), "2.5");
}

#[test]
fn division_by_zero_preserves_the_expression_for_editing() {
    let mut calculator = Calculator::default();
    type_keys(&mut calculator, "1/(2-2)");
    let view = calculator.press_key("equals").unwrap();
    assert_eq!(view.error.as_deref(), Some("不能除以 0"));
    assert_eq!(view.display, "错误");
    assert!(!view.expression.is_empty());
    calculator.press_key("clear").unwrap();
    type_keys(&mut calculator, "0/3");
    assert_eq!(calculator.press_key("equals").unwrap().display, "0");
}

#[test]
fn malformed_expressions_return_errors() {
    for expression in ["", "1+", "()", "(1+2", "1+2)", "1..2", "2**3", "abc"] {
        assert!(evaluate(expression).is_err(), "{expression}");
    }
}

#[test]
fn overflow_returns_an_error() {
    let large = "9".repeat(309);
    assert!(evaluate(&large).is_err());
}

#[test]
fn unsupported_keys_cannot_change_the_expression() {
    let mut calculator = Calculator::default();
    type_keys(&mut calculator, "12");
    for key in ["abc", "1+2", " ", "💧", "%", ""] {
        assert!(calculator.press_key(key).is_err());
    }
    assert_eq!(calculator.press_key("snapshot").unwrap().display, "12");
}

#[test]
fn backspace_and_clear_update_the_rust_state() {
    let mut calculator = Calculator::default();
    type_keys(&mut calculator, "12+3");
    assert_eq!(calculator.press_key("backspace").unwrap().display, "12 + ");
    type_keys(&mut calculator, "4");
    assert_eq!(calculator.press_key("equals").unwrap().display, "16");
    let view = calculator.press_key("clear").unwrap();
    assert_eq!(view.display, "0");
    assert!(view.expression.is_empty());
    assert!(view.error.is_none());
}

#[test]
fn an_operator_continues_from_the_previous_result() {
    let mut calculator = Calculator::default();
    type_keys(&mut calculator, "2+3");
    assert_eq!(calculator.press_key("equals").unwrap().display, "5");
    type_keys(&mut calculator, "*4");
    assert_eq!(calculator.press_key("equals").unwrap().display, "20");
}

#[test]
fn a_digit_starts_a_new_calculation_after_equals() {
    let mut calculator = Calculator::default();
    type_keys(&mut calculator, "2+3");
    calculator.press_key("equals").unwrap();
    type_keys(&mut calculator, "7");
    assert_eq!(calculator.press_key("equals").unwrap().display, "7");
}

#[test]
fn snapshot_and_equals_do_not_reset_the_result() {
    let mut calculator = Calculator::default();
    type_keys(&mut calculator, "2+3");
    calculator.press_key("equals").unwrap();
    assert_eq!(calculator.press_key("snapshot").unwrap().display, "5");
    assert_eq!(calculator.press_key("equals").unwrap().display, "5");
}
