use hangman::failures::AllowedFailures;

#[test]
fn does_not_build_with_too_little_failures() {
    let result = AllowedFailures::from(0);

    assert!(result.is_err());
}

#[test]
fn failures_are_consumed() {
    let limit = 3;
    let mut failures = AllowedFailures::from(limit).unwrap();

    failures.consume();

    assert_eq!(failures.remaining(), limit - 1);
}

#[test]
fn some_failures_left() {
    let mut failures = AllowedFailures::from(2).unwrap();

    failures.consume();

    assert!(failures.any_left());
}

#[test]
fn no_failures_left() {
    let mut failures = AllowedFailures::from(1).unwrap();

    failures.consume();

    assert!(!failures.any_left());
}
