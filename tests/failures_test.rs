use hangman::failures::AllowedFailures;

#[test]
fn errors_with_too_little_failures() {
    let result = AllowedFailures::limit(0);

    assert!(result.is_err());
}

#[test]
fn remaining_failures_are_consumed() {
    let limit = 3;
    let mut failures = AllowedFailures::limit(limit).unwrap();

    failures.consume();

    assert_eq!(failures.remaining(), limit - 1);
}

#[test]
fn some_failures_left() {
    let mut failures = AllowedFailures::limit(2).unwrap();

    failures.consume();

    assert!(!failures.none_left());
}

#[test]
fn no_failures_left() {
    let mut failures = AllowedFailures::limit(1).unwrap();

    failures.consume();

    assert!(failures.none_left());
}
